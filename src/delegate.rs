use crate::command::command;
use crate::AppState;
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
            return match infer::get_from_path(info.path()) {
                Ok(typ) => match typ {
                    None => {
                        log::info!("unknown type: {:?}", info);
                        Handled::No
                    }
                    Some(file_type) => {
                        log::info!("file type: {:?}", file_type);
                        data.workspace.set_file(info.path().to_owned());
                        ctx.submit_command(command::REBUILD_MENUS);
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
