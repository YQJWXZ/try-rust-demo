use anyhow::{anyhow, Result};

use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::{dot_product, Vector};

const NUM_THREADS: usize = 4;

// [[1,2], [1,2], [1,2]] -> [1,2,1,2,1,2]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    // sender to send the result back
    sender: oneshot::Sender<MsgOutput<T>>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Send + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix multiply error: a.cols != b.rows"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, mut rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col).unwrap();
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send Error: {:?}", e);
                    }
                }

                Ok::<_, anyhow::Error>(())
            });

            tx
        })
        .collect::<Vec<_>>();

    // generate 4 threads which receive msg and do not product

    let mut data = Vec::with_capacity(a.rows * b.cols);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let input = MsgInput::new(i * b.cols + j, row, col);
            let msg = data[i * b.cols + j] += dot_product(row, col)?;
        }
    }

    Ok(Matrix {
        data,
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

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
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
