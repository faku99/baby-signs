use serde::Serialize;
use tuono_lib::Type;

#[derive(Clone, Serialize, Type)]
pub struct Sign {
    pub id: &'static str,
}

impl Sign {
    pub const AGAIN: Sign = Sign { id: "again" };
    pub const ANGRY: Sign = Sign { id: "angry" };
    pub const ASHAMED: Sign = Sign { id: "ashamed" };
    pub const BIBERON: Sign = Sign { id: "biberon" };
    pub const BOOK: Sign = Sign { id: "book" };
    pub const CALM: Sign = Sign { id: "calm" };
    pub const DONE: Sign = Sign { id: "done" };
    pub const DOUDOU: Sign = Sign { id: "doudou" };
    pub const DRINK: Sign = Sign { id: "drink" };
    pub const EAT: Sign = Sign { id: "eat" };
    pub const HAPPY: Sign = Sign { id: "happy" };
    pub const MILK: Sign = Sign { id: "milk" };
    pub const NIGHT: Sign = Sign { id: "night" };
    pub const PACIFIER: Sign = Sign { id: "pacifier" };
    pub const PROUD: Sign = Sign { id: "proud" };
    pub const SAD: Sign = Sign { id: "sad" };
    pub const SCARED: Sign = Sign { id: "scared" };
    pub const SLEEP: Sign = Sign { id: "sleep" };
    pub const STORY: Sign = Sign { id: "story" };
    pub const TIRED: Sign = Sign { id: "tired" };
    pub const WATER: Sign = Sign { id: "water" };

    pub fn signs() -> Vec<Sign> {
        vec![
            Sign::AGAIN,
            Sign::ANGRY,
            Sign::ASHAMED,
            Sign::BIBERON,
            Sign::BOOK,
            Sign::CALM,
            Sign::DONE,
            Sign::DOUDOU,
            Sign::DRINK,
            Sign::EAT,
            Sign::HAPPY,
            Sign::MILK,
            Sign::NIGHT,
            Sign::PACIFIER,
            Sign::PROUD,
            Sign::SAD,
            Sign::SCARED,
            Sign::SLEEP,
            Sign::STORY,
            Sign::TIRED,
            Sign::WATER,
        ]
    }
}
