// metrics data structure
// 基本功能：inc/dec/snapshot

use std::collections::HashMap;

#[derive(Debug)]
pub struct Metrics<T> {
    data: HashMap<String, T>,
}
