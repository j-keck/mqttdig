use crate::{
    error::*,
    mqtt::{
        subscriber::Subscriber,
        publisher::Publisher,
    },
    Result,
    MqttArgs,
};
use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions};
use std::{
    convert::TryFrom,
    sync::{
        Arc,
        Mutex,
    },
    time::SystemTime,
};
use serde::Serialize;


mod publisher;
mod subscriber;
mod payload;
use payload::Payload;



#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub ts: SystemTime,
    pub topic: String,
    pub payload: Payload,
}

#[derive(Debug)]
pub enum Event {
    Disconnection(SystemTime),
    Reconnection(SystemTime),
    Message(Message),
}

impl From<Message> for Event {
    fn from(msg: Message) -> Self {
        Self::Message(msg)
    }
}



pub struct Mqtt {
    client: Arc<Mutex<MqttClient>>,
    subscriber: Subscriber,
    publisher: Publisher,
}

impl Mqtt {
    pub fn connect(args: MqttArgs) -> Result<Self, Error> {
        let keep_alive = u16::try_from(args.mqtt_keep_alive.as_secs()).map_err(|_| {
            Error::new(
                ErrorKind::UserInput,
                "keep-alive out of range - max value: 65535s",
            )
        })?;

        let reconnect_opts =
            ReconnectOptions::AfterFirstSuccess(args.mqtt_reconnect_after.as_secs());

        let mqtt_options = MqttOptions::new("mqttdig", args.mqtt_host.clone(), args.mqtt_port)
            .set_keep_alive(keep_alive)
            .set_reconnect_opts(reconnect_opts)
            .set_clean_session(true);

        let (client, notifications) = MqttClient::start(mqtt_options).unwrap();
        let client = Arc::new(Mutex::new(client));
        let subscriber = Subscriber::new(client.clone());
        let publisher = Publisher::new(notifications);

        Ok(Self {
            client,
            subscriber,
            publisher,
        })
    }

    pub fn subscribe<S>(&mut self, topic: S) -> Result<(), Error>
    where
        S: Into<String> + std::fmt::Debug,
    {
        println!("subscribe: {:?}", topic);
        self.subscriber.subscribe(topic)
    }

    pub fn publish<S, V>(&mut self, topic: S, payload: V) -> Result<(), Error>
    where
        S: Into<String>,
        V: Into<Vec<u8>>,
    {
        loop {
            if let Ok(mut client) = self.client.try_lock() {
                return Ok(client.publish(topic, QoS::AtLeastOnce, false, payload)?);
            } else {
                println!("WAIT");
            }
        }

        // Ok(self
        //     .client
        //     .lock()
        //     .unwrap()
        //     .publish(topic, QoS::AtLeastOnce, false, payload)?)
    }

    pub fn register_subscriber<F>(&mut self, f: F)
    where
        F: Fn(&Event) -> () + Send + Sync + 'static,
    {
        self.publisher.register_subscriber(Box::new(f));
    }
}
