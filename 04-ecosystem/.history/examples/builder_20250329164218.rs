use derive_builder::Builder;
use anyhow::Result;

#[derive(Debug,Builder)]
struct MyStruct {
  name: String,
  age: u32,
}
fn main() -> Result<()>{

  #[builder(setter(into))]
  name: 
  Ok(())
}
