use std::sync::Arc;

use dashmap::DashMap;
use std::ops::Deref;
use crate::RespFrame;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug)]
pub struct BackendInner {
    map: DashMap<String, RespFrame>,
    hmap: DashMap<String, DashMap<String, RespFrame>>,
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BackendInner {
    pub fn new() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self(Arc::new(BackendInner::new()))
    }
}
