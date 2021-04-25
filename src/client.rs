use crate::message::{Message, Notification, Request, Response};
use druid::Data;
use glib::clone;
use pipe::{pipe, PipeReader, PipeWriter};
use serde_json::{to_vec, Value};
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
    pending_requests: Arc<Mutex<HashMap<u64, Box<dyn Callback>>>>,
    current_request_id: Cell<u64>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(format!("current_request_id: {:?}", self.current_request_id).as_str())
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
        let (mut _receiver, sender) = Client::start_xi_thread();
        Client {
            sender,
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            current_request_id: Cell::new(0),
        }
    }
}

impl Client {
    pub fn new() -> Rc<Client> {
        let (mut receiver, sender) = Client::start_xi_thread();
        let client = Rc::new(Client {
            sender,
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            current_request_id: Cell::new(0),
        });

        thread::spawn(
            clone!(@weak client.pending_requests as pending_requests => @default-panic, move || {
                let mut buf = String::new();
                while receiver.read_line(&mut buf).is_ok() {
                    let msg = Message::decode(&buf).unwrap();
                    trace!("Received message from xi: {:?}", msg);
                    match msg {
                        Message::Request(res) => {
                            let Request { method, params, id } = res;
                            println!("{:?}, {:?}", method, params);
                        }
                        Message::Response(res) => {
                            let Response { id, result } = res;
                            if let Some(cb) = pending_requests.lock().unwrap().remove(&id) {
                                cb.call(result);
                            }
                        }
                        Message::Notification(res) => {
                            let Notification { method, params } = res;
                            println!("{:?}, {:?}", method, params);
                        }
                    }

                    buf.clear();
                }
            }),
        );

        client
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

    fn send_notification(&self, method: &str, params: &Value) {
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
