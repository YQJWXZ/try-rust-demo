use std::sync::Arc;

use dashmap::DashMap;
use std::ops::Deref;
use crate::RespFrame;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug)]
struct BackendInner {
    map: DashMap<String, RespFrame>,
}
