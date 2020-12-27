use crate::banner;
use crate::colour::Colour;
use crate::inventory::Inventory;
use crate::nbt_lookup::*;

// Block entity, aka "tile entity". Contains additional block data, a bit
// analogous to storing variable size data in a heap.
#[derive(Debug)]
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
        primary: i32, // TODO change type to potion effect
        secondary: i32, // TODO change type to potion effect
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
    Comparator,
    CommandBlock,
    DaylightDetector,
    Dispenser {
        tags: ChestTags,
    },
    Dropper {
        tags: ChestTags,
    },
    EnchantingTable,
    EnderChest {
        common: CommonTags,
    },
    EndGateway,
    EndPortal,
    FlowerPot,
    Furnace {
        tags: FurnaceTags,
    },
    Hopper {
        tags: ChestTags,
    },
    Jukebox,
    MobSpawner,
    Noteblock,
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
    pub fn vec_from_nbt_list(list: &nbt::Value) -> Vec<Self> {
        if let nbt::Value::List(block_entities) = list {
            block_entities.iter().map(|nbt| BlockEntity::from_nbt_value(nbt)).collect()
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
                "minecraft:cauldron" => Self::cauldron_from_nbt_value(&value),
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

    fn beacon_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn bed_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn beehive_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn bell_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
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

    fn campfire_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn cauldron_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn chest_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Chest {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn comparator_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn command_block_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn conduit_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn daylight_detector_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
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

    fn enchanting_table_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn ender_chest_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn end_gateway_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn end_portal_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
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

    fn jigsaw_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn jukebox_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn lectern_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn mob_spawner_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn piston_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn shulker_box_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::ShulkerBox {
            tags: ChestTags::from_nbt_value(&value),
        }
    }

    fn sign_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Sign {
            common: CommonTags::from_nbt_value(&value),
            colour: {
                if let Some(colour) = nbt_value_lookup_string(&value, "Color") {
                    Colour::from(colour.as_str())
                } else {
                    Colour::Black
                }
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
        todo!();
    }

    fn smoker_from_nbt_value(value: &nbt::Value) -> Self {
        BlockEntity::Smoker {
            tags: FurnaceTags::from_nbt_value(&value),
        }
    }

    fn soul_campfire_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }

    fn structure_block_from_nbt_value(_value: &nbt::Value) -> Self {
        todo!();
    }
}

// Tags present for all block entities.
#[derive(Debug)]
pub struct CommonTags {
    id: String, // block entity ID
    x: i32, // chunk local x coordinate
    y: i32, // chunk local y coordinate
    z: i32, // chunk local z coordinate
    keep_packed: bool, // 1 indicates invalidated block entity
}

impl CommonTags {
    pub fn from_nbt_value(value: &nbt::Value) -> Self {
        Self {
            id: nbt_value_lookup_string(&value, "id").unwrap(),
            x: nbt_value_lookup_int(&value, "x").unwrap(),
            y: nbt_value_lookup_int(&value, "y").unwrap(),
            z: nbt_value_lookup_int(&value, "z").unwrap(),
            keep_packed: nbt_value_lookup_byte(&value, "keepPacked").unwrap_or(0) != 0,
        }
    }
}

// Tags present for all "chest similar" block entities, e.g. Chest, Dropper, etc.
#[derive(Debug)]
pub struct ChestTags {
        common: CommonTags,
        custom_name: Option<String>,
        lock: Option<String>,
        items: Inventory, // <slot, (count, item)>
        loot_table: Option<()>, // TODO support for loot tables
        loot_table_seed: Option<()>, // TODO support for loot tables
}

impl ChestTags {
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
            loot_table: None, // TODO
            loot_table_seed: None, // TODO
        }
    }
}

// Tags present for all "furnace similar" block entities, e.g. Furnace, Smoker, etc.
#[derive(Debug)]
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
