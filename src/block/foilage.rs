use crate::block::Block;

impl Block {
    pub fn is_foilage(&self) -> bool {
        matches!(
            self,
            Block::Snow { .. }
                | Block::Lava { .. }
                | Block::LavaSource
                | Block::Leaves { .. }
                | Block::Spawner
                | Block::Water { .. }
                | Block::WaterSource
                | Block::Beehive { .. }
                | Block::BeeNest { .. }
                | Block::BrownMushroomBlock { .. }
                | Block::Cactus { .. }
                | Block::CarvedPumpkin { .. }
                | Block::CoralBlock { .. }
                | Block::HayBale { .. }
                | Block::JackOLantern { .. }
                | Block::Log(_)
                | Block::Melon
                | Block::Pumpkin { .. }
                | Block::RedMushroomBlock { .. }
                | Block::WarpedRoots
                | Block::None
                | Block::Air
                | Block::Bamboo { .. }
                | Block::Barrier
                | Block::Beetroots { .. }
                | Block::BrownMushroom
                | Block::Campfire { .. }
                | Block::Carrots { .. }
                | Block::CaveAir
                | Block::Cocoa { .. }
                | Block::Coral { .. }
                | Block::CoralFan { .. }
                | Block::CrimsonFungus
                | Block::CrimsonRoots
                | Block::DeadBush
                | Block::Fire { .. }
                | Block::Flower(_)
                | Block::Grass(_)
                | Block::Kelp { .. }
                | Block::LilyPad
                | Block::MelonStem { .. }
                | Block::MushroomStem { .. }
                | Block::NetherSprouts
                | Block::NetherWart { .. }
                | Block::Potatoes { .. }
                | Block::PumpkinStem { .. }
                | Block::Rail { .. }
                | Block::RedMushroom
                | Block::Sapling { .. }
                | Block::SeaPickle { .. }
                | Block::Seagrass { .. }
                | Block::SoulCampfire { .. }
                | Block::SoulFire { .. }
                | Block::StructureVoid
                | Block::SugarCane { .. }
                | Block::SweetBerryBush { .. }
                | Block::TurtleEgg { .. }
                | Block::TwistingVines { .. }
                | Block::TwistingVinesPlant
                | Block::Vines(_)
                | Block::WarpedFungus
                | Block::WeepingVines { .. }
                | Block::WeepingVinesPlant
                | Block::Wheat { .. }
                | Block::JigsawBlock { .. }
                | Block::StructureBlock
        )
    }
}
