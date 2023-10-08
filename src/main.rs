use serde::{Deserialize, Deserializer};

fn main() {
    assert_eq!(serde_xml::from_str::<Foo>("<x>12345</x>").unwrap().x, 12345);
}

struct Foo {
    x: i32
}

impl<'de> Deserialize for Foo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let x = i32::deserialize(deserializer)?;

        Ok(Self {
            x
        })
    }
}
