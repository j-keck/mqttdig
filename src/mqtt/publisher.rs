use log::{debug, info, warn};
use rumqtt::Notification;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
};

use crate::mqtt::{Event, Message, Payload};

type Subscriber = Box<dyn Fn(&Event) -> () + Sync + Send>;

pub struct Publisher {
    subscribers: Arc<Mutex<Vec<Subscriber>>>,
}

impl Publisher {
    pub fn new(notifications: rumqtt::Receiver<Notification>) -> Self {
        let subscribers = Arc::new(Mutex::new(Vec::new()));
        thread::spawn({
            let subscribers = subscribers.clone();
            move || {
                for notification in notifications {
                    debug!("new notification: {:?}", notification);
                    let ts = SystemTime::now();
                    match notification {
                        Notification::Publish(publish) => {
                            let topic = publish.topic_name;
                            let payload = Payload::new((*publish.payload).clone());
                            Publisher::publish(
                                &subscribers,
                                Message { ts, topic, payload }.into(),
                            );
                        }
                        Notification::Disconnection => {
                            warn!("disconnected from mqtt server");
                            Publisher::publish(&subscribers, Event::Disconnection(ts));
                        }

                        Notification::Reconnection => {
                            info!("reconnected to the mqtt server");
                            Publisher::publish(&subscribers, Event::Reconnection(ts));
                        }
                        _ => warn!("ignore notification: {:#?}", notification),
                    };
                }
            }
        });
        Self { subscribers }
    }

    pub fn register_subscriber(&mut self, subscriber: Subscriber) {
        self.subscribers.lock().unwrap().push(subscriber);
    }

    fn publish(subscribers: &Arc<Mutex<Vec<Subscriber>>>, event: Event) {
        for subscriber in &*subscribers.lock().unwrap() {
            subscriber(&event)
        }
    }
}
