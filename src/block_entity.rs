use std::collections::HashMap;

use crate::banner;
use crate::colour::Colour;
use crate::coordinates::BlockCoord;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::nbt_lookup::*;
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
        custom_name: Option<String>,
        patterns: Vec<banner::ColouredPattern>,
    },
    Barrel {
        tags: ChestTags,
    },
    Beacon {
        common: CommonTags,
        lock: Option<String>,
        levels: i32, // TODO change type to integer with valid range
        primary: StatusEffect,
        secondary: StatusEffect,
    },
    Bed {
        common: CommonTags,
        colour: Colour,
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
        fuel: i8, // TODO change to integer with valid range
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
    Furnace {
        tags: FurnaceTags,
    },
    Hopper {
        tags: ChestTags,
    },
    Jukebox {
        common: CommonTags,
        record: Item,
    },
    Lectern {
        common: CommonTags,
        book: Option<(Item, i32)>, // (book, page)
    },
    MobSpawner,
    Piston,
    ShulkerBox {
        tags: ChestTags,
    },
    Sign {
        common: CommonTags,
        colour: Colour,
        text: Vec<String>, // NB: Four strings, format is "compound object" as "JSON text".
    },
    Skull,
    Smoker {
        tags: FurnaceTags,
    },
    StructureBlock,
}

impl BlockEntity {
    pub fn map_from_nbt_list(list: &nbt::Value) -> HashMap<BlockCoord, Self> {
        Self::vec_from_nbt_list(&list)
            .iter()
            .map(|entity| (entity.coordinates(), entity))
            .filter(|(coord, _entity)| coord.is_some())
            .map(|(coord, entity)| (coord.unwrap(), entity.clone()))
            .collect()
    }

    pub fn vec_from_nbt_list(list: &nbt::Value) -> Vec<Self> {
        if let nbt::Value::List(block_entities) = list {
            block_entities
                .iter()
                .map(|nbt| BlockEntity::from_nbt_value(nbt))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn from_nbt_value(value: &nbt::Value) -> Self {
        if let Some(id) = nbt_value_lookup_string(&value, "id") {
            match id.as_str() {
                "minecraft:banner" => Self::banner_from_nbt_value(&value),
                "minecraft:barrel" => Self::barrel_from_nbt_value(&value),
                "minecraft:beacon" => Self::beacon_from_nbt_value(&value),
                "minecraft:bed" => Self::bed_from_nbt_value(&value),
                "minecraft:beehive" => Self::beehive_from_nbt_value(&value),
                "minecraft:bell" => Self::bell_from_nbt_value(&value),
                "minecraft:blast_furnace" => Self::blast_furnace_from_nbt_value(&value),
                "minecraft:brewing_stand" => Self::brewing_stand_from_nbt_value(&value),
                "minecraft:campfire" => Self::campfire_from_nbt_value(&value),
                "minecraft:chest" => Self::chest_from_nbt_value(&value),
                "minecraft:comparator" => Self::comparator_from_nbt_value(&value),
                "minecraft:command_block" => Self::command_block_from_nbt_value(&value),
                "minecraft:conduit" => Self::conduit_from_nbt_value(&value),
                "minecraft:daylight_detector" => Self::daylight_detector_from_nbt_value(&value),
                "minecraft:dispenser" => Self::dispenser_from_nbt_value(&value),
                "minecraft:dropper" => Self::dropper_from_nbt_value(&value),
                "minecraft:enchanting_table" => Self::enchanting_table_from_nbt_value(&value),
                "minecraft:ender_chest" => Self::ender_chest_from_nbt_value(&value),
                "minecraft:end_gateway" => Self::end_gateway_from_nbt_value(&value),
                "minecraft:end_portal" => Self::end_portal_from_nbt_value(&value),
                "minecraft:furnace" => Self::furnace_from_nbt_value(&value),
                "minecraft:hopper" => Self::hopper_from_nbt_value(&value),
                "minecraft:jigsaw" => Self::jigsaw_from_nbt_value(&value),
                "minecraft:jukebox" => Self::jukebox_from_nbt_value(&value),
                "minecraft:lectern" => Self::lectern_from_nbt_value(&value),
                "minecraft:mob_spawner" => Self::mob_spawner_from_nbt_value(&value),
                "minecraft:piston" => Self::piston_from_nbt_value(&value),
                "minecraft:shulker_box" => Self::shulker_box_from_nbt_value(&value),
                "minecraft:sign" => Self::sign_from_nbt_value(&value),
                "minecraft:skull" => Self::skull_from_nbt_value(&value),
                "minecraft:smoker" => Self::smoker_from_nbt_value(&value),
                "minecraft:soul_campfire" => Self::soul_campfire_from_nbt_value(&value),
                "minecraft:structure_block" => Self::structure_block_from_nbt_value(&value),
                _ => BlockEntity::Unknown { id: Some(id) },
            }
        } else {
            BlockEntity::Unknown { id: None }
        }
    }

    fn banner_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Banner {
            common: CommonTags::from_nbt_value(&value),
            custom_name: nbt_value_lookup_string(&value, "CustomName"),
            patterns: Vec::new(), // TODO actually parse and fill patterns
        }
    }

    fn barrel_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Barrel {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn beacon_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Beacon {
            common: CommonTags::from_nbt_value(&value),
            lock: nbt_value_lookup_string(&value, "Lock"),
            levels: nbt_value_lookup_int(&value, "Levels").unwrap(),
            primary: StatusEffect::from(nbt_value_lookup_int(&value, "Primary").unwrap()),
            secondary: StatusEffect::from(nbt_value_lookup_int(&value, "Sekondary").unwrap()),
        }
    }

    fn bed_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Bed {
            common: CommonTags::from_nbt_value(&value),
            colour: Colour::from(nbt_value_lookup_int(&value, "color").unwrap()),
        }
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn beehive_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown { id: nbt_value_lookup_string(&value, "id") }
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn bell_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown { id: nbt_value_lookup_string(&value, "id") }
    }

    fn blast_furnace_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::BlastFurnace {
            tags: FurnaceTags::from_nbt_value(&value),
        }
    }

    fn brewing_stand_from_nbt_value(value: &nbt::Value) -> Self {
        let items = if let Some(items) = nbt_value_lookup_list(&value, "Items") {
            Inventory::from_nbt_value_vec(&items)
        } else {
            Inventory::new()
        };

        BlockEntity::BrewingStand {
            common: CommonTags::from_nbt_value(&value),
            custom_name: nbt_value_lookup_string(&value, "CustomName"),
            lock: nbt_value_lookup_string(&value, "Lock"),
            items,
            brew_time: nbt_value_lookup_short(&value, "BrewTime").unwrap(),
            fuel: nbt_value_lookup_byte(&value, "Fuel").unwrap(),
        }
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn campfire_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown { id: nbt_value_lookup_string(&value, "id") }
    }

    fn chest_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Chest {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn comparator_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Comparator {
            common: CommonTags::from_nbt_value(&value),
            output_signal: nbt_value_lookup_int(&value, "OutputSignal").unwrap(),
        }
    }

    fn command_block_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::CommandBlock
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn conduit_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown { id: nbt_value_lookup_string(&value, "id") }
    }

    fn daylight_detector_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::DaylightDetector {
            common: CommonTags::from_nbt_value(&value),
        }
    }

    fn dispenser_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Dispenser {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn dropper_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Dropper {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn enchanting_table_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EnchantingTable {
            common: CommonTags::from_nbt_value(&value),
            custom_name: nbt_value_lookup_string(&value, "CustomName"),
        }
    }

    fn ender_chest_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EnderChest {
            common: CommonTags::from_nbt_value(&value),
        }
    }

    fn end_gateway_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EndGateway {
            common: CommonTags::from_nbt_value(&value),
            age: nbt_value_lookup_long(&value, "Age").unwrap(),
            exact_teleport: match nbt_value_lookup_byte(&value, "ExactTeleport") {
                Some(0) => false,
                Some(1) => true,
                Some(n) => panic!("Unknown ExactTeleport value of {}", n),
                None => panic!("ExactTeleport nbt value not found"),
            },
            exit_portal: (
                nbt_value_lookup_int(&value, "ExitPortal/X").unwrap() as i64,
                nbt_value_lookup_int(&value, "ExitPortal/Y").unwrap() as i64,
                nbt_value_lookup_int(&value, "ExitPortal/Z").unwrap() as i64,
            ).into(),
        }
    }

    fn end_portal_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::EndPortal {
            common: CommonTags::from_nbt_value(&value),
        }
    }

    fn furnace_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Furnace {
            tags: FurnaceTags::from_nbt_value(&value),
        }
    }

    fn hopper_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Hopper {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn jigsaw_from_nbt_value(value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::Unknown { id: nbt_value_lookup_string(&value, "id") }
    }

    fn jukebox_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Jukebox {
            common: CommonTags::from_nbt_value(&value),
            record: Item::from_nbt_value(&nbt_value_lookup(&value, "RecordItem").unwrap()),
        }
    }

    fn lectern_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Lectern {
            common: CommonTags::from_nbt_value(&value),
            book: if let Some(book_value) = nbt_value_lookup(&value, "Book") {
                Some(( Item::from_nbt_value(&book_value),
                    nbt_value_lookup_int(&value, "Page").unwrap()))
            } else {
                None
            }
        }
    }

    fn mob_spawner_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::MobSpawner
    }

    fn piston_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::Piston
    }

    fn shulker_box_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::ShulkerBox {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn sign_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Sign {
            common: CommonTags::from_nbt_value(&value),
            colour: if let Some(colour) = nbt_value_lookup_string(&value, "Color") {
                    Colour::from(colour.as_str())
            } else {
                    Colour::Black
            },
            // TODO handle text in a better manner than to expose "compound object" JSON
            text: vec!(
                nbt_value_lookup_string(&value, "Text1").unwrap_or_default(),
                nbt_value_lookup_string(&value, "Text2").unwrap_or_default(),
                nbt_value_lookup_string(&value, "Text3").unwrap_or_default(),
                nbt_value_lookup_string(&value, "Text4").unwrap_or_default(),
            ),
        }
    }

    fn skull_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::Skull
    }

    fn smoker_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Smoker {
            tags: FurnaceTags::from_nbt_value(&value),
        }
    }

    // TODO (deferred as not present in Minecraft 1.12.2)
    fn soul_campfire_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Unknown { id: nbt_value_lookup_string(&value, "id") }
    }

    fn structure_block_from_nbt_value(_value: &nbt::Value) -> Self {
        // TODO (deferred as too complicated)
        BlockEntity::StructureBlock
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
            Self::Furnace { tags } => Some(tags.common.coordinates()),
            Self::Hopper { tags } => Some(tags.common.coordinates()),
            Self::Jukebox { common, .. } => Some(common.coordinates()),
            Self::Lectern { common, .. } => Some(common.coordinates()),
            Self::MobSpawner => None,
            Self::Piston => None,
            Self::ShulkerBox { tags } => Some(tags.common.coordinates()),
            Self::Sign { common, .. } => Some(common.coordinates()),
            Self::Skull => None,
            Self::Smoker { tags } => Some(tags.common.coordinates()),
            Self::StructureBlock => None,
        }
    }
}

// Tags present for all block entities.
#[derive(Clone, Debug)]
pub struct CommonTags {
    id: String, // block entity ID
    x: i32, // chunk local x coordinate
    y: i32, // chunk local y coordinate
    z: i32, // chunk local z coordinate
    keep_packed: bool, // 1 indicates invalidated block entity
}

impl CommonTags {
    fn from_nbt_value(value: &nbt::Value) -> Self {
        Self {
            id: nbt_value_lookup_string(&value, "id").unwrap(),
            x: nbt_value_lookup_int(&value, "x").unwrap(),
            y: nbt_value_lookup_int(&value, "y").unwrap(),
            z: nbt_value_lookup_int(&value, "z").unwrap(),
            keep_packed: nbt_value_lookup_byte(&value, "keepPacked").unwrap_or(0) != 0,
        }
    }

    fn coordinates(&self) -> BlockCoord {
        (self.x as i64, self.y as i64, self.z as i64).into()
    }
}

// Tags present for all "chest similar" block entities, e.g. Chest, Dropper, etc.
#[derive(Clone, Debug)]
pub struct ChestTags {
        common: CommonTags,
        custom_name: Option<String>,
        lock: Option<String>,
        items: Inventory,
        loot_table: Option<()>, // TODO support for loot tables
        loot_table_seed: Option<()>, // TODO support for loot tables
}

impl ChestTags {
    fn from_nbt_value(value: &nbt::Value) -> Self {
        //chest_tags_from_nbt_value!(value)
        Self {
            common: CommonTags::from_nbt_value(&value),
            custom_name: nbt_value_lookup_string(&value, "CustomName"),
            lock: nbt_value_lookup_string(&value, "Lock"),
            items: if let Some(items) = nbt_value_lookup_list(&value, "Items") {
                Inventory::from_nbt_value_vec(&items)
            } else {
                Inventory::new()
            },
            loot_table: None, // TODO
            loot_table_seed: None, // TODO
        }
    }
}

// Tags present for all "furnace similar" block entities, e.g. Furnace, Smoker, etc.
#[derive(Clone, Debug)]
pub struct FurnaceTags {
    common: CommonTags,
    custom_name: Option<String>,
    lock: Option<String>,
    items: Inventory,
    burn_time: i16,
    cook_time: i16,
    cook_time_total: i16,
    // TODO Add structure for recipes for which XP is not collected yet..
}

impl FurnaceTags {
    fn from_nbt_value(value: &nbt::Value) -> Self {
        let items = if let Some(items) = nbt_value_lookup_list(&value, "Items") {
            Inventory::from_nbt_value_vec(&items)
        } else {
            Inventory::new()
        };

        Self {
            common: CommonTags::from_nbt_value(&value),
            custom_name: nbt_value_lookup_string(&value, "CustomName"),
            lock: nbt_value_lookup_string(&value, "Lock"),
            items,
            burn_time: nbt_value_lookup_short(&value, "BurnTime").unwrap(),
            cook_time: nbt_value_lookup_short(&value, "CookTime").unwrap(),
            cook_time_total: nbt_value_lookup_short(&value, "CookTimeTotal").unwrap(),
        }
    }
}

// All block entities have a root TAG_Compound that contains the common tags
// in addition to any specific tags.
