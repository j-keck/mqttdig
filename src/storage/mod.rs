use crate::mqtt::payload::Payload;
use crate::*;
use log::{debug, info, warn};
use std::sync::{Arc, Mutex};

pub struct Storage {
    backend: Arc<Mutex<Box<dyn Backend + Send>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            backend: Arc::new(Mutex::new(Box::new(BackendMock::default()))),
        }
    }

    pub fn find_messages<S>(&mut self, pattern: S) -> Vec<String>
    where
        S: Into<String>,
    {
        let mut backend = self.backend.lock().unwrap();
        backend.find_messages(pattern.into()).unwrap()
    }

    pub fn persist_message<S>(&mut self, topic: S, payload: Payload) -> Result<(), Error>
    where
        S: Into<String>,
    {
        let mut backend = self.backend.lock().unwrap();
        backend.persist_message(topic.into(), payload)
    }
}

pub trait Backend {
    fn find_messages(&mut self, pattern: String) -> Result<Vec<String>, Error>;
    fn persist_message(&mut self, topic: String, payload: Payload) -> Result<(), Error>;
}

pub struct BackendMock {
    history: Vec<(String, Payload)>,
}
impl Default for BackendMock {
    fn default() -> Self {
        Self { history: vec![] }
    }
}
impl Backend for BackendMock {
    fn find_messages(&mut self, pattern: String) -> Result<Vec<String>, Error> {
        info!("find messages with pattern: {}", pattern);
        Ok(self
            .history
            .clone()
            .into_iter()
            .map(|(topic, _)| topic)
            .collect())
        //        Ok(vec!["hallo", "du", "da"].into_iter().map(String::from).collect())
    }

    fn persist_message(&mut self, topic: String, payload: Payload) -> Result<(), Error> {
        info!(
            "persist messsage - topic: {}, payload: {:?}",
            topic, payload
        );
        self.history.push((topic, payload));
        Ok(())
    }
}
