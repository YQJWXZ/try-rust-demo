use std::sync::Arc;

use dashmap::DashMap;

use crate::RespFrame;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

struct BackendInner {
    map: DashMap<String, RespFrame>,
}
