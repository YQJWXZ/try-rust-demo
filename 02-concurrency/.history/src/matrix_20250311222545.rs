use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

// [[1,2], [1,2], [1,2]] -> [1,2,1,2,1,2]
pub struct Matrix<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub fn multiply(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Add<Output = T> + AddAssign<Output = T> + Mul<Output = T>,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix multiply error: a.cols != b.rows"));
    }
    let mut result = Matrix::new(a.rows, b.cols);
    for j in 0..b.cols {
        let mut sum = 0;
        for k in 0..a.cols {
            sum += a.get(i, k) * b.get(k, j);
        }
        result.set(i, j, sum);
    }
    Ok(result)
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
