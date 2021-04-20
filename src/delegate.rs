use crate::AppState;
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

#[derive(Debug, Default)]
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command<'a>(
        &mut self,
        ctx: &mut DelegateCtx<'a>,
        target: Target,
        cmd: &Command,
        data: &mut AppState,
        env: &Env,
    ) -> Handled {
        if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            println!("{:?}", info);
            return Handled::Yes;
        }

        Handled::No
    }
}
