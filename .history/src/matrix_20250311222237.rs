use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

// [[1,2], [1,2], [1,2]] -> [1,2,1,2,1,2]
struct Matrix<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}
