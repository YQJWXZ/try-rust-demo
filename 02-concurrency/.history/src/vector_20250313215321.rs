use anyhow::Result;
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&mut self, item: T) {}
}
