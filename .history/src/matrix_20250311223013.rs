use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

// [[1,2], [1,2], [1,2]] -> [1,2,1,2,1,2]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix multiply error: a.cols != b.rows"));
    }
    let mut result = vec![T::default(); a.rows * b.cols];
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}
