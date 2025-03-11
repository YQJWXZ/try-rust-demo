use anyhow::Result;
fn main() -> Result<()> {
    let a = vec![1, 2, 3, 4];
    let b = vec![2, 3, 4];

    let ret = merge_and_sum(&a, &b);
    println!("{:?}", ret);
    Ok(())
}

fn merge_and_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut ret = Vec::new();
    let max_len = a.len().max(b.len());
    for i in 0..max_len {
        ret.push(a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0));
    }

    ret
}
