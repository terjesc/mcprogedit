use crate::block::Block;

pub enum LightBlockingProperty {
    /// Transparent in some directions, Opaque in others
    Directional,
    /// Transparent to block light, dims sky light by one
    FilterSkyLight,
    /// Blocks light, in all directions
    Opaque,
    /// Lets all light through, in all directions
    Transparent,
    /// Light blocking properties not determined
    Unknown,
}

impl Block {
    pub fn is_affecting_sky_light(&self) -> bool {
        !matches!(self.light_blocking_properties(), LightBlockingProperty::Transparent)
    }

    pub fn light_blocking_properties(&self) -> LightBlockingProperty {
        match self {
            // Directional
            Block::Piston { .. }
            | Block::PistonHead { .. }
            | Block::DaylightDetector
            | Block::EnchantingTable { .. }
            | Block::EndPortalFrame { .. }
            | Block::Farmland { .. }
            | Block::GrassPath
            | Block::InvertedDaylightDetector
            | Block::Lectern { .. }
            | Block::Slab(_)
            | Block::Snow { .. }
            | Block::Stairs(_) => LightBlockingProperty::Directional,

            // FilterSkyLight
            // TODO Add all transparent waterlogged blocks
            Block::Beacon(_)
            | Block::BlockOfSlime
            | Block::BubbleColumn { .. }
            | Block::ChorusFlower { .. }
            | Block::ChorusPlant
            | Block::Cobweb
            | Block::EndGateway
            | Block::FrostedIce
            | Block::HoneyBlock
            | Block::Ice
            | Block::Lava { .. }
            | Block::LavaSource
            | Block::Leaves { .. }
            | Block::ShulkerBox(_)
            | Block::Spawner
            | Block::Water { .. }
            | Block::WaterSource => LightBlockingProperty::FilterSkyLight,

            // Opaque
            Block::AncientDebris
            | Block::Andesite
            | Block::Barrel { .. }
            | Block::Basalt { .. }
            | Block::Bedrock
            | Block::Beehive { .. }
            | Block::BeeNest { .. }
            | Block::Blackstone
            | Block::BlastFurnace(_)
            | Block::BlockOfCoal
            | Block::BlockOfDiamond
            | Block::BlockOfEmerald
            | Block::BlockOfGold
            | Block::BlockOfIron
            | Block::BlockOfNetherite
            | Block::BlockOfQuartz
            | Block::BlockOfRedstone
            | Block::BlueIce
            | Block::BoneBlock { .. }
            | Block::Bookshelf
            | Block::BrickBlock
            | Block::BrownMushroomBlock { .. }
            | Block::BrownMushroomStem { .. }
            | Block::Cactus { .. }
            | Block::CartographyTable
            | Block::CarvedPumpkin { .. }
            | Block::ChiseledNetherBricks
            | Block::ChiseledPolishedBlackstone
            | Block::ChiseledQuartzBlock
            | Block::ChiseledRedSandstone
            | Block::ChiseledSandstone
            | Block::ChiseledStoneBricks
            | Block::Clay
            | Block::CoalOre
            | Block::CoarseDirt
            | Block::Cobblestone
            | Block::CommandBlock(_)
            | Block::Concrete { .. }
            | Block::ConcretePowder { .. }
            | Block::CoralBlock { .. }
            | Block::CrackedNetherBricks
            | Block::CrackedPolishedBlackstoneBricks
            | Block::CrackedStoneBricks
            | Block::CraftingTable
            | Block::CrimsonNylium
            | Block::CryingObsidian
            | Block::CutRedSandstone
            | Block::CutSandstone
            | Block::DarkPrismarine
            | Block::DiamondOre
            | Block::Diorite
            | Block::Dirt
            | Block::Dispenser(_)
            | Block::DriedKelpBlock
            | Block::Dropper(_)
            | Block::EmeraldOre
            | Block::EndStone
            | Block::EndStoneBricks
            | Block::FletchingTable
            | Block::Furnace(_)
            | Block::GildedBlackstone
            | Block::GlazedTerracotta(_)
            | Block::GoldOre
            | Block::Granite
            | Block::GrassBlock
            | Block::Gravel
            | Block::HayBale { .. }
            | Block::HoneycombBlock
            | Block::InfestedChiseledStoneBricks
            | Block::InfestedCobblestone
            | Block::InfestedCrackedStoneBricks
            | Block::InfestedMossyStoneBricks
            | Block::InfestedStone
            | Block::InfestedStoneBricks
            | Block::IronOre
            | Block::JackOLantern { .. }
            | Block::Jukebox(_)
            | Block::LapisLazuliBlock
            | Block::LapisLazuliOre
            | Block::LodeStone
            | Block::Log(_)
            | Block::Loom { .. }
            | Block::MagmaBlock
            | Block::Melon
            | Block::MossyCobblestone
            | Block::MossyStoneBricks
            | Block::Mycelium
            | Block::NetherBricks
            | Block::NetherGoldOre
            | Block::NetherWartBlock
            | Block::Netherrack
            | Block::Noteblock(_)
            | Block::Observer { .. }
            | Block::Obsidian
            | Block::PackedIce
            | Block::Planks { .. }
            | Block::Podzol
            | Block::PolishedAndesite
            | Block::PolishedBasalt { .. }
            | Block::PolishedBlackstone
            | Block::PolishedBlackstoneBricks
            | Block::PolishedDiorite
            | Block::PolishedGranite
            | Block::Prismarine
            | Block::PrismarineBricks
            | Block::Pumpkin { .. }
            | Block::PurpurBlock
            | Block::PurpurPillar { .. }
            | Block::QuartzBricks
            | Block::QuartzOre
            | Block::QuartzPillar { .. }
            | Block::RedMushroomBlock { .. }
            | Block::RedMushroomStem { .. }
            | Block::RedNetherBricks
            | Block::RedSand
            | Block::RedSandstone
            | Block::RedstoneLamp // Transparent when lit?
            | Block::RedstoneOre
            | Block::RespawnAnchor { .. }
            | Block::Sand
            | Block::Sandstone
            | Block::SeaLantern
            | Block::Shroomlight
            | Block::SmithingTable
            | Block::Smoker { .. }
            | Block::SmoothQuartz
            | Block::SmoothRedSandstone
            | Block::SmoothSandstone
            | Block::SmoothStone
            | Block::SnowBlock
            | Block::SoulSoil
            | Block::Sponge
            | Block::Stone
            | Block::StoneBricks
            | Block::Target
            | Block::Terracotta { .. }
            | Block::TNT
            | Block::WarpedNyliym
            | Block::WarpedRoots
            | Block::WarpedWartBlock
            | Block::WetSponge
            | Block::Wool { .. } => LightBlockingProperty::Opaque,

            // Transparent
            Block::None
            | Block::Air
            | Block::Anvil { .. }
            | Block::Bamboo { .. }
            | Block::Banner(_)
            | Block::Barrier
            | Block::Beetroots { .. }
            | Block::Bell { .. }
            | Block::Bed(_)
            | Block::BrewingStand(_)
            | Block::BrownMushroom
            | Block::Button(_, _)
            | Block::Cake { .. }
            | Block::Campfire { .. }
            | Block::Carpet { .. }
            | Block::Carrots { .. }
            | Block::Cauldron { .. }
            | Block::CaveAir
            | Block::Chest(_)
            | Block::CocoaBeans { .. }
            | Block::Composter { .. }
            | Block::Conduit { .. }
            | Block::Coral { .. }
            | Block::CoralFan { .. }
            | Block::CrimsonFungus
            | Block::CrimsonRoots
            | Block::DeadBush
            | Block::Door(_)
            | Block::DragonEgg
            | Block::EndPortal
            | Block::EndRod { .. }
            | Block::EnderChest { .. }
            | Block::Fence { .. }
            | Block::FenceGate { .. }
            | Block::Fire { .. }
            | Block::Flower(_)
            | Block::FlowerPot(_)
            | Block::Glass { .. }
            | Block::GlassPane { .. }
            | Block::Glowstone
            | Block::Grass(_)
            | Block::GrindStone(_)
            | Block::Head(_)
            | Block::Hopper(_)
            | Block::IronBars { .. }
            | Block::Kelp { .. }
            | Block::Ladder { .. }
            | Block::Lantern { .. }
            | Block::Lever(_, _)
            | Block::LilyPad
            | Block::MelonStem { .. }
            | Block::NetherPortal { .. }
            | Block::NetherSprouts
            | Block::NetherWart { .. }
            | Block::Potatoes { .. }
            | Block::PressurePlate { .. }
            | Block::PumpkinStem { .. }
            | Block::Rail { .. }
            | Block::RedMushroom
            | Block::RedstoneComparator { .. }
            | Block::RedstoneRepeater(_)
            | Block::RedstoneSubtractor { .. }
            | Block::RedstoneTorch { .. }
            | Block::RedstoneWire
            | Block::Sapling { .. }
            | Block::Scaffolding { .. }
            | Block::SeaPickle { .. }
            | Block::Seagrass { .. }
            | Block::Sign(_)
            | Block::SoulCampfire { .. }
            | Block::SoulFire { .. }
            | Block::SoulLantern { .. }
            | Block::SoulTorch { .. }
            | Block::SoulSand
            | Block::StickyPiston { .. }
            | Block::StickyPistonHead { .. }
            | Block::StoneCutter { .. }
            | Block::StructureVoid
            | Block::SugarCane { .. }
            | Block::SweetBerryBush { .. }
            | Block::Torch { .. }
            | Block::Trapdoor(_)
            | Block::TrappedChest(_)
            | Block::Tripwire
            | Block::TripwireHook { .. }
            | Block::TurtleEgg { .. }
            | Block::TwistingVines { .. }
            | Block::TwistingVinesPlant
            | Block::Vines(_)
            | Block::Wall { .. }
            | Block::WarpedFungus
            | Block::WeepingVines { .. }
            | Block::WeepingVinesPlant
            | Block::Wheat { .. } => LightBlockingProperty::Transparent,

            // Unknown
            Block::Unknown(_)
            | Block::JigsawBlock { .. }
            | Block::StructureBlock => LightBlockingProperty::Unknown,
        }
    }
}
