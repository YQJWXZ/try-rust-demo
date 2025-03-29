use anyhow::Result;
use derive_builder::Builder;

#[derive(Debug, Builder)]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(default = "42")]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}
fn main() -> Result<()> {
    Ok(())
}
