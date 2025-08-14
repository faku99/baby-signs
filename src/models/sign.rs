use serde::Serialize;
use tuono_lib::Type;

#[derive(Serialize, Type)]
pub struct Sign {
    id: &'static str,
}

impl Sign {
    pub const HELLO: Sign = Sign { id: "hello" };

    pub fn signs() -> Vec<Sign> {
        vec![Sign::HELLO]
    }
}
