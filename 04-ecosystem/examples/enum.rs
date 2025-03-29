use anyhow::Result;
use serde::Serialize;
use strum::{
    Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator,
    IntoStaticStr, VariantNames,
};

#[derive(Debug, Display, Serialize)]
enum Color {
    #[strum(serialize = "redred", to_string = "red")] // to_string优先于serialize
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

#[derive(
    Debug,
    Display,
    Serialize,
    EnumString,
    EnumCount,
    EnumDiscriminants,
    EnumIter,
    EnumIs,
    IntoStaticStr,
    VariantNames,
)]
enum MyEnum {
    A,
    B(String),
    C,
    D,
}

fn main() -> Result<()> {
    println!("{:?}", MyEnum::VARIANTS);
    MyEnum::iter().for_each(|v| println!("{:?}", v));
    println!("total: {:?}", MyEnum::COUNT);

    let my_enum = MyEnum::B("hello".to_string());
    println!("is B: {:?}", my_enum.is_b());
    let s: &'static str = my_enum.into();
    println!("{}", s);

    let red = Color::Red;
    let green = Color::Green { range: 10 };
    let blue = Color::Blue(20);
    let yellow = Color::Yellow;
    let purple = Color::Purple { sat: 30 };

    println!(
        "red:{}, green:{}, blue:{}, yellow:{}, purple:{}",
        red, green, blue, yellow, purple
    );

    let red_str = serde_json::to_string(&red)?;
    println!("red_str: {}", red_str);
    Ok(())
}
