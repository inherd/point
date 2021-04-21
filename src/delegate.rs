use crate::app_state::AppState;
use crate::command::print_command;
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};
use std::path::PathBuf;

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
        if let Some(info) = cmd.get(print_command::OPEN_FILE) {
            let path = PathBuf::from(info.path.as_str());
            log::info!("open file: {:?}", path.display());
            data.set_file(path);
            return Handled::Yes;
        } else if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
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

        Handled::No
    }
}
