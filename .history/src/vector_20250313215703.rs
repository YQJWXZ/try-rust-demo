use anyhow::Result;
pub struct Vector<T> {
    data: Vec<T>,
}
use std::ops::{Deref, Index};

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
