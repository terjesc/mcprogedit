use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, CommonTags};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Noteblock {
    pub pitch: Pitch,
}

impl Noteblock {
    pub fn has_pitch_of(&self, pitch: Pitch) -> bool {
        self.pitch == pitch
    }

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Noteblock {
            common: CommonTags {
                id: "minecraft:noteblock".into(),
                x,
                y,
                z,
                keep_packed: false,
            },
            note: self.pitch,
            powered: false,
        }
    }
}

impl TryFrom<Block> for Noteblock {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Noteblock(noteblock) => Ok(noteblock),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Pitch {
    Fs0 = 0,
    G0,
    Gs0,
    A0,
    As0,
    B0,
    C1,
    Cs1,
    D1,
    Ds1,
    E1,
    F1,
    Fs1,
    G1,
    Gs1,
    A1,
    As1,
    B1,
    C2,
    Cs2,
    D2,
    Ds2,
    E2,
    F2,
    Fs2,
}

impl Pitch {
    pub fn from_value(value: u8) -> Self {
        match value {
            0 => Pitch::Fs0,
            1 => Pitch::G0,
            2 => Pitch::Gs0,
            3 => Pitch::A0,
            4 => Pitch::As0,
            5 => Pitch::B0,
            6 => Pitch::C1,
            7 => Pitch::Cs1,
            8 => Pitch::D1,
            9 => Pitch::Ds1,
            10 => Pitch::E1,
            11 => Pitch::F1,
            12 => Pitch::Fs1,
            13 => Pitch::G1,
            14 => Pitch::Gs1,
            15 => Pitch::A1,
            16 => Pitch::As1,
            17 => Pitch::B1,
            18 => Pitch::C2,
            19 => Pitch::Cs2,
            20 => Pitch::D2,
            21 => Pitch::Ds2,
            22 => Pitch::E2,
            23 => Pitch::F2,
            24 => Pitch::Fs2,
            n => panic!("Pitch value {} is out of range!", n),
        }
    }

    pub fn to_i8(self) -> i8 {
        match self {
            Pitch::Fs0 => 0,
            Pitch::G0 => 1,
            Pitch::Gs0 => 2,
            Pitch::A0 => 3,
            Pitch::As0 => 4,
            Pitch::B0 => 5,
            Pitch::C1 => 6,
            Pitch::Cs1 => 7,
            Pitch::D1 => 8,
            Pitch::Ds1 => 9,
            Pitch::E1 => 10,
            Pitch::F1 => 11,
            Pitch::Fs1 => 12,
            Pitch::G1 => 13,
            Pitch::Gs1 => 14,
            Pitch::A1 => 15,
            Pitch::As1 => 16,
            Pitch::B1 => 17,
            Pitch::C2 => 18,
            Pitch::Cs2 => 19,
            Pitch::D2 => 20,
            Pitch::Ds2 => 21,
            Pitch::E2 => 22,
            Pitch::F2 => 23,
            Pitch::Fs2 => 24,
        }
    }
}

// TODO put somewhere suitable
// TODO utility functions
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Instrument {
    Banjo,
    Basedrum,
    Bass,
    Bell,
    Bit,
    Chime,
    CowBell,
    Didgeridoo,
    Flute,
    Guitar,
    Harp,
    Hat,
    IronXylophone,
    Pling,
    Snare,
    Xylophone,
}
