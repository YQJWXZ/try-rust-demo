use anyhow::Result;
use derive_builder::Builder;

#[derive(Debug, Builder)]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(default = "42")]
    age: u32,
}
fn main() -> Result<()> {
    Ok(())
}
