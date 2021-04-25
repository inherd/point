use crate::errors::DecodeError;
use crate::message::{Message, Notification, Request, Response};
use crate::structs::{
    Alert, AvailableLanguages, AvailablePlugins, AvailableThemes, ConfigChanged, FindStatus,
    LanguageChanged, MeasureWidth, PluginStarted, PluginStopped, ReplaceStatus, ScrollTo, Style,
    ThemeChanged, Update, UpdateCmds,
};
use druid::Data;
use pipe::{pipe, PipeReader, PipeWriter};
use serde_json::{self, from_value, json, to_vec, Value};
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{fmt, thread};
use xi_core_lib::XiCore;
use xi_rpc::RpcLoop;

type XiSender = Mutex<PipeWriter>;
type XiReceiver = PipeReader;

pub trait Callback: Send {
    fn call(self: Box<Self>, result: Result<Value, Value>);
}

impl<F: FnOnce(Result<Value, Value>) + Send> Callback for F {
    fn call(self: Box<Self>, result: Result<Value, Value>) {
        (*self)(result)
    }
}

pub struct Client {
    sender: XiSender,
    receiver: XiReceiver,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("")
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl Data for Client {
    fn same(&self, other: &Self) -> bool {
        return true;
    }
}

impl Default for Client {
    fn default() -> Self {
        let (mut receiver, sender) = Client::start_xi_thread();
        Client { sender, receiver }
    }
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

impl Client {
    pub fn new() -> Rc<Client> {
        let (mut receiver, sender) = Client::start_xi_thread();
        let client = Rc::new(Client { sender, receiver });

        client
    }

    fn handle_notification(method: String, params: Value) -> RpcOperations {
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
            "theme_changed" => {
                RpcOperations::ThemeChanged(from_value::<ThemeChanged>(params).unwrap())
            }
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

    fn start_xi_thread() -> (XiReceiver, XiSender) {
        let (to_core_rx, to_core_tx) = pipe();
        let (from_core_rx, from_core_tx) = pipe();
        let mut state = XiCore::new();
        let mut rpc_looper = RpcLoop::new(from_core_tx);
        thread::spawn(move || rpc_looper.mainloop(|| to_core_rx, &mut state));
        (from_core_rx, Mutex::new(to_core_tx))
    }

    pub fn client_started(&self, config_dir: Option<&String>, client_extras_dir: Option<&String>) {
        self.send_notification(
            "client_started",
            &json!({
                "config_dir": config_dir,
                "client_extras_dir": client_extras_dir,
            }),
        );
    }

    pub fn send_notification(&self, method: &str, params: &Value) {
        let cmd = json!({
            "method": method,
            "params": params,
        });
        let mut sender = self.sender.lock().unwrap();
        debug!("Xi-CORE <-- {}", cmd);
        sender.write_all(&to_vec(&cmd).unwrap()).unwrap();
        sender.write_all(b"\n").unwrap();
        sender.flush().unwrap();
    }
}
