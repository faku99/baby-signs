use serde::Serialize;

#[derive(Serialize)]
pub struct Sign {
    name: &'static str,
}

impl Sign {
    pub const HELLO: Sign = Sign { name: "hello" };

    pub fn signs() -> Vec<Sign> {
        vec![
            Sign::HELLO
        ]
    }
}
