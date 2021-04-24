use crate::app_state::{AppState, Workspace};
use crate::command::print_command;
use crate::components::modal_host::ModalHost;
use druid::widget::{Flex, Label};
use druid::{AppDelegate, Command, DelegateCtx, Env, FileInfo, Handled, Target, Widget, WidgetExt};
use regex::Regex;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command<'a>(
        &mut self,
        ctx: &mut DelegateCtx<'a>,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(info) = cmd.get(print_command::SET_FILE) {
            let path = PathBuf::from(info.path.as_str());
            log::info!("open file: {:?}", path.display());
            data.set_file(path);
            return Handled::Yes;
        } else if cmd.is(druid::commands::SAVE_FILE) {
            return Delegate::save_file(data);
        } else if cmd.is(druid::commands::SHOW_ABOUT) {
            let host = ModalHost::new(Delegate::paint_preferences());
            host.lens(AppState::workspace);
            // .controller(RootWindowController::default());
            return Handled::Yes;
        } else if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            return Delegate::open_file(ctx, data, info);
        }

        Handled::No
    }
}

impl Delegate {
    fn open_file(ctx: &mut DelegateCtx, data: &mut AppState, info: &FileInfo) -> Handled {
        if info.path().is_dir() {
            data.set_dir(info.path().to_owned());
            ctx.submit_command(print_command::OPEN);
            return Handled::Yes;
        }

        if let Ok(typ) = infer::get_from_path(info.path()) {
            if let Some(_file_type) = typ {
                if let Some(parent) = info.path().parent() {
                    data.set_dir(Some(parent.to_owned()));
                }

                data.set_file(info.path().to_owned());
                ctx.submit_command(print_command::OPEN);
                return Handled::Yes;
            }
        };

        log::info!("under type: {:?}", info);
        return Handled::No;
    }

    fn save_file(data: &mut AppState) -> Handled {
        let file_path = data.current_file.as_ref().unwrap();
        let buf = file_path.to_path_buf();

        if data.workspace.input_text == data.workspace.origin_text {
            return Handled::Yes;
        }

        data.workspace.char_count = count_text(data.workspace.input_text.clone());

        let mut ifile = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&buf)
            .expect("unable to open file");

        let result = ifile.write_all(data.text().as_bytes());

        match result {
            Ok(_) => log::info!("save file: {:?}", buf),
            Err(e) => log::info!("Failed to write data: {}", { e }),
        }

        return Handled::Yes;
    }

    fn paint_preferences() -> impl Widget<Workspace> {
        let flex = Flex::column()
            .with_child(Label::new("preferences").with_text_color(crate::theme::BASIC_TEXT_COLOR))
            .with_default_spacer();

        return flex;
    }
}

pub fn count_text(content: String) -> usize {
    let mut word_count = 0;
    let mut line_count = 0;
    let mut blank_lines_count = 0;

    // regex to remove unnecessary whitespace inside markdown file
    // see VS Code documentation: https://vscode-docs.readthedocs.io/en/stable/extensions/example-word-count/
    let whitespace_re = Regex::new(
        r"(?x)
      (< ([^>]+)<)
      -
      ^\s\s*
      -
      \s\s*$
      ",
    )
    .unwrap();
    // match multiple spaces and change to single space
    let multiple_spaces_re = Regex::new(r"\s+").unwrap();
    // match links and files in grammar "[](...)"
    let link_re = Regex::new(r"\]\((.*?)\)").unwrap();

    // process document
    for line in content.lines() {
        let clean_line = String::from(line.trim());

        if !clean_line.is_empty() {
            // remove whitespace
            let clean_line = replace_whitespace(&clean_line, "", &whitespace_re);
            let clean_line = multiple_spaces_re.replace_all(&clean_line, " ");
            let clean_line = link_re.replace_all(&clean_line, "]");

            // split words using unicode standards
            let words: Vec<&str> = clean_line.unicode_words().collect();
            word_count = word_count + words.len();
        } else {
            blank_lines_count = blank_lines_count + 1;
        }
    }

    word_count
}

// replace whitespace according to regex pattern
fn replace_whitespace(input: &str, placeholder: &str, re: &Regex) -> String {
    re.replace_all(input, placeholder).into()
}
