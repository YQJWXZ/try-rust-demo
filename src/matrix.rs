use std::{
    fmt,
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
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                result[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }
    Ok(Matrix {
        data: result,
        rows: a.rows,
        cols: b.cols,
    })
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j != self.cols - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix (row={}, col={},{})", self.rows, self.cols, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.cols, 2);
        assert_eq!(c.rows, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix (row=2, col=2,{22 28, 49 64})");

        Ok(())
    }

    #[test]
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        assert_eq!(format!("{}", c), "{7 10, 15 22}");
        Ok(())
    }
}
