use std::fmt::Debug;

use anyhow::{Ok, Result};

// [[1,2], [1,2], [1,2]] -> [1,2,1,2,1,2]
struct Matrix<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}
fn main() -> Result<()> {
    Ok(())
}

fn multiply() {}
