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
    AzureBluet,
    Bamboo,
    BirchSapling,
    BlueOrchid,
    BrownMushroom,
    Cactus,
    Cornflower,
    CrimsonFungus,
    CrimsonRoots,
    Dandelion,
    DarkOakSapling,
    DeadBush,
    Fern,
    JungleSapling,
    LilyOfTheValley,
    OakSapling,
    OxeyeDaisy,
    Poppy,
    RedMushroom,
    SpruceSapling,
    TulipOrange,
    TulipPink,
    TulipRed,
    TulipWhite,
    WarpedFungus,
    WarpedRoots,
    WitherRose,
}
