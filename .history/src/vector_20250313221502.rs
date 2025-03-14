pub struct Vector<T> {
    data: Vec<T>,
}
use std::ops::{Deref, Index};

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
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

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    if a.len() != b.len() {
        // a.len() => a.data.len() (Deref trait)
        return Err(anyhow!("Matrix dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}
