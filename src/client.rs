use crate::structs::{
    Alert, AvailableLanguages, AvailablePlugins, AvailableThemes, ConfigChanged, FindStatus,
    LanguageChanged, MeasureWidth, PluginStarted, PluginStopped, ReplaceStatus, ScrollTo, Style,
    ThemeChanged, Update, UpdateCmds,
};
use serde_json::{self, from_value, json, Value};
use std::io::{self, BufRead, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use xi_core_lib::XiCore;
use xi_rpc::RpcLoop;

/// Wraps an instance of `mpsc::Sender`, implementing `Write`.
///
/// This lets the tx side of an mpsc::channel serve as the destination
/// stream for an RPC loop.
pub struct Writer(Sender<String>);

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8(buf.to_vec()).unwrap();
        self.0
            .send(s)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))
            .map(|_| buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Wraps an instance of `mpsc::Receiver`, providing convenience methods
/// for parsing received messages.
pub struct Reader(Receiver<String>);

impl Read for Reader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        unreachable!("didn't expect xi-rpc to call read");
    }
}

// Note: we don't properly implement BufRead, only the stylized call patterns
// used by xi-rpc.
impl BufRead for Reader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        unreachable!("didn't expect xi-rpc to call fill_buf");
    }

    fn consume(&mut self, _amt: usize) {
        unreachable!("didn't expect xi-rpc to call consume");
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let event = match self.0.recv() {
            Ok(s) => s,
            Err(_) => return Ok(0),
        };

        if &event == r#"{"method":"command","params":{"method":"exit"}}"# {
            // It receive a close commmand from the writer indicating the chan
            // should be closes. The event is sent by the InputController when
            // the user ask to quit the program.
            //
            // This method is required because the chan producers a shared between
            // The InputController and the EventController threads and it's
            // impossible for the InputController to close the EventController
            // channel.
            //
            // When the Reader receives the command, it close the channel between
            // the InputController which lead to the following steps in order:
            // - The channel between the the InputController and the Core close itself.
            // - The Core event loop stop itself safely.
            // - The channel between the Core and the EventController close itself.
            // - The the EventController event loop stop itself safely.
            // - The main exit.
            return Ok(0);
        }

        buf.push_str(&event);
        Ok(event.len())
    }
}

pub struct ClientToClientWriter(Writer);

impl ClientToClientWriter {
    pub fn send_rpc_notification(&mut self, method: &str, params: &Value) {
        let raw_content = match serde_json::to_vec(&json!({"method": method, "params": params})) {
            Ok(raw) => raw,
            Err(err) => {
                error!("failed to create the notification {}: {}", method, err);
                return;
            }
        };

        match self.0.write(&raw_content) {
            Ok(_) => (),
            Err(err) => error!("failed to send the notification {}: {}", method, err),
        };
    }
}

pub fn start_xi_core() -> (Writer, Reader, ClientToClientWriter) {
    let mut core = XiCore::new();

    let (to_core_tx, to_core_rx) = channel();
    let client_to_core_writer = Writer(to_core_tx);
    let client_to_core_reader = Reader(to_core_rx);

    let (from_core_tx, from_core_rx) = channel();
    let core_to_client_writer = Writer(from_core_tx.clone());
    let core_to_client_reader = Reader(from_core_rx);

    let client_to_client_writer = ClientToClientWriter(Writer(from_core_tx));

    let mut core_event_loop = RpcLoop::new(core_to_client_writer);
    thread::spawn(move || core_event_loop.mainloop(|| client_to_core_reader, &mut core));

    (
        client_to_core_writer,
        core_to_client_reader,
        client_to_client_writer,
    )
}

#[derive(Debug)]
pub enum RpcOperations {
    Update(Update),
    ScrollTo(ScrollTo),
    DefStyle(Style),
    AvailablePlugins(AvailablePlugins),
    UpdateCmds(UpdateCmds),
    PluginStarted(PluginStarted),
    PluginStopped(PluginStopped),
    ConfigChanged(ConfigChanged),
    ThemeChanged(ThemeChanged),
    Alert(Alert),
    AvailableThemes(AvailableThemes),
    FindStatus(FindStatus),
    ReplaceStatus(ReplaceStatus),
    AvailableLanguages(AvailableLanguages),
    LanguageChanged(LanguageChanged),
    MeasureWidth((u64, MeasureWidth)),
    None,
}

pub fn handle_notification(method: String, params: Value) -> RpcOperations {
    match method.as_str() {
        "update" => RpcOperations::Update(from_value::<Update>(params).unwrap()),
        "scroll_to" => RpcOperations::ScrollTo(from_value::<ScrollTo>(params).unwrap()),
        "def_style" => RpcOperations::DefStyle(from_value::<Style>(params).unwrap()),
        "available_plugins" => {
            RpcOperations::AvailablePlugins(from_value::<AvailablePlugins>(params).unwrap())
        }
        "plugin_started" => {
            RpcOperations::PluginStarted(from_value::<PluginStarted>(params).unwrap())
        }
        "plugin_stopped" => {
            RpcOperations::PluginStopped(from_value::<PluginStopped>(params).unwrap())
        }
        "update_cmds" => RpcOperations::UpdateCmds(from_value::<UpdateCmds>(params).unwrap()),
        "config_changed" => {
            RpcOperations::ConfigChanged(from_value::<ConfigChanged>(params).unwrap())
        }
        "theme_changed" => RpcOperations::ThemeChanged(from_value::<ThemeChanged>(params).unwrap()),
        "alert" => RpcOperations::Alert(from_value::<Alert>(params).unwrap()),
        "available_themes" => {
            RpcOperations::AvailableThemes(from_value::<AvailableThemes>(params).unwrap())
        }
        "find_status" => RpcOperations::FindStatus(from_value::<FindStatus>(params).unwrap()),
        "replace_status" => {
            RpcOperations::ReplaceStatus(from_value::<ReplaceStatus>(params).unwrap())
        }
        "available_languages" => {
            RpcOperations::AvailableLanguages(from_value::<AvailableLanguages>(params).unwrap())
        }
        "language_changed" => {
            RpcOperations::LanguageChanged(from_value::<LanguageChanged>(params).unwrap())
        }
        _ => {
            RpcOperations::None
            // unreachable!("Unknown method {}", method)
        }
    }
}
