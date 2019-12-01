use crate::{
    mqtt,
    Result,
};
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    collections::HashMap,
    thread,
};
use tungstenite::{
    Message as WsMessage,
    server as ws,
};
use log::debug;

pub fn start(host: String, bus: Arc<Mutex<bus::Bus<mqtt::Message>>>) -> Result<()>{
    let cache = Arc::new(Mutex::new(HashMap::new()));
    thread::spawn({
        let mut rx = bus.lock().unwrap().add_rx();
        let cache = cache.clone();
        move || {
            while let Ok(msg) = rx.recv() {
                cache.lock().unwrap().insert(msg.topic.clone(), msg);
            }
        }
    });



    debug!("startup ws server");
    let addr = format!("{}:9001", host);
    let server = TcpListener::bind(addr).expect("unable to start ws-server");
    thread::spawn(move || {
        for stream in server.incoming() {
            let stream = stream.expect("stream error");
            let mut rx = bus.lock().unwrap().add_rx();
            let cache = cache.clone();
            thread::spawn(move || {
                debug!("new ws client connected - {:?}", &stream.peer_addr());
                let mut ws = ws::accept(stream).unwrap();


                // send cached values
                // FIXME: create struct 'Cache' which sort's the values,
                // and removes old messages.
                // this is in his own block to unlock the MutexGuard after sending the messages
                {
                    let cache = cache.lock().unwrap();
                    let mut values = cache.values().collect::<Vec<_>>();
                    values.sort_by_key(|msg| msg.ts);
                    for msg in values {
                        ws.write_message(WsMessage::Text(serde_json::to_string(msg).unwrap())).expect("unable to send cached message");
                    }
                }


                // wait for new messages and publish
                while let Ok(msg) = rx.recv() {
                    ws.write_message(WsMessage::Text(serde_json::to_string(&msg).unwrap())).expect("unable to send message");
                }
            });
        }
    });
    Ok(())
}
