use crate::Result;
use rumqtt::{MqttClient, QoS};
use std::sync::{Arc, Mutex};

pub struct Subscriber {
    client: Arc<Mutex<MqttClient>>,
    /// FIXME: resubscribe when clcean_session == true
    subscriptions: Vec<String>,
}

impl Subscriber {
    pub fn new(client: Arc<Mutex<MqttClient>>) -> Self {
        Self {
            client,
            subscriptions: Vec::new(),
        }
    }

    pub fn subscribe<S>(&mut self, topic: S) -> Result<()>
    where
        S: Into<String>,
    {
        let topic = topic.into();
        loop {
            if let Ok(mut client) = self.client.try_lock() {
                client.subscribe(&topic, QoS::AtLeastOnce)?;
                self.subscriptions.push(topic);
                return Ok(());
            } else {
                println!("WAIT");
            }
        }

        // self.client.lock().unwrap().subscribe(&topic, QoS::AtLeastOnce)?;
        // self.subscriptions.push(topic);
        // Ok(())
    }

}
