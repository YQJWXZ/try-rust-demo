// Atomic map

use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        AmapMetrics {
            data: Arc::new(HashMap::new()),
        }
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}
