// metrics data structure
// 基本功能：inc/dec/snapshot

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: HashMap::new(),
        }
    }

    pub fn inc(&mut self, key: &str) {
        let counter = self.data.entry(key.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn dec(&mut self, key: &str) {
        let counter = self.data.entry(key.to_string()).or_insert(0);
        *counter -= 1;
    }

    pub fn snapshot(&self) -> HashMap<String, i64> {
        self.data.clone()
    }
}
