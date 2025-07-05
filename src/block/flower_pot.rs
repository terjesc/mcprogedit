use std::convert::TryFrom;

use crate::block::Block;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FlowerPot {
    pub(crate) plant: Option<PottedPlant>,
}

impl FlowerPot {
    pub fn new_empty() -> Self {
        Self { plant: None }
    }

    pub fn new_with_plant(plant: PottedPlant) -> Self {
        Self { plant: Some(plant) }
    }

    pub fn has_plant_of(&self, plant: PottedPlant) -> bool {
        self.plant == Some(plant)
    }

    pub fn is_empty(&self) -> bool {
        self.plant.is_none()
    }
}

impl Default for FlowerPot {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl TryFrom<Block> for FlowerPot {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::FlowerPot(flower_pot) => Ok(flower_pot),
            _ => Err(()),
        }
    }
}

impl From<FlowerPot> for Block {
    fn from(flower_pot: FlowerPot) -> Block {
        Block::FlowerPot(flower_pot)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PottedPlant {
    AcaciaSapling,
    Allium,
    Azalea,
    AzureBluet,
    Bamboo,
    BirchSapling,
    BlueOrchid,
    BrownMushroom,
    Cactus,
    //CherrySapling,
    //ClosedEyeblossom,
    Cornflower,
    CrimsonFungus,
    CrimsonRoots,
    Dandelion,
    DarkOakSapling,
    DeadBush,
    Fern,
    FloweringAzalea,
    JungleSapling,
    LilyOfTheValley,
    //MangrovePropagule,
    OakSapling,
    //OpenEyeblossom,
    OxeyeDaisy,
    //PaleOakSapling,
    Poppy,
    RedMushroom,
    SpruceSapling,
    //Torchflower,
    TulipOrange,
    TulipPink,
    TulipRed,
    TulipWhite,
    WarpedFungus,
    WarpedRoots,
    WitherRose,
}

impl From<super::Flower> for FlowerPot {
    fn from(flower: super::Flower) -> Self {
        match flower {
            super::Flower::Allium => FlowerPot{ plant: Some(PottedPlant::Allium) },
            super::Flower::AzureBluet => FlowerPot{ plant: Some(PottedPlant::AzureBluet) },
            super::Flower::BlueOrchid => FlowerPot{ plant: Some(PottedPlant::BlueOrchid) },
            super::Flower::Cornflower => FlowerPot{ plant: Some(PottedPlant::Cornflower) },
            super::Flower::Dandelion => FlowerPot{ plant: Some(PottedPlant::Dandelion) },
            super::Flower::LilacBottom => FlowerPot{ plant: None },
            super::Flower::LilacTop => FlowerPot{ plant: None },
            super::Flower::LilyOfTheValley => FlowerPot{ plant: Some(PottedPlant::LilyOfTheValley) },
            super::Flower::OxeyeDaisy => FlowerPot{ plant: Some(PottedPlant::OxeyeDaisy) },
            super::Flower::PeonyBottom => FlowerPot{ plant: None },
            super::Flower::PeonyTop => FlowerPot{ plant: None },
            super::Flower::Poppy => FlowerPot{ plant: Some(PottedPlant::Poppy) },
            super::Flower::RoseBushBottom => FlowerPot{ plant: None },
            super::Flower::RoseBushTop => FlowerPot{ plant: None },
            super::Flower::SunflowerBottom => FlowerPot{ plant: None },
            super::Flower::SunflowerTop => FlowerPot{ plant: None },
            super::Flower::TulipWhite => FlowerPot{ plant: Some(PottedPlant::TulipWhite) },
            super::Flower::TulipOrange => FlowerPot{ plant: Some(PottedPlant::TulipOrange) },
            super::Flower::TulipPink => FlowerPot{ plant: Some(PottedPlant::TulipPink) },
            super::Flower::TulipRed => FlowerPot{ plant: Some(PottedPlant::TulipRed) },
            super::Flower::WitherRose => FlowerPot{ plant: Some(PottedPlant::WitherRose) },
        }
    }
}
