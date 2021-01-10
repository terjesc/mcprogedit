use std::convert::TryFrom;

use crate::block::Block;

#[derive(Clone, Debug, PartialEq)]
pub struct Noteblock {
    pub pitch: Pitch,
}

impl Noteblock {
    pub fn has_pitch_of(&self, pitch: Pitch) -> bool {
        self.pitch == pitch
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

#[derive(Clone, Debug, PartialEq)]
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
}

// TODO put somewhere suitable
// TODO utility functions
#[derive(Clone, Debug, PartialEq)]
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
