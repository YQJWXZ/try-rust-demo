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

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn build(&self) -> Result<User> {
        let mut user = self._priv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as _;
        Ok(user)
    }

    pub fn dob(&mut self, val: &str) -> &mut Self {
      self.dob = DateTime::parse_from_rfc3339(val).map(|dt| dt.with_timezone(&Utc)).ok()

      self
    }
}
