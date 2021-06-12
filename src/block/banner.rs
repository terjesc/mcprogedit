use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, CommonTags};
use crate::colour::*;
use crate::positioning::*;

/// Banner "block".
#[derive(Clone, Debug, PartialEq)]
pub struct Banner {
    /// Base colour of the banner.
    pub colour: Colour,
    /// If used: The name is used as a marker on maps.
    pub custom_name: Option<String>,
    pub placement: WallOrRotatedOnFloor,
    /// List of (normally) up to 6 coloured patterns,
    /// that are featured on top of each other the banner.
    pub patterns: Vec<ColouredPattern>,
}

impl Banner {
    pub fn has_colour_of(&self, colour: Colour) -> bool {
        colour == self.colour
    }

    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.placement.into()
    }

    pub fn is_on_floor(&self) -> bool {
        self.placement.is_on_floor()
    }

    pub fn is_on_wall(&self) -> bool {
        self.placement.is_on_wall()
    }

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Banner {
            common: CommonTags {
                id: "minecraft:banner".into(),
                x,
                y,
                z,
                keep_packed: false,
            },
            colour: self.colour,
            custom_name: self.custom_name.clone(),
            patterns: self.patterns.clone(),
        }
    }
}

impl TryFrom<Block> for Banner {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Banner(banner) => Ok(*banner),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColouredPattern {
    pub colour: Colour,
    pub pattern: BannerPattern,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BannerPattern {
    BaseColor,               // b (base)
    Base,                    // bs (bottom stripe)
    Chief,                   // ts (top stripe)
    PaleDexter,              // ls (left stripe)
    PaleSinister,            // rs (right sripe)
    Pale,                    // cs (center stripe, vertical)
    Fess,                    // ms (middle sripe, horizontal)
    Bend,                    // drs (down right stripe)
    BendSinister,            // dls (down left stripe)
    Paly,                    // ss (smal vertical stripes)
    Saltire,                 // cr (diagonal cross)
    Cross,                   // sc (square cross)
    PerBendSinister,         // ld (left of diagonal)
    PerBend,                 // rud (right of upside-down diagonal)
    PerBendInverted,         // lud (left of upside-down diagonal)
    PerBendSinisterInverted, // rd (right of diagonal)
    PerPale,                 // vh (vertical half left)
    PerPaleInverted,         // vhr (vertical half right)
    PerFess,                 // hh (horizontal half top)
    PerFessInverted,         // hhb (horizontal half bottom)
    BaseDexterCanton,        // bl (bottom left corner)
    BaseSinisterCanton,      // br (bottom right corner)
    ChiefDexterCanton,       // tl (top left corner)
    ChiefSinisterCanton,     // tr (top right corner)
    Chevron,                 // bt (bottom triangle)
    InvertedChevron,         // tt (top triangle)
    BaseIndented,            // bts (bottom triangle sawtooth
    ChiefIndented,           // tts (top triangle sawtooth
    Roundel,                 // mc (middle circle)
    Lozenge,                 // mr (middle rhombus)
    Bordure,                 // bo (border)
    BordureIndented,         // cbo (curly border)
    FieldMasoned,            // bri (brick)
    Gradient,                // gra (gradient)
    BaseGradient,            // gru (gradient upside-down)
    CreeperCharge,           // cre (creeper)
    SkullCharge,             // sku (skull)
    FlowerCharge,            // flo (flower)
    Thing,                   // moj (mojang)
    Globe,                   // glb (globe)
    Snout,                   // pig (piglin)
}

impl From<&str> for BannerPattern {
    fn from(pattern_string: &str) -> Self {
        match pattern_string {
            "b" => Self::BaseColor,                // b (base)
            "bs" => Self::Base,                    // bs (bottom stripe)
            "ts" => Self::Chief,                   // ts (top stripe)
            "ls" => Self::PaleDexter,              // ls (left stripe)
            "rs" => Self::PaleSinister,            // rs (right sripe)
            "cs" => Self::Pale,                    // cs (center stripe, vertical)
            "ms" => Self::Fess,                    // ms (middle sripe, horizontal)
            "drs" => Self::Bend,                   // drs (down right stripe)
            "dls" => Self::BendSinister,           // dls (down left stripe)
            "ss" => Self::Paly,                    // ss (smal vertical stripes)
            "cr" => Self::Saltire,                 // cr (diagonal cross)
            "sc" => Self::Cross,                   // sc (square cross)
            "ld" => Self::PerBendSinister,         // ld (left of diagonal)
            "rud" => Self::PerBend,                // rud (right of upside-down diagonal)
            "lud" => Self::PerBendInverted,        // lud (left of upside-down diagonal)
            "rd" => Self::PerBendSinisterInverted, // rd (right of diagonal)
            "vh" => Self::PerPale,                 // vh (vertical half left)
            "vhr" => Self::PerPaleInverted,        // vhr (vertical half right)
            "hh" => Self::PerFess,                 // hh (horizontal half top)
            "hhb" => Self::PerFessInverted,        // hhb (horizontal half bottom)
            "bl" => Self::BaseDexterCanton,        // bl (bottom left corner)
            "br" => Self::BaseSinisterCanton,      // br (bottom right corner)
            "tl" => Self::ChiefDexterCanton,       // tl (top left corner)
            "tr" => Self::ChiefSinisterCanton,     // tr (top right corner)
            "bt" => Self::Chevron,                 // bt (bottom triangle)
            "tt" => Self::InvertedChevron,         // tt (top triangle)
            "bts" => Self::BaseIndented,           // bts (bottom triangle sawtooth
            "tts" => Self::ChiefIndented,          // tts (top triangle sawtooth
            "mc" => Self::Roundel,                 // mc (middle circle)
            "mr" => Self::Lozenge,                 // mr (middle rhombus)
            "bo" => Self::Bordure,                 // bo (border)
            "cbo" => Self::BordureIndented,        // cbo (curly border)
            "bri" => Self::FieldMasoned,           // bri (brick)
            "gra" => Self::Gradient,               // gra (gradient)
            "gru" => Self::BaseGradient,           // gru (gradient upside-down)
            "cre" => Self::CreeperCharge,          // cre (creeper)
            "sku" => Self::SkullCharge,            // sku (skull)
            "flo" => Self::FlowerCharge,           // flo (flower)
            "moj" => Self::Thing,                  // moj (mojang)
            "glb" => Self::Globe,                  // glb (globe)
            "pig" => Self::Snout,                  // pig (piglin)
            other => panic!("Unknown banner pattern string: {}", other),
        }
    }
}
