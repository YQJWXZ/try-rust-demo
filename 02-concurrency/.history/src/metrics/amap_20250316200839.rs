// Atomic map

use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: self.data.clone(),
        }
    }
}
