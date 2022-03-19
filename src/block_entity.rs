use std::collections::HashMap;

use crate::block::{
    BannerPattern, ColouredPattern, Flower, Grass, HeadVariant, Hinge, Pitch, PottedPlant,
};
use crate::colour::Colour;
use crate::coordinates::BlockCoord;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::mc_version::{McVersion, THE_FLATTENING};
use crate::nbt_lookup::*;
use crate::positioning::{Direction16, Surface4};
use crate::status_effect::StatusEffect;

// Block entity, aka "tile entity". Contains additional block data, a bit
// analogous to storing variable size data in a heap.
#[derive(Clone, Debug)]
pub enum BlockEntity {
    Unknown {
        id: Option<String>,
    },
    Banner {
        common: CommonTags,
        colour: Colour,
        custom_name: Option<String>,
        patterns: Vec<ColouredPattern>,
    },
    Barrel {
        tags: ChestTags,
    },
    Beacon {
        common: CommonTags,
        lock: Option<String>,
        levels: i32, // TODO change type to integer with valid range
        primary: Option<StatusEffect>,
        secondary: Option<StatusEffect>,
    },
    Bed {
        common: CommonTags,
        colour: Option<Colour>,
    },
    BlastFurnace {
        tags: FurnaceTags,
    },
    BrewingStand {
        common: CommonTags,
        custom_name: Option<String>,
        lock: Option<String>,
        items: Inventory,
        brew_time: i16, // TODO change to integer with valid range
        fuel: i8,       // TODO change to integer with valid range
    },
    Chest {
        tags: ChestTags,
    },
    Comparator {
        common: CommonTags,
        output_signal: i32,
    },
    CommandBlock,
    DaylightDetector {
        common: CommonTags,
    },
    Dispenser {
        tags: ChestTags,
    },
    Dropper {
        tags: ChestTags,
    },
    EnchantingTable {
        common: CommonTags,
        custom_name: Option<String>,
    },
    EnderChest {
        common: CommonTags,
    },
    EndGateway {
        common: CommonTags,
        age: i64,
        exact_teleport: bool,
        exit_portal: BlockCoord,
    },
    EndPortal {
        common: CommonTags,
    },
    FlowerPot {
        common: CommonTags,
        plant: Option<PottedPlant>,
    },
    Furnace {
        tags: FurnaceTags,
    },
    Hopper {
        tags: ChestTags,
    },
    Jukebox {
        common: CommonTags,
        record: Option<Item>,
    },
    Lectern {
        common: CommonTags,
        book: Option<(Item, i32)>, // (book, page)
    },
    MobSpawner,
    Noteblock {
        common: CommonTags,
        note: Pitch,
        powered: bool,
    },
    Piston,
    ShulkerBox {
        tags: ChestTags,
    },
    Sign {
        common: CommonTags,
        colour: Colour,
        text: Vec<String>, // NB: Four strings, format is "compound object" as "JSON text".
    },
    Skull {
        common: CommonTags,
        skull_type: HeadVariant,
        facing: Direction16,
    },
    Smoker {
        tags: FurnaceTags,
    },
    StructureBlock,
    TrappedChest {
        tags: ChestTags,
    },
    /// "Pseudo" variants are not found in game save files.
    /// They are internal to mcprogedit, and used for storing parameters from
    /// multiblock structures during world loading.
    PseudoDoorBottom {
        open: bool,
        facing: Surface4,
    },
    PseudoDoorTop {
        hinge: Hinge,
    },
    PseudoFlowerBottom(Flower),
    PseudoGrassBottom(Grass),
}

impl BlockEntity {
    pub fn map_from_nbt_list(list: &nbt::Value, data_version: McVersion) -> HashMap<BlockCoord, Self> {
        Self::vec_from_nbt_list(list, data_version)
            .iter()
            .map(|entity| (entity.coordinates(), entity))
            .filter(|(coord, _entity)| coord.is_some())
            .map(|(coord, entity)| (coord.unwrap(), entity.clone()))
            .collect()
    }

    pub fn vec_from_nbt_list(list: &nbt::Value, data_version: McVersion) -> Vec<Self> {
        if let nbt::Value::List(block_entities) = list {
            block_entities
                .iter()
                .map(|nbt_value| BlockEntity::from_nbt_value(nbt_value, data_version))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn from_nbt_value(value: &nbt::Value, data_version: McVersion) -> Self {
        if let Ok(id) = nbt_value_lookup_string(value, "id") {
            match id.as_str() {
                "minecraft:banner" => Self::banner_from_nbt_value(value),
                "minecraft:barrel" => Self::barrel_from_nbt_value(value),
                "minecraft:beacon" => Self::beacon_from_nbt_value(value),
                "minecraft:bed" => Self::bed_from_nbt_value(value),
                "minecraft:beehive" => Self::beehive_from_nbt_value(value),
                "minecraft:bell" => Self::bell_from_nbt_value(value),
                "minecraft:blast_furnace" => Self::blast_furnace_from_nbt_value(value),
                "minecraft:brewing_stand" => Self::brewing_stand_from_nbt_value(value),
                "minecraft:campfire" => Self::campfire_from_nbt_value(value),
                "minecraft:chest" => Self::chest_from_nbt_value(value),
                "minecraft:comparator" => Self::comparator_from_nbt_value(value),
                "minecraft:command_block" => Self::command_block_from_nbt_value(value),
                "minecraft:conduit" => Self::conduit_from_nbt_value(value),
                "minecraft:daylight_detector" => Self::daylight_detector_from_nbt_value(value),
                "minecraft:dispenser" => Self::dispenser_from_nbt_value(value),
                "minecraft:dropper" => Self::dropper_from_nbt_value(value),
                "minecraft:enchanting_table" => Self::enchanting_table_from_nbt_value(value),
                "minecraft:ender_chest" => Self::ender_chest_from_nbt_value(value),
                "minecraft:end_gateway" => Self::end_gateway_from_nbt_value(value),
                "minecraft:end_portal" => Self::end_portal_from_nbt_value(value),
                "minecraft:flower_pot" if data_version < THE_FLATTENING => Self::flower_pot_from_nbt_value(value),
                "minecraft:furnace" => Self::furnace_from_nbt_value(value),
                "minecraft:hopper" => Self::hopper_from_nbt_value(value),
                "minecraft:jigsaw" => Self::jigsaw_from_nbt_value(value),
                "minecraft:jukebox" => Self::jukebox_from_nbt_value(value),
                "minecraft:lectern" => Self::lectern_from_nbt_value(value),
                "minecraft:mob_spawner" => Self::mob_spawner_from_nbt_value(value),
                "minecraft:noteblock" => Self::noteblock_from_nbt_value(value),
                "minecraft:piston" => Self::piston_from_nbt_value(value),
                "minecraft:shulker_box" => Self::shulker_box_from_nbt_value(value),
                "minecraft:sign" => Self::sign_from_nbt_value(value),
                "minecraft:skull" if data_version < THE_FLATTENING => Self::skull_from_nbt_value(value),
                "minecraft:smoker" => Self::smoker_from_nbt_value(value),
                "minecraft:soul_campfire" => Self::soul_campfire_from_nbt_value(value),
                "minecraft:structure_block" => Self::structure_block_from_nbt_value(value),
                "minecraft:trapped_chest" => Self::trapped_chest_from_nbt_value(value),
                _ => {
                    eprintln!("Unknown tile entity ID: {}", id);
                    BlockEntity::Unknown { id: Some(id) }
                }
            }
        } else {
            BlockEntity::Unknown { id: None }
        }
    }

    pub fn to_nbt_value(&self) -> Option<nbt::Value> {
        match self {
            Self::Banner { .. } => self.banner_to_nbt_value(),
            Self::Barrel { .. } => self.barrel_to_nbt_value(),
            Self::Beacon { .. } => self.beacon_to_nbt_value(),
            Self::Bed { .. } => self.bed_to_nbt_value(),
            //Self::Beehive { .. } => self.beehive_to_nbt_value(),
            //Self::Bell { .. } => self.bell_to_nbt_value(),
            Self::BlastFurnace { .. } => self.blast_furnace_to_nbt_value(),
            Self::BrewingStand { .. } => self.brewing_stand_to_nbt_value(),
            //Self::Campfire { .. } => self.campfire_to_nbt_value(),
            Self::Chest { .. } => self.chest_to_nbt_value(),
            Self::Comparator { .. } => self.comparator_to_nbt_value(),
            Self::CommandBlock { .. } => self.command_block_to_nbt_value(),
            //Self::Conduit { .. } => self.conduit_to_nbt_value(),
            Self::DaylightDetector { .. } => self.daylight_detector_to_nbt_value(),
            Self::Dispenser { .. } => self.dispenser_to_nbt_value(),
            Self::Dropper { .. } => self.dropper_to_nbt_value(),
            Self::EnchantingTable { .. } => self.enchanting_table_to_nbt_value(),
            Self::EnderChest { .. } => self.ender_chest_to_nbt_value(),
            Self::EndGateway { .. } => self.end_gateway_to_nbt_value(),
            Self::EndPortal { .. } => self.end_portal_to_nbt_value(),
            Self::FlowerPot { .. } => self.flower_pot_to_nbt_value(),
            Self::Furnace { .. } => self.furnace_to_nbt_value(),
            Self::Hopper { .. } => self.hopper_to_nbt_value(),
            //Self::Jigsaw { .. } => self.jigsaw_to_nbt_value(),
            Self::Jukebox { .. } => self.jukebox_to_nbt_value(),
            Self::Lectern { .. } => self.lectern_to_nbt_value(),
            Self::MobSpawner { .. } => self.mob_spawner_to_nbt_value(),
            Self::Noteblock { .. } => self.noteblock_to_nbt_value(),
            Self::Piston { .. } => self.piston_to_nbt_value(),
            Self::ShulkerBox { .. } => self.shulker_box_to_nbt_value(),
            Self::Sign { .. } => self.sign_to_nbt_value(),
            Self::Skull { .. } => self.skull_to_nbt_value(),
            Self::Smoker { .. } => self.smoker_to_nbt_value(),
            //Self::SoulCampfire { .. } => self.soul_campfire_to_nbt_value(),
            Self::StructureBlock { .. } => self.structure_block_to_nbt_value(),
            Self::TrappedChest { .. } => self.trapped_chest_to_nbt_value(),

            // TODO add more block entity types
            _ => None,
        }
    }

    fn banner_from_nbt_value(value: &nbt::Value) -> Self {
        let mut patterns = Vec::new();

        if let Ok(pattern_entries) = nbt_value_lookup_list(value, "Patterns") {
            for pattern_entry in pattern_entries {
                let pattern = ColouredPattern {
                    colour: Colour::from(nbt_value_lookup_int(&pattern_entry, "Color").unwrap()),
                    pattern: BannerPattern::from(
                        nbt_value_lookup_string(&pattern_entry, "Pattern")
                            .unwrap()
                            .as_str(),
                    ),
                };
                patterns.push(pattern);
            }
        }

        BlockEntity::Banner {
            common: CommonTags::from_nbt_value(value),
            colour: if let Ok(colour) = nbt_value_lookup_string(value, "Color") {
                Colour::from(colour.as_str())
            } else if let Ok(colour) = nbt_value_lookup_int(value, "Base") {
                Colour::from(15 - colour)
            } else {
                Colour::White
            },
            custom_name: nbt_value_lookup_string(value, "CustomName").ok(),
            patterns,
        }
    }

    fn banner_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 2);
        if let Self::Banner {
            common,
            colour,
            custom_name,
            patterns: _,
        } = self
        {
            for (key, value) in common.to_nbt_values() {
                entity.insert(key, value);
            }

            entity.insert(
                "Base".into(),
                nbt::Value::Int((15 - u8::from(*colour)).into()),
            );

            if let Some(name) = custom_name {
                entity.insert("CustomName".into(), nbt::Value::String(name.clone()));
            }

            // TODO Add all TAG_Compound("Color", "Pattern") to the patterns list
            entity.insert("Patterns".into(), nbt::Value::List(Vec::new()));

            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn barrel_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Barrel {
            tags: ChestTags::from_nbt_value(value),
        }
    }

    fn barrel_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn beacon_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Beacon {
            common: CommonTags::from_nbt_value(value),
            lock: nbt_value_lookup_string(value, "Lock").ok(),
            levels: nbt_value_lookup_int(value, "Levels").unwrap(),
            primary: nbt_value_lookup_int(value, "Primary")
                .ok()
                .filter(|i| *i != 0)
                .map(StatusEffect::from),
            secondary: nbt_value_lookup_int(value, "Secondary")
                .ok()
                .filter(|i| *i != 0)
                .map(StatusEffect::from),
        }
    }

    fn beacon_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 4);
        if let Self::Beacon {
            common,
            lock,
            levels,
            primary,
            secondary,
        } = self
        {
            // Common tags
            for (key, value) in common.to_nbt_values() {
                entity.insert(key, value);
            }

            if let Some(lock) = lock {
                entity.insert("Lock".into(), nbt::Value::String(lock.clone()));
            }

            entity.insert("Levels".into(), nbt::Value::Int(*levels));

            entity.insert(
                "Primary".into(),
                nbt::Value::Int(primary.map_or(0, |effect| effect.into())),
            );

            entity.insert(
                "Secondary".into(),
                nbt::Value::Int(secondary.map_or(0, |effect| effect.into())),
            );

            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn bed_from_nbt_value(value: &nbt::Value) -> Self {
        let colour = match nbt_value_lookup_int(value, "color") {
            Ok(colour) => Some(Colour::from(colour)),
            Err(_) => None,
        };
        BlockEntity::Bed {
            common: CommonTags::from_nbt_value(value),
            colour,
        }
    }

    fn bed_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn beehive_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown {
            id: nbt_value_lookup_string(value, "id").ok(),
        }
    }

    fn _beehive_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn bell_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown {
            id: nbt_value_lookup_string(value, "id").ok(),
        }
    }

    fn _bell_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn blast_furnace_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::BlastFurnace {
            tags: FurnaceTags::from_nbt_value(value),
        }
    }

    fn blast_furnace_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn brewing_stand_from_nbt_value(value: &nbt::Value) -> Self {
        let items = if let Ok(items) = nbt_value_lookup_list(value, "Items") {
            Inventory::from_nbt_value_vec(&items)
        } else {
            Inventory::new()
        };

        BlockEntity::BrewingStand {
            common: CommonTags::from_nbt_value(value),
            custom_name: nbt_value_lookup_string(value, "CustomName").ok(),
            lock: nbt_value_lookup_string(value, "Lock").ok(),
            items,
            brew_time: nbt_value_lookup_short(value, "BrewTime").unwrap(),
            fuel: nbt_value_lookup_byte(value, "Fuel").unwrap(),
        }
    }

    fn brewing_stand_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn campfire_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown {
            id: nbt_value_lookup_string(value, "id").ok(),
        }
    }

    fn _campfire_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn chest_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Chest {
            tags: ChestTags::from_nbt_value(value),
        }
    }

    fn chest_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::Chest { tags } = self {
            for (key, value) in tags.to_nbt_values() {
                entity.insert(key, value);
            }
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn comparator_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Comparator {
            common: CommonTags::from_nbt_value(value),
            output_signal: nbt_value_lookup_int(value, "OutputSignal").unwrap(),
        }
    }

    fn comparator_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn command_block_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::CommandBlock
    }

    fn command_block_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn conduit_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown {
            id: nbt_value_lookup_string(value, "id").ok(),
        }
    }

    fn _conduit_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn daylight_detector_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::DaylightDetector {
            common: CommonTags::from_nbt_value(value),
        }
    }

    fn daylight_detector_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn dispenser_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Dispenser {
            tags: ChestTags::from_nbt_value(value),
        }
    }

    fn dispenser_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::Dispenser { tags } = self {
            for (key, value) in tags.to_nbt_values() {
                entity.insert(key, value);
            }
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn dropper_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Dropper {
            tags: ChestTags::from_nbt_value(value),
        }
    }

    fn dropper_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::Dropper { tags } = self {
            for (key, value) in tags.to_nbt_values() {
                entity.insert(key, value);
            }
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn enchanting_table_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EnchantingTable {
            common: CommonTags::from_nbt_value(value),
            custom_name: nbt_value_lookup_string(value, "CustomName").ok(),
        }
    }

    fn enchanting_table_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn ender_chest_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EnderChest {
            common: CommonTags::from_nbt_value(value),
        }
    }

    fn ender_chest_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn end_gateway_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EndGateway {
            common: CommonTags::from_nbt_value(value),
            age: nbt_value_lookup_long(value, "Age").unwrap(),
            exact_teleport: match nbt_value_lookup_byte(value, "ExactTeleport") {
                Ok(0) => false,
                Ok(1) => true,
                Ok(n) => panic!("Unknown ExactTeleport value of {}", n),
                Err(_) => panic!("ExactTeleport nbt value not found"),
            },
            exit_portal: (
                nbt_value_lookup_int(value, "ExitPortal/X").unwrap() as i64,
                nbt_value_lookup_int(value, "ExitPortal/Y").unwrap() as i64,
                nbt_value_lookup_int(value, "ExitPortal/Z").unwrap() as i64,
            )
                .into(),
        }
    }

    fn end_gateway_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn end_portal_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EndPortal {
            common: CommonTags::from_nbt_value(value),
        }
    }

    fn end_portal_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    /*
        if let Some(id) = nbt_value_lookup_string(&value, "id") {
            match id.as_str() {
    */

    fn flower_pot_from_nbt_value(value: &nbt::Value) -> Self {
        let (group, plant) = (
            nbt_value_lookup_string(value, "Item").unwrap(),
            nbt_value_lookup_int(value, "Data").unwrap(),
        );
        BlockEntity::FlowerPot {
            common: CommonTags::from_nbt_value(value),
            plant: match (group.as_str(), plant) {
                ("minecraft:air", _) => None,
                ("minecraft:brown_mushroom", _) => Some(PottedPlant::BrownMushroom),
                ("minecraft:cactus", _) => Some(PottedPlant::Cactus),
                ("minecraft:deadbush", _) => Some(PottedPlant::DeadBush),
                ("minecraft:red_flower", 0) => Some(PottedPlant::Poppy),
                ("minecraft:red_flower", 1) => Some(PottedPlant::BlueOrchid),
                ("minecraft:red_flower", 2) => Some(PottedPlant::Allium),
                ("minecraft:red_flower", 3) => Some(PottedPlant::AzureBluet),
                ("minecraft:red_flower", 4) => Some(PottedPlant::TulipRed),
                ("minecraft:red_flower", 5) => Some(PottedPlant::TulipOrange),
                ("minecraft:red_flower", 6) => Some(PottedPlant::TulipWhite),
                ("minecraft:red_flower", 7) => Some(PottedPlant::TulipPink),
                ("minecraft:red_flower", 8) => Some(PottedPlant::OxeyeDaisy),
                ("minecraft:red_mushroom", _) => Some(PottedPlant::RedMushroom),
                ("minecraft:sapling", 0) => Some(PottedPlant::OakSapling),
                ("minecraft:sapling", 1) => Some(PottedPlant::SpruceSapling),
                ("minecraft:sapling", 2) => Some(PottedPlant::BirchSapling),
                ("minecraft:sapling", 3) => Some(PottedPlant::JungleSapling),
                ("minecraft:sapling", 4) => Some(PottedPlant::AcaciaSapling),
                ("minecraft:sapling", 5) => Some(PottedPlant::DarkOakSapling),
                ("minecraft:tallgrass", 2) => Some(PottedPlant::Fern),
                ("minecraft:yellow_flower", _) => Some(PottedPlant::Dandelion),
                _ => {
                    eprintln!(
                        "Unknown flower pot tile entity: [Item={:?}, Data={:?}]",
                        group.as_str(),
                        plant
                    );
                    None
                }
            },
        }
    }

    fn flower_pot_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn furnace_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Furnace {
            tags: FurnaceTags::from_nbt_value(value),
        }
    }

    fn furnace_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::Furnace { tags } = self {
            for (key, value) in tags.to_nbt_values() {
                entity.insert(key, value);
            }
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn hopper_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Hopper {
            tags: ChestTags::from_nbt_value(value),
        }
    }

    fn hopper_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::Hopper { tags } = self {
            for (key, value) in tags.to_nbt_values() {
                entity.insert(key, value);
            }
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn jigsaw_from_nbt_value(value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::Unknown {
            id: nbt_value_lookup_string(value, "id").ok(),
        }
    }

    fn _jigsaw_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn jukebox_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Jukebox {
            common: CommonTags::from_nbt_value(value),
            record: nbt_value_lookup(value, "RecordItem").map(|value| Item::from_nbt_value(&value)).ok(),
        }
    }

    fn jukebox_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn lectern_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Lectern {
            common: CommonTags::from_nbt_value(value),
            book: nbt_value_lookup(value, "Book").map(|book_value| (
                    Item::from_nbt_value(&book_value),
                    nbt_value_lookup_int(value, "Page").unwrap(),
                )).ok(),
        }
    }

    fn lectern_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn mob_spawner_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::MobSpawner
    }

    fn mob_spawner_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn noteblock_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Noteblock {
            common: CommonTags::from_nbt_value(value),
            note: Pitch::from_value(nbt_value_lookup_byte(value, "note").unwrap() as u8),
            powered: !matches!(nbt_value_lookup_byte(value, "powered"), Ok(0)),
        }
    }

    fn noteblock_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 2);
        if let Self::Noteblock {
            common,
            note,
            powered,
        } = self
        {
            for (key, value) in common.to_nbt_values() {
                entity.insert(key, value);
            }
            entity.insert("note".into(), nbt::Value::Byte(note.to_i8()));
            let powered = if *powered { 1 } else { 0 };
            entity.insert("powered".into(), nbt::Value::Byte(powered));
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn piston_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::Piston
    }

    fn piston_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn shulker_box_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::ShulkerBox {
            tags: ChestTags::from_nbt_value(value),
        }
    }

    fn shulker_box_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::ShulkerBox { tags } = self {
            for (key, value) in tags.to_nbt_values() {
                entity.insert(key, value);
            }
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn sign_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Sign {
            common: CommonTags::from_nbt_value(value),
            colour: if let Ok(colour) = nbt_value_lookup_string(value, "Color") {
                Colour::from(colour.as_str())
            } else {
                Colour::Black
            },
            // TODO handle text in a better manner than to expose "compound object" JSON
            text: vec![
                nbt_value_lookup_string(value, "Text1").unwrap_or(r#"{"text":""}"#.into()),
                nbt_value_lookup_string(value, "Text2").unwrap_or(r#"{"text":""}"#.into()),
                nbt_value_lookup_string(value, "Text3").unwrap_or(r#"{"text":""}"#.into()),
                nbt_value_lookup_string(value, "Text4").unwrap_or(r#"{"text":""}"#.into()),
            ],
        }
    }

    fn sign_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::Sign { common, colour, text } = self {
            for (key, value) in common.to_nbt_values() {
                entity.insert(key, value);
            }
            entity.insert("Color".into(), nbt::Value::String(colour.to_string()));
            entity.insert("Text1".into(), nbt::Value::String(text[0].clone()));
            entity.insert("Text2".into(), nbt::Value::String(text[1].clone()));
            entity.insert("Text3".into(), nbt::Value::String(text[2].clone()));
            entity.insert("Text4".into(), nbt::Value::String(text[3].clone()));
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn skull_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Skull {
            common: CommonTags::from_nbt_value(value),
            skull_type: match nbt_value_lookup_byte(value, "SkullType").unwrap() {
                0 => HeadVariant::SkeletonSkull,
                1 => HeadVariant::WitherSkeletonSkull,
                2 => HeadVariant::ZombieHead,
                3 => HeadVariant::PlayerHead,
                4 => HeadVariant::CreeperHead,
                5 => HeadVariant::DragonHead,
                n => panic!("Unknown SkullType value of {}", n),
            },
            facing: Direction16::from(nbt_value_lookup_byte(value, "Rot").unwrap()).opposite(),
        }
    }

    fn skull_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn smoker_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Smoker {
            tags: FurnaceTags::from_nbt_value(value),
        }
    }

    fn smoker_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn soul_campfire_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown {
            id: nbt_value_lookup_string(value, "id").ok(),
        }
    }

    fn _soul_campfire_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn structure_block_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::StructureBlock
    }

    fn structure_block_to_nbt_value(&self) -> Option<nbt::Value> {
        // TODO
        unimplemented!()
    }

    fn trapped_chest_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::TrappedChest {
            tags: ChestTags::from_nbt_value(value),
        }
    }

    fn trapped_chest_to_nbt_value(&self) -> Option<nbt::Value> {
        let mut entity: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(5 + 5);

        if let Self::TrappedChest { tags } = self {
            for (key, value) in tags.to_nbt_values() {
                entity.insert(key, value);
            }
            Some(nbt::Value::Compound(entity))
        } else {
            None
        }
    }

    fn coordinates(&self) -> Option<BlockCoord> {
        match self {
            BlockEntity::Unknown { .. } => None,
            Self::Banner { common, .. } => Some(common.coordinates()),
            Self::Barrel { tags } => Some(tags.common.coordinates()),
            Self::Beacon { common, .. } => Some(common.coordinates()),
            Self::Bed { common, .. } => Some(common.coordinates()),
            Self::BlastFurnace { tags } => Some(tags.common.coordinates()),
            Self::BrewingStand { common, .. } => Some(common.coordinates()),
            Self::Chest { tags } => Some(tags.common.coordinates()),
            Self::Comparator { common, .. } => Some(common.coordinates()),
            Self::CommandBlock => None,
            Self::DaylightDetector { common } => Some(common.coordinates()),
            Self::Dispenser { tags } => Some(tags.common.coordinates()),
            Self::Dropper { tags } => Some(tags.common.coordinates()),
            Self::EnchantingTable { common, .. } => Some(common.coordinates()),
            Self::EnderChest { common } => Some(common.coordinates()),
            Self::EndGateway { common, .. } => Some(common.coordinates()),
            Self::EndPortal { common } => Some(common.coordinates()),
            Self::FlowerPot { common, .. } => Some(common.coordinates()),
            Self::Furnace { tags } => Some(tags.common.coordinates()),
            Self::Hopper { tags } => Some(tags.common.coordinates()),
            Self::Jukebox { common, .. } => Some(common.coordinates()),
            Self::Lectern { common, .. } => Some(common.coordinates()),
            Self::MobSpawner => None,
            Self::Noteblock { common, .. } => Some(common.coordinates()),
            Self::Piston => None,
            Self::ShulkerBox { tags } => Some(tags.common.coordinates()),
            Self::Sign { common, .. } => Some(common.coordinates()),
            Self::Skull { common, .. } => Some(common.coordinates()),
            Self::Smoker { tags } => Some(tags.common.coordinates()),
            Self::StructureBlock => None,
            Self::TrappedChest { tags } => Some(tags.common.coordinates()),
            // Internal mcprogedit block entities do not contain x, y, z tags
            Self::PseudoDoorBottom { .. }
            | Self::PseudoDoorTop { .. }
            | Self::PseudoFlowerBottom(_)
            | Self::PseudoGrassBottom(_) => None,
        }
    }
}

// Tags present for all block entities.
#[derive(Clone, Debug)]
pub struct CommonTags {
    pub(crate) id: String,        // block entity ID
    pub(crate) x: i32,            // chunk local x coordinate
    pub(crate) y: i32,            // chunk local y coordinate
    pub(crate) z: i32,            // chunk local z coordinate
    pub(crate) keep_packed: bool, // 1 indicates invalidated block entity
}

impl CommonTags {
    fn from_nbt_value(value: &nbt::Value) -> Self {
        Self {
            id: nbt_value_lookup_string(value, "id").unwrap(),
            x: nbt_value_lookup_int(value, "x").unwrap(),
            y: nbt_value_lookup_int(value, "y").unwrap(),
            z: nbt_value_lookup_int(value, "z").unwrap(),
            keep_packed: nbt_value_lookup_byte(value, "keepPacked").unwrap_or(0) != 0,
        }
    }

    fn to_nbt_values(&self) -> Vec<(String, nbt::Value)> {
        let mut nbt_values = Vec::with_capacity(5);
        nbt_values.push(("id".into(), nbt::Value::String(self.id.clone())));
        nbt_values.push(("x".into(), nbt::Value::Int(self.x)));
        nbt_values.push(("y".into(), nbt::Value::Int(self.y)));
        nbt_values.push(("z".into(), nbt::Value::Int(self.z)));
        let keep_packed = if self.keep_packed { 1 } else { 0 };
        nbt_values.push(("keepPacked".into(), nbt::Value::Byte(keep_packed)));
        nbt_values
    }

    fn coordinates(&self) -> BlockCoord {
        (self.x as i64, self.y as i64, self.z as i64).into()
    }
}

// Tags present for all "chest similar" block entities, e.g. Chest, Dropper, etc.
#[derive(Clone, Debug)]
pub struct ChestTags {
    pub(crate) common: CommonTags,
    pub(crate) custom_name: Option<String>,
    pub(crate) lock: Option<String>,
    pub(crate) items: Inventory,
    pub(crate) loot_table: Option<()>,      // TODO support for loot tables
    pub(crate) loot_table_seed: Option<()>, // TODO support for loot tables
}

impl ChestTags {
    fn from_nbt_value(value: &nbt::Value) -> Self {
        //chest_tags_from_nbt_value!(value)
        Self {
            common: CommonTags::from_nbt_value(value),
            custom_name: nbt_value_lookup_string(value, "CustomName").ok(),
            lock: nbt_value_lookup_string(value, "Lock").ok(),
            items: if let Ok(items) = nbt_value_lookup_list(value, "Items") {
                Inventory::from_nbt_value_vec(&items)
            } else {
                Inventory::new()
            },
            loot_table: None,      // TODO
            loot_table_seed: None, // TODO
        }
    }

    fn to_nbt_values(&self) -> Vec<(String, nbt::Value)> {
        let mut nbt_values = self.common.to_nbt_values();
        if let Some(name) = &self.custom_name {
            nbt_values.push(("CustomName".into(), nbt::Value::String(name.clone())));
        }
        if let Some(lock) = &self.lock {
            nbt_values.push(("Lock".into(), nbt::Value::String(lock.clone())));
        }
        nbt_values.push(("Items".into(), self.items.to_nbt_value()));
        // TODO support for loot tables
        // TODO support for loot tables
        nbt_values
    }
}

// Tags present for all "furnace similar" block entities, e.g. Furnace, Smoker, etc.
#[derive(Clone, Debug)]
pub struct FurnaceTags {
    pub(crate) common: CommonTags,
    pub(crate) custom_name: Option<String>,
    pub(crate) lock: Option<String>,
    pub(crate) items: Inventory,
    pub(crate) burn_time: i16,
    pub(crate) cook_time: i16,
    pub(crate) cook_time_total: i16,
    // TODO Add structure for recipes for which XP is not collected yet..
}

impl FurnaceTags {
    fn from_nbt_value(value: &nbt::Value) -> Self {
        let items = if let Ok(items) = nbt_value_lookup_list(value, "Items") {
            Inventory::from_nbt_value_vec(&items)
        } else {
            Inventory::new()
        };

        Self {
            common: CommonTags::from_nbt_value(value),
            custom_name: nbt_value_lookup_string(value, "CustomName").ok(),
            lock: nbt_value_lookup_string(value, "Lock").ok(),
            items,
            burn_time: nbt_value_lookup_short(value, "BurnTime").unwrap(),
            cook_time: nbt_value_lookup_short(value, "CookTime").unwrap(),
            cook_time_total: nbt_value_lookup_short(value, "CookTimeTotal").unwrap(),
        }
    }

    fn to_nbt_values(&self) -> Vec<(String, nbt::Value)> {
        let mut nbt_values = self.common.to_nbt_values();
        if let Some(name) = &self.custom_name {
            nbt_values.push(("CustomName".into(), nbt::Value::String(name.clone())));
        }
        if let Some(lock) = &self.lock {
            nbt_values.push(("Lock".into(), nbt::Value::String(lock.clone())));
        }
        nbt_values.push(("Items".into(), self.items.to_nbt_value()));
        nbt_values.push(("BurnTime".into(), nbt::Value::Short(self.burn_time)));
        nbt_values.push(("CookTime".into(), nbt::Value::Short(self.cook_time)));
        nbt_values.push(("CookTimeTotal".into(), nbt::Value::Short(self.cook_time_total)));
        // TODO Add compound RecipesUsed
        nbt_values
    }
}

// All block entities have a root TAG_Compound that contains the common tags
// in addition to any specific tags.
