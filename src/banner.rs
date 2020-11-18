use crate::colour::*;
use crate::positioning::*;

/// Banner "block".
#[derive(Clone, PartialEq)]
pub struct Banner {
    /// Base colour of the banner.
    pub colour: Colour,
    /// If used: The name is used as a marker on maps.
    pub custom_name: Option<String>,
    pub placement: WallOrRotatedOnFloor,
    /// List of up to 6 coloured patterns that are featured on the banner.
    pub patterns: [ColouredPattern; 6],
}

// [ BANNER ]
//
// TAGS
//
// CustomName: Option<TAG_String> (The name of this banner in JSON text component, which appears
//                                 when added as markers on maps.)
// Patterns: TAG_List (List of all patterns applied to the banner.)
//  -> TAG_Compound (Individual pattern)
//      -> Color: TAG_Int (Color of the section.)
//      -> Pattern: TAG_String (The banner pattern code the color is applied to.)
//
// VALUES
//
// Values for Color: White: 0, Orange: 1, Magenta: 2, LightBlue: 3, Yellow: 4,
//                   Lime: 5, Pink: 6, Gray: 7, LightGray: 8, Cyan: 9, Purple: 10,
//                   Blue: 11, Brown: 12, Green: 13, Red: 14, Black: 15.
//

#[derive(Clone, PartialEq)]
pub struct ColouredPattern {
    colour: Colour,
    pattern: BannerPattern,
}

#[derive(Clone, PartialEq)]
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
