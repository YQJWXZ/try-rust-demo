// metrics data structure
// 基本功能：inc/dec/snapshot

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}
// pub struct Metrics {
//     data: Arc<Mutex<HashMap<String, i64>>>,
// }

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        // Metrics {
        //     data: Arc::new(Mutex::new(HashMap::new())),
        // }
        Metrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        // Ok(self
        //     .data
        //     .lock()
        //     .map_err(|e| anyhow!(e.to_string()))?
        //     .clone())
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.read().unwrap();
        for (key, value) in data {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
