use anyhow::Result;
use chrono::{DateTime, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Debug, Builder)]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,
    #[builder(setter(skip))]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}
fn main() -> Result<()> {
    let user = User::build()
        .name("Alice")
        .skill("programming")
        .skill("debugging")
        .email("alice@example.com")
        .dob("1990-01-01T00:00:00Z")
        .build();

    println!("{:?}", user);
    Ok(())
}
