use crate::app_state::AppState;
use crate::command::print_command;
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

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
        if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            if info.path().is_dir() {
                data.set_dir(info.path().to_owned());
                ctx.submit_command(print_command::OPEN);
                return Handled::Yes;
            }
            return match infer::get_from_path(info.path()) {
                Ok(typ) => match typ {
                    None => {
                        log::info!("unknown type: {:?}", info);
                        Handled::No
                    }
                    Some(file_type) => {
                        log::info!("file type: {:?}", file_type);

                        match info.path().parent() {
                            None => {}
                            Some(parent) => {
                                data.set_dir(Some(parent.to_owned()));
                            }
                        }

                        data.set_file(info.path().to_owned());
                        ctx.submit_command(print_command::OPEN);
                        Handled::Yes
                    }
                },
                Err(err) => {
                    log::info!("error file type: {:?}", err);
                    Handled::No
                }
            };
        }

        Handled::No
    }
}
