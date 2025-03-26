use dashmap::DashMap;

use crate::RespFrame;

#[derive(Debug, Clone)]
pub struct Backend {
    map: DashMap<String, RespFrame>,
}
