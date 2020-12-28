//! Items.

use crate::block::{AnvilDamage, Flower};
use crate::colour::Colour;
use crate::enchantment::Enchantment;
use crate::material::*;
use crate::nbt_lookup::*;

#[derive(Clone, Debug)]
pub struct Item {
    custom_name: Option<String>,
    enchantments: Vec<Enchantment>,
    damage: Option<i16>,
    kind: ItemKind,
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}

impl Item {
    pub fn new() -> Self {
        Self {
            custom_name: None,
            enchantments: Vec::new(),
            damage: None,
            kind: ItemKind::Apple,
        }
    }

    // Source: https://minecraft.gamepedia.com/Java_Edition_data_value/Pre-flattening#Item_IDs
    // Implemented block items
    // Next item to implement: pumpkin_pie (from beginning) or beetroot_soup (from end)
    // Missing: 
    // * lingering_potion
    // * potion
    // * splash_potion
    // * tipped_arrow
    // * spectral_arrow
    // * dragon_breath
    pub fn from_nbt_value(value: &nbt::Value) -> Self {
        let id = nbt_value_lookup_string(&value, "id").unwrap(); // mandatory
        let damage = nbt_value_lookup_short(&value, "Damage").unwrap_or(0);
        // TODO Some items must store data from a "tag" field as well.
        // let tag = nbt_value_lookup(&value, "tag"); // optional

        let kind = match id.as_str() {
            "minecraft:acacia_boat" => ItemKind::Boat(WoodMaterial::Acacia),
            "minecraft:acacia_door" => ItemKind::Door(DoorMaterial::Acacia),
            "minecraft:acacia_fence" => ItemKind::Fence(FenceMaterial::Acacia),
            "minecraft:acacia_fence_gate" => ItemKind::FenceGate(WoodMaterial::Acacia),
            "minecraft:acacia_stairs" => ItemKind::Stairs(StairMaterial::Acacia),
            "minecraft:activator_rail" => ItemKind::ActivatorRail,
            "minecraft:anvil" => ItemKind::Anvil(AnvilDamage::from(damage)),
            "minecraft:apple" => ItemKind::Apple,
            "minecraft:armor_stand" => ItemKind::ArmorStand,
            "minecraft:arrow" => ItemKind::Arrow,
            "minecraft:baked_potato" => ItemKind::BakedPotato,
            "minecraft:banner" => ItemKind::Banner,
            "minecraft:barrier" => ItemKind::Barrier,
            "minecraft:beacon" => ItemKind::Beacon,
            "minecraft:bed" => ItemKind::Bed(Colour::from(damage as i32)),
            "minecraft:bedrock" => ItemKind::Bedrock,
            "minecraft:beef" => ItemKind::RawBeef,
            "minecraft:beetroot" => ItemKind::Beetroot,
            "minecraft:beetroot_seeds" => ItemKind::Seeds(SeedMaterial::Beetroot),
            "minecraft:beetroot_soup" => ItemKind::Bowl(Some(BowlContents::BeetrootSoup)),
            "minecraft:birch_boat" => ItemKind::Boat(WoodMaterial::Birch),
            "minecraft:birch_door" => ItemKind::Door(DoorMaterial::Birch),
            "minecraft:birch_fence" => ItemKind::Fence(FenceMaterial::Birch),
            "minecraft:birch_fence_gate" => ItemKind::FenceGate(WoodMaterial::Birch),
            "minecraft:birch_stairs" => ItemKind::Stairs(StairMaterial::Birch),
            "minecraft:black_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Black))
            }
            "minecraft:black_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Black))
            }
            "minecraft:blaze_powder" => ItemKind::BlazePowder,
            "minecraft:blaze_rod" => ItemKind::BlazeRod,
            "minecraft:blue_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Blue))
            }
            "minecraft:blue_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Blue))
            }
            "minecraft:boat" => ItemKind::Boat(WoodMaterial::Oak),
            "minecraft:bone" => ItemKind::Bone,
            "minecraft:bone_block" => ItemKind::BoneBlock,
            "minecraft:book" => ItemKind::Book,
            "minecraft:book_and_quill" => ItemKind::BookAndQuill,
            "minecraft:bookshelf" => ItemKind::Bookshelf,
            "minecraft:bow" => ItemKind::Bow,
            "minecraft:bowl" => ItemKind::Bowl(None),
            "minecraft:bread" => ItemKind::Bread,
            "minecraft:brewing_stand" => ItemKind::BrewingStand,
            "minecraft:brick" => ItemKind::Brick,
            "minecraft:brick_block" => ItemKind::Bricks,
            "minecraft:brick_stairs" => ItemKind::Stairs(StairMaterial::Brick),
            "minecraft:brown_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Brown))
            }
            "minecraft:brown_mushroom" => ItemKind::BrownMushroom,
            "minecraft:brown_mushroom_block" => ItemKind::BrownMushroomBlock,
            "minecraft:brown_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Brown))
            }
            "minecraft:bucket" => ItemKind::Bucket(None),
            "minecraft:cake" => ItemKind::Cake,
            "minecraft:cactus" => ItemKind::Cactus,
            "minecraft:carpet" => ItemKind::Carpet(Colour::from(damage as i32)),
            "minecraft:carrot" => ItemKind::Carrot,
            "minecraft:carrot_on_a_stick" => ItemKind::CarrotOnAStick,
            "minecraft:cauldron" => ItemKind::Cauldron,
            "minecraft:chain_command_block" => ItemKind::ChainCommandBlock,
            "minecraft:chainmail_boots" => ItemKind::Boots(ArmourMaterial::Chainmail),
            "minecraft:chainmail_chestplate" => {
                ItemKind::Chestplate(ArmourMaterial::Chainmail)
            }
            "minecraft:chainmail_helmet" => ItemKind::Helmet(ArmourMaterial::Chainmail),
            "minecraft:chainmail_leggings" => ItemKind::Leggings(ArmourMaterial::Chainmail),
            "minecraft:chest" => ItemKind::Chest,
            "minecraft:chest_minecart" => ItemKind::Minecart(Some(MinecartContents::Chest)),
            "minecraft:chicken" => ItemKind::RawChicken,
            "minecraft:chorus_flower" => ItemKind::ChorusFlower,
            "minecraft:chorus_fruit" => ItemKind::ChorusFruit,
            "minecraft:chorus_fruit_popped" => ItemKind::PoppedChorusFruit,
            "minecraft:chorus_plant" => ItemKind::ChorusPlant,
            "minecraft:clay" => ItemKind::ClayBlock,
            "minecraft:clay_ball" => ItemKind::Clay,
            "minecraft:clock" => ItemKind::Clock,
            "minecraft:coal" => {
                match damage {
                    0 => ItemKind::Coal,
                    1 => ItemKind::Charcoal,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:coal_block" => ItemKind::BlockOfCoal,
            "minecraft:coal_ore" => ItemKind::CoalOre,
            "minecraft:cobblestone" => ItemKind::Cobblestone,
            "minecraft:cobblestone_wall" => {
                match damage {
                    0 => ItemKind::CobblestoneWall,
                    1 => ItemKind::MossyCobblestoneWall,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:command_block" => ItemKind::CommandBlock,
            "minecraft:command_block_minecraft" => {
                ItemKind::Minecart(Some(MinecartContents::CommandBlock))
            }
            "minecraft:comparator" => ItemKind::RedstoneComparator,
            "minecraft:compass" => ItemKind::Compass,
            "minecraft:concrete" => ItemKind::Concrete(None),
            "minecraft:concrete_powder" => ItemKind::ConcretePowder(None),
            "minecraft:cooked_beef" => ItemKind::Steak,
            "minecraft:cooked_chicken" => ItemKind::CookedChicken,
            "minecraft:cooked_fish" => ItemKind::Fish, // TODO Damage denotes type of fish
            "minecraft:cooked_mutton" => ItemKind::CookedMutton,
            "minecraft:cooked_porkchop" => ItemKind::Porkchop { cooked: true },
            "minecraft:cooked_rabbit" => ItemKind::CookedRabbit,
            "minecraft:cookie" => ItemKind::Cookie,
            "minecraft:crafting_table" => ItemKind::CraftingTable,
            "minecraft:cyan_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Cyan))
            }
            "minecraft:cyan_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Cyan))
            }
            "minecraft:dark_oak_boat" => ItemKind::Boat(WoodMaterial::DarkOak),
            "minecraft:dark_oak_door" => ItemKind::Door(DoorMaterial::DarkOak),
            "minecraft:dark_oak_fence" => ItemKind::Fence(FenceMaterial::DarkOak),
            "minecraft:dark_oak_fence_gate" => ItemKind::FenceGate(WoodMaterial::DarkOak),
            "minecraft:dark_oak_stairs" => ItemKind::Stairs(StairMaterial::DarkOak),
            "minecraft:daylight_detector" => ItemKind::DaylightDetector,
            "minecraft:detector_rail" => ItemKind::DetectorRail,
            "minecraft:deadbush" => ItemKind::DeadBush,
            "minecraft:diamond" => ItemKind::Diamond,
            "minecraft:diamond_axe" => ItemKind::Axe(ToolMaterial::Diamond),
            "minecraft:diamond_block" => ItemKind::BlockOfDiamond,
            "minecraft:diamond_boots" => ItemKind::Boots(ArmourMaterial::Diamond),
            "minecraft:diamond_chestplate" => ItemKind::Chestplate(ArmourMaterial::Diamond),
            "minecraft:diamond_helmet" => ItemKind::Helmet(ArmourMaterial::Diamond),
            "minecraft:diamond_hoe" => ItemKind::Hoe(ToolMaterial::Diamond),
            "minecraft:diamond_horse_armor" => {
                ItemKind::HorseArmor(HorseArmorMaterial::Diamond)
            }
            "minecraft:diamond_leggings" => ItemKind::Leggings(ArmourMaterial::Diamond),
            "minecraft:diamond_ore" => ItemKind::DiamondOre,
            "minecraft:diamond_pickaxe" => ItemKind::Pickaxe(ToolMaterial::Diamond),
            "minecraft:diamond_shovel" => ItemKind::Shovel(ToolMaterial::Diamond),
            "minecraft:diamond_sword" => ItemKind::Sword(ToolMaterial::Diamond),
            "minecraft:dirt" => {
                match damage {
                    0 => ItemKind::Dirt,
                    1 => ItemKind::CoarseDirt,
                    2 => ItemKind::Podzol,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:dispenser" => ItemKind::Dispenser,
            "minecraft:double_plant" => {
                match damage {
                    // TODO check what values are correct / actually needed
                    0 => ItemKind::Flower(Flower::SunflowerBottom),
                    1 => ItemKind::Flower(Flower::LilacBottom),
                    2 => ItemKind::DoubleTallgrass,
                    3 => ItemKind::LargeFern,
                    4 => ItemKind::Flower(Flower::RoseBushBottom),
                    5 => ItemKind::Flower(Flower::PeonyBottom),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:dragon_egg" => ItemKind::DragonEgg,
            "minecraft:dragons_breath" => ItemKind::DragonBreath,
            "minecraft:dropper" => ItemKind::Dropper,
            "minecraft:dye" => {
                match damage {
                    0 => ItemKind::InkSac,
                    1 => ItemKind::RoseRed,
                    2 => ItemKind::CactusGreen,
                    3 => ItemKind::CocoaBeans,
                    4 => ItemKind::LapisLazuli,
                    5 => ItemKind::PurpleDye,
                    6 => ItemKind::CyanDye,
                    7 => ItemKind::LightGrayDye,
                    8 => ItemKind::GrayDye,
                    9 => ItemKind::PinkDye,
                    10 => ItemKind::LimeDye,
                    11 => ItemKind::DandelionYellow,
                    12 => ItemKind::LightBlueDye,
                    13 => ItemKind::MagentaDye,
                    14 => ItemKind::OrangeDye,
                    15 => ItemKind::BoneMeal,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:egg" => ItemKind::Egg,
            "minecraft:elytra" => ItemKind::Elytra,
            "minecraft:emerald" => ItemKind::Emerald,
            "minecraft:emerald_block" => ItemKind::BlockOfEmerald,
            "minecraft:emerald_ore" => ItemKind::EmeraldOre,
            "minecraft:enchanted_book" => ItemKind::EnchantedBook,
            "minecraft:enchanting_table" => ItemKind::EnchantingTable,
            "minecraft:end_bricks" => ItemKind::EndBricks,
            "minecraft:end_crystal" => ItemKind::EndCrystal,
            "minecraft:ender_chest" => ItemKind::EnderChest,
            "minecraft:ender_eye" => ItemKind::EnderEye,
            "minecraft:ender_pearl" => ItemKind::EnderPearl,
            "minecraft:end_portal_frame" => ItemKind::EndPortalFrame,
            "minecraft:end_rod" => ItemKind::EndRod,
            "minecraft:end_stone" => ItemKind::EndStone,
            "minecraft:experience_bottle" => ItemKind::BottleOEnchanting,
            "minecraft:farmland" => ItemKind::Farmland,
            "minecraft:feather" => ItemKind::Feather,
            "minecraft:fence" => ItemKind::Fence(FenceMaterial::Oak),
            "minecraft:fence_gate" => ItemKind::FenceGate(WoodMaterial::Oak),
            "minecraft:fermented_spider_eye" => ItemKind::FermentedSpiderEye,
            "minecraft:filled_map" => ItemKind::Map,
            "minecraft:fire_charge" => ItemKind::FireCharge,
            "minecraft:firework_charge" => ItemKind::FireworkStar,
            "minecraft:fireworks" => ItemKind::Fireworks,
            "minecraft:fish" => ItemKind::Fish, // TODO Damage denotes type of fish
            "minecraft:fishing_rod" => ItemKind::FishingRod,
            "minecraft:flint" => ItemKind::Flint,
            "minecraft:flint_and_steel" => ItemKind::FlintAndSteel,
            "minecraft:flower_pod" => ItemKind::FlowerPot,
            "minecraft:furnace" => ItemKind::Furnace,
            "minecraft:furnace_minecart" => {
                ItemKind::Minecart(Some(MinecartContents::Furnace))
            }
            "minecraft:ghast_tear" => ItemKind::GhastTear,
            "minecraft:glass" => ItemKind::Glass(None),
            "minecraft:glass_bottle" => ItemKind::GlassBottle,
            "minecraft:glass_pane" => ItemKind::GlassPane(None),
            "minecraft:glowstone" => ItemKind::Glowstone,
            "minecraft:glowstone_dust" => ItemKind::GlowstoneDust,
            "minecraft:gold_block" => ItemKind::BlockOfGold,
            "minecraft:golden_apple" => {
                match damage {
                    0 => ItemKind::GoldenApple,
                    1 => ItemKind::EnchantedGoldenApple,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:golden_axe" => ItemKind::Axe(ToolMaterial::Gold),
            "minecraft:golden_boots" => ItemKind::Boots(ArmourMaterial::Gold),
            "minecraft:golden_carrot" => ItemKind::GoldenCarrot,
            "minecraft:golden_chestplate" => ItemKind::Chestplate(ArmourMaterial::Gold),
            "minecraft:golden_helmet" => ItemKind::Helmet(ArmourMaterial::Gold),
            "minecraft:golden_hoe" => ItemKind::Hoe(ToolMaterial::Gold),
            "minecraft:golden_horse_armor" => ItemKind::HorseArmor(HorseArmorMaterial::Gold),
            "minecraft:golden_leggings" => ItemKind::Leggings(ArmourMaterial::Gold),
            "minecraft:golden_pickaxe" => ItemKind::Pickaxe(ToolMaterial::Gold),
            "minecraft:golden_shovel" => ItemKind::Shovel(ToolMaterial::Gold),
            "minecraft:golden_sword" => ItemKind::Sword(ToolMaterial::Gold),
            "minecraft:gold_ingot" => ItemKind::Ingot(IngotMaterial::Gold),
            "minecraft:gold_nugget" => ItemKind::Nugget(NuggetMaterial::Gold),
            "minecraft:gold_ore" => ItemKind::GoldOre,
            "minecraft:golden_rail" => ItemKind::PoweredRail,
            "minecraft:grass" => ItemKind::GrassBlock,
            "minecraft:grass_path" => ItemKind::GrassPath,
            "minecraft:gravel" => ItemKind::Gravel,
            "minecraft:gray_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Gray))
            }
            "minecraft:gray_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Gray))
            }
            "minecraft:green_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Green))
            }
            "minecraft:green_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Green))
            }
            "minecraft:gunpowder" => ItemKind::Gunpowder,
            "minecraft:hardened_clay" => ItemKind::Terracotta(None),
            "minecraft:hay_block" => ItemKind::HayBale,
            "minecraft:heavy_weighted_pressure_plate" => {
                ItemKind::PressurePlate(PressurePlateMaterial::Iron)
            }
            "minecraft:hopper" => ItemKind::Hopper,
            "minecrart:hopper_minecart" => {
                ItemKind::Minecart(Some(MinecartContents::Hopper))
            }
            "minecraft:ice" => ItemKind::Ice,
            "minecraft:iron_axe" => ItemKind::Axe(ToolMaterial::Iron),
            "minecraft:iron_bars" => ItemKind::IronBars,
            "minecraft:iron_block" => ItemKind::BlockOfIron,
            "minecraft:iron_boots" => ItemKind::Boots(ArmourMaterial::Iron),
            "minecraft:iron_chestplate" => ItemKind::Chestplate(ArmourMaterial::Iron),
            "minecraft:iron_door" => ItemKind::Door(DoorMaterial::Iron),
            "minecraft:iron_helmet" => ItemKind::Helmet(ArmourMaterial::Iron),
            "minecraft:iron_hoe" => ItemKind::Hoe(ToolMaterial::Iron),
            "minecraft:iron_horse_armor" => ItemKind::HorseArmor(HorseArmorMaterial::Iron),
            "minecraft:iron_ingot" => ItemKind::Ingot(IngotMaterial::Iron),
            "minecraft:iron_leggings" => ItemKind::Leggings(ArmourMaterial::Iron),
            "minecraft:iron_nugget" => ItemKind::Nugget(NuggetMaterial::Iron),
            "minecraft:iron_ore" => ItemKind::IronOre,
            "minecraft:iron_pickaxe" => ItemKind::Pickaxe(ToolMaterial::Iron),
            "minecraft:iron_shovel" => ItemKind::Shovel(ToolMaterial::Iron),
            "minecraft:iron_sword" => ItemKind::Sword(ToolMaterial::Iron),
            "minecraft:iron_trapdoor" => ItemKind::Trapdoor(DoorMaterial::Iron),
            "minecraft:item_frame" => ItemKind::ItemFrame,
            "minecraft:jukebox" => ItemKind::Jukebox,
            "minecraft:jungle_boat" => ItemKind::Boat(WoodMaterial::Jungle),
            "minecraft:jungle_door" => ItemKind::Door(DoorMaterial::Jungle),
            "minecraft:jungle_fence" => ItemKind::Fence(FenceMaterial::Jungle),
            "minecraft:jungle_fence_gate" => ItemKind::FenceGate(WoodMaterial::Jungle),
            "minecraft:jungle_stairs" => ItemKind::Stairs(StairMaterial::Jungle),
            "minecraft:knowledge_book" => ItemKind::KnowledgeBook,
            "minecraft:ladder" => ItemKind::Ladder,
            "minecraft:lapis_block" => ItemKind::LapisLazuliBlock,
            "minecraft:lapis_ore" => ItemKind::LapisLazuliOre,
            "minecraft:lava_bucket" => ItemKind::Bucket(Some(BucketContents::Lava)),
            "minecraft:lead" => ItemKind::Lead,
            "minecraft:leather" => ItemKind::Leather,
            "minecraft:leather_boots" => ItemKind::Boots(ArmourMaterial::Leather),
            "minecraft:leather_chestplate" => ItemKind::Chestplate(ArmourMaterial::Leather),
            "minecraft:leather_helmet" => ItemKind::Helmet(ArmourMaterial::Leather),
            "minecraft:leather_leggings" => ItemKind::Leggings(ArmourMaterial::Leather),
            "minecraft:leaves" => {
                match damage {
                    0 => ItemKind::OakLeaves,
                    1 => ItemKind::SpruceLeaves,
                    2 => ItemKind::BirchLeaves,
                    3 => ItemKind::JungleLeaves,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:leaves2" => {
                match damage {
                    // TODO figure out what values are correct
                    0 | 4 => ItemKind::AcaciaLeaves,
                    1 | 5 => ItemKind::DarkOakLeaves,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:lever" => ItemKind::Lever,
            "minecraft:light_blue_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::LightBlue))
            }
            "minecraft:light_blue_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::LightBlue))
            }
            "minecraft:light_weighted_pressure_plate" => {
                ItemKind::PressurePlate(PressurePlateMaterial::Gold)
            }
            "minecraft:lime_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Lime))
            }
            "minecraft:lime_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Lime))
            }
            "minecraft:lit_pumpkin" => ItemKind::JackOLantern,
            "minecraft:log" => {
                match damage {
                    0 => ItemKind::Log(WoodMaterial::Oak),
                    1 => ItemKind::Log(WoodMaterial::Spruce),
                    2 => ItemKind::Log(WoodMaterial::Birch),
                    3 => ItemKind::Log(WoodMaterial::Jungle),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:log2" => {
                match damage {
                    // TODO figure out which values are correct
                    0 | 4 => ItemKind::Log(WoodMaterial::Acacia),
                    1 | 5 => ItemKind::Log(WoodMaterial::DarkOak),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:magenta_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Magenta))
            }
            "minecraft:magenta_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Magenta))
            }
            "minecraft:magma" => ItemKind::Magma,
            "minecraft:magma_cream" => ItemKind::MagmaCream,
            "minecraft:map" => ItemKind::EmptyMap,
            "minecraft:melon" => ItemKind::Melon,
            "minecraft:melon_block" => ItemKind::MelonBlock,
            "minecraft:melon_seeds" => ItemKind::Seeds(SeedMaterial::Melon),
            "minecraft:milk_bucket" => ItemKind::Bucket(Some(BucketContents::Milk)),
            "minecraft:minecart" => ItemKind::Minecart(None),
            "minecraft:mob_spawner" => ItemKind::MobSpawner,
            "minecraft:monster_egg" => {
                match damage {
                    0 => ItemKind::InfestedStone,
                    1 => ItemKind::InfestedCobblestone,
                    2 => ItemKind::InfestedStoneBricks,
                    3 => ItemKind::InfestedMossyStoneBricks,
                    4 => ItemKind::InfestedCrackedStoneBricks,
                    5 => ItemKind::InfestedChiseledStoneBricks,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:mossy_cobblestone" => ItemKind::MossyCobblestone,
            "minecraft:mushroom_stew" => ItemKind::Bowl(Some(BowlContents::MushroomStew)),
            "minecraft:mutton" => ItemKind::RawMutton,
            "minecraft:mycelium" => ItemKind::Mycelium,
            "minecraft:name_tag" => ItemKind::NameTag,
            "minecraft:netherbrick" => ItemKind::NetherBrick,
            "minecraft:nether_brick" => ItemKind::NetherBrickBlock,
            "minecraft:nether_brick_fence" => ItemKind::Fence(FenceMaterial::NetherBrick),
            "minecraft:nether_brick_stairs" => {
                ItemKind::Stairs(StairMaterial::NetherBrick)
            }
            "minecraft:netherrack" => ItemKind::Netherrack,
            "minecraft:nether_star" => ItemKind::NetherStar,
            "minecraft:nether_wart" => ItemKind::NetherWart,
            "minecraft:nether_wart_block" => ItemKind::NetherWartBlock,
            "minecraft:noteblock" => ItemKind::Noteblock,
            "minecraft:oak_button" => ItemKind::Button(ButtonMaterial::Oak),
            "minecraft:oak_stairs" => ItemKind::Stairs(StairMaterial::Oak),
            "minecraft:observer" => ItemKind::Observer,
            "minecraft:obsidian" => ItemKind::Obsidian,
            "minecraft:orange_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Orange))
            }
            "minecraft:orange_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Orange))
            }
            "minecraft:packed_ice" => ItemKind::PackedIce,
            "minecraft:painting" => ItemKind::Painting,
            "minecraft:paper" => ItemKind::Paper,
            "minecraft:pink_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Pink))
            }
            "minecraft:pink_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Pink))
            }
            "minecraft:piston" => ItemKind::Piston,
            "minecraft:planks" => {
                match damage {
                    0 => ItemKind::Planks(WoodMaterial::Oak),
                    1 => ItemKind::Planks(WoodMaterial::Spruce),
                    2 => ItemKind::Planks(WoodMaterial::Birch),
                    3 => ItemKind::Planks(WoodMaterial::Jungle),
                    4 => ItemKind::Planks(WoodMaterial::Acacia),
                    5 => ItemKind::Planks(WoodMaterial::DarkOak),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:porkchop" => ItemKind::Porkchop { cooked: false },
            "minecraft:potato" => ItemKind::Potato,
            "minecraft:prismarine" => {
                match damage {
                    0 => ItemKind::Prismarine,
                    1 => ItemKind::DarkPrismarine,
                    2 => ItemKind::PrismarineBricks,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:prismarine_crystals" => ItemKind::PrismarineCrystals,
            "minecraft:prismarine_shard" => ItemKind::PrismarineShard,
            "minecraft:pumpkin" => ItemKind::Pumpkin,
            "minecraft:pumpkin_pie" => ItemKind::PumpkinPie,
            "minecraft:pumpkin_seeds" => ItemKind::Seeds(SeedMaterial::Pumpkin),
            "minecraft:purple_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Purple))
            }
            "minecraft:purple_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Purple))
            }
            "minecraft:purpur_block" => ItemKind::PurpurBlock,
            "minecraft:purpur_pillar" => ItemKind::PurpurPillar,
            "minecraft:purpur_slab" => ItemKind::Slab(SlabMaterial::Purpur),
            "minecraft:purpur_stairs" => {
                ItemKind::Stairs(StairMaterial::Purpur)
            }
            "minecraft:quartz" => ItemKind::NetherQuartz,
            "minecraft:quartz_block" => {
                match damage {
                    0 => ItemKind::BlockOfQuartz,
                    1 => ItemKind::ChiseledQuartzBlock,
                    2 => ItemKind::PillarQuartzBlock,
                    3 => ItemKind::SmoothQuartzBlock,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:quartz_ore" => ItemKind::QuartzOre,
            "minecraft:quartz_stairs" => ItemKind::Stairs(StairMaterial::Quartz),
            "minecraft:rabbit" => ItemKind::RawRabbit,
            "minecraft:rabbit_foot" => ItemKind::RabbitFoot,
            "minecraft:rabbit_hide" => ItemKind::RabbitHide,
            "minecraft:rabbit_stew" => ItemKind::Bowl(Some(BowlContents::RabbitStew)),
            "minecraft:rail" => ItemKind::Rail,
            "minecraft:record_11" => ItemKind::Record(Recording::Eleven),
            "minecraft:record_13" => ItemKind::Record(Recording::Thirteen),
            "minecraft:record_blocks" => ItemKind::Record(Recording::Blocks),
            "minecraft:record_cat" => ItemKind::Record(Recording::Cat),
            "minecraft:record_chirp" => ItemKind::Record(Recording::Chirp),
            "minecraft:record_far" => ItemKind::Record(Recording::Far),
            "minecraft:record_mall" => ItemKind::Record(Recording::Mall),
            "minecraft:record_mellohi" => ItemKind::Record(Recording::Mellohi),
            "minecraft:record_stal" => ItemKind::Record(Recording::Stal),
            "minecraft:record_strad" => ItemKind::Record(Recording::Strad),
            "minecraft:record_ward" => ItemKind::Record(Recording::Ward),
            "minecraft:record_wait" => ItemKind::Record(Recording::Wait),
            "minecraft:red_flower" => {
                match damage {
                    0 => ItemKind::Flower(Flower::Poppy),
                    1 => ItemKind::Flower(Flower::BlueOrchid),
                    2 => ItemKind::Flower(Flower::Allium),
                    3 => ItemKind::Flower(Flower::AzureBluet),
                    4 => ItemKind::Flower(Flower::TulipRed),
                    5 => ItemKind::Flower(Flower::TulipOrange),
                    6 => ItemKind::Flower(Flower::TulipLightGray),
                    7 => ItemKind::Flower(Flower::TulipPink),
                    8 => ItemKind::Flower(Flower::OxeyeDaisy),
                    9 => ItemKind::Flower(Flower::Cornflower),
                    10 => ItemKind::Flower(Flower::LilyOfTheValley),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:red_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Red))
            }
            "minecraft:red_mushroom" => ItemKind::RedMushroom,
            "minecraft:red_mushroom_block" => ItemKind::RedMushroomBlock,
            "minecraft:red_nether_brick" => ItemKind::RedNetherBrick,
            "minecraft:red_sandstone" => {
                match damage {
                    0 => ItemKind::RedSandstone,
                    1 => ItemKind::ChiseledRedSandstone,
                    2 => ItemKind::CutRedSandstone,
                    3 => ItemKind::SmoothRedSandstone,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:red_sandstone_stairs" => {
                ItemKind::Stairs(StairMaterial::RedSandstone)
            }
            "minecraft:red_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Red))
            }
            "minecraft:redstone" => ItemKind::Redstone,
            "minecraft:redstone_block" => ItemKind::BlockOfRedstone,
            "minecraft:redstone_lamp" => ItemKind::RedstoneLamp,
            "minecraft:redstone_ore" => ItemKind::RedstoneOre,
            "minecraft:redstone_torch" => ItemKind::RedstoneTorch,
            "minecraft:reeds" => ItemKind::SugarCane,
            "minecraft:repeater" => ItemKind::RedstoneRepeater,
            "minecraft:repeating_command_block" => ItemKind::RepeatingCommandBlock,
            "minecraft:rotten_flesh" => ItemKind::RottenFlesh,
            "minecraft:saddle" => ItemKind::Saddle,
            "minecraft:sand" => {
                match damage {
                    0 => ItemKind::Sand,
                    1 => ItemKind::RedSand,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:sandstone" => {
                match damage {
                    0 => ItemKind::Sandstone,
                    1 => ItemKind::ChiseledSandstone,
                    2 => ItemKind::CutSandstone,
                    3 => ItemKind::SmoothSandstone,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:sandstone_stairs" => ItemKind::Stairs(StairMaterial::Sandstone),
            "minecraft:sapling" => {
                match damage {
                    0 => ItemKind::Sapling(SaplingMaterial::Oak),
                    1 => ItemKind::Sapling(SaplingMaterial::Spruce),
                    2 => ItemKind::Sapling(SaplingMaterial::Birch),
                    3 => ItemKind::Sapling(SaplingMaterial::Jungle),
                    4 => ItemKind::Sapling(SaplingMaterial::Acacia),
                    5 => ItemKind::Sapling(SaplingMaterial::DarkOak),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:sea_lantern" => ItemKind::SeaLantern,
            "minecraft:shears" => ItemKind::Shears,
            "minecraft:shield" => ItemKind::Shield,
            "minecraft:shulker_shell" => ItemKind::ShulkerShell,
            "minecraft:sign" => ItemKind::Sign(WoodMaterial::Oak),
            "minecraft:silver_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::LightGray))
            }
            "minecraft:silver_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::LightGray))
            }
            // TODO Heads: 0 skeleton, 1 wither skeleton, 2 zombie,
            // 3 player (steve?), 4 creeper, 5 dragon (?)
            "minecraft:skull" => ItemKind::MobHead,
            "minecraft:slime" => ItemKind::SlimeBlock,
            "minecraft:slime_ball" => ItemKind::Slimeball,
            "minecraft:snow" => ItemKind::Snow,
            "minecraft:snowball" => ItemKind::Snowball,
            "minecraft:snow_layer" => ItemKind::SnowLayer,
            "minecraft:soul_sand" => ItemKind::SoulSand,
            "minecraft:spawn_egg" => ItemKind::SpawnEgg,
            "minecraft:speckled_melon" => ItemKind::GlisteringMelon,
            "minecraft:spider_eye" => ItemKind::SpiderEye,
            "minecraft:sponge" => {
                match damage {
                    0 => ItemKind::Sponge,
                    1 => ItemKind::WetSponge,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:spruce_boat" => ItemKind::Boat(WoodMaterial::Spruce),
            "minecraft:spruce_door" => ItemKind::Door(DoorMaterial::Spruce),
            "minecraft:spruce_fence" => ItemKind::Fence(FenceMaterial::Spruce),
            "minecraft:spruce_fence_gate" => ItemKind::FenceGate(WoodMaterial::Spruce),
            "minecraft:spruce_stairs" => ItemKind::Stairs(StairMaterial::Spruce),
            "minecraft:stained_glass" => ItemKind::Glass(Some(Colour::from(damage as i32))),
            "minecraft:stained_glass_pane" => {
                ItemKind::GlassPane(Some(Colour::from(damage as i32)))
            }
            "minecraft:stained_hardened_clay" => {
                ItemKind::Terracotta(Some(Colour::from(damage as i32)))
            }
            "minecraft:stick" => ItemKind::Stick,
            "minecraft:sticky_piston" => ItemKind::StickyPiston,
            "minecraft:stone" => {
                match damage {
                    0 => ItemKind::Stone,
                    1 => ItemKind::Granite,
                    2 => ItemKind::PolishedGranite,
                    3 => ItemKind::Diorite,
                    4 => ItemKind::PolishedDiorite,
                    5 => ItemKind::Andesite,
                    6 => ItemKind::PolishedAndesite,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:stone_axe" => ItemKind::Axe(ToolMaterial::Stone),
            "minecraft:stonebrick" => {
                match damage {
                    0 => ItemKind::StoneBricks,
                    1 => ItemKind::MossyStoneBricks,
                    2 => ItemKind::CrackedStoneBricks,
                    3 => ItemKind::ChiseledStoneBricks,
                    4 => ItemKind::SmoothStoneBricks,
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:stone_brick_stairs" => ItemKind::Stairs(StairMaterial::StoneBrick),
            "minecraft:stone_button" => ItemKind::Button(ButtonMaterial::Stone),
            "minecraft:stone_hoe" => ItemKind::Hoe(ToolMaterial::Stone),
            "minecraft:stone_pickaxe" => ItemKind::Pickaxe(ToolMaterial::Stone),
            "minecraft:stone_pressure_plate" => {
                ItemKind::PressurePlate(PressurePlateMaterial::Stone)
            }
            "minecraft:stone_shovel" => ItemKind::Shovel(ToolMaterial::Stone),
            "minecraft:stone_slab" => {
                match damage {
                    0 => ItemKind::Slab(SlabMaterial::SmoothStone),
                    1 => ItemKind::Slab(SlabMaterial::Sandstone),
                    3 => ItemKind::Slab(SlabMaterial::Cobblestone),
                    4 => ItemKind::Slab(SlabMaterial::Brick),
                    5 => ItemKind::Slab(SlabMaterial::StoneBrick),
                    6 => ItemKind::Slab(SlabMaterial::Quartz),
                    7 => ItemKind::Slab(SlabMaterial::NetherBrick),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:stone_slab2" => ItemKind::Slab(SlabMaterial::RedSandstone),
            "minecraft:stone_stairs" => ItemKind::Stairs(StairMaterial::Cobblestone),
            "minecraft:stone_sword" => ItemKind::Sword(ToolMaterial::Stone),
            "minecraft:string" => ItemKind::String,
            "minecraft:structure_block" => ItemKind::StructureBlock,
            "minecraft:structure_void" => ItemKind::StructureVoid,
            "minecraft:sugar" => ItemKind::Sugar,
            "minecraft:tallgrass" => ItemKind::Grass,
            "minecraft:tnt" => ItemKind::TNT,
            "minecraft:tnt_minecart" => {
                ItemKind::Minecart(Some(MinecartContents::TNT))
            }
            "minecraft:torch" => ItemKind::Torch,
            "minecraft:totem_of_undying" => ItemKind::TotemOfUndying,
            "minecraft:trapdoor" => ItemKind::Trapdoor(DoorMaterial::Oak),
            "minecraft:trapped_chest" => ItemKind::TrappedChest,
            "minecraft:tripwire_hook" => ItemKind::TripwireHook,
            "minecraft:vine" => ItemKind::Vines,
            "minecraft:water_bucket" => ItemKind::Bucket(Some(BucketContents::Water)),
            "minecraft:waterlily" => ItemKind::LilyPad,
            "minecraft:web" => ItemKind::Cobweb,
            "minecraft:wheat" => ItemKind::Wheat,
            "minecraft:wheat_seeds" => ItemKind::Seeds(SeedMaterial::Wheat),
            "minecraft:white_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::White))
            }
            "minecraft:white_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::White))
            }
            "minecraft:wooden_axe" => ItemKind::Axe(ToolMaterial::Wood),
            "minecraft:wooden_door" => ItemKind::Door(DoorMaterial::Oak),
            "minecraft:wooden_hoe" => ItemKind::Hoe(ToolMaterial::Wood),
            "minecraft:wooden_pickaxe" => ItemKind::Pickaxe(ToolMaterial::Wood),
            "minecraft:wooden_pressure_plate" => {
                ItemKind::PressurePlate(PressurePlateMaterial::Oak)
            }
            "minecraft:wooden_shovel" => ItemKind::Shovel(ToolMaterial::Wood),
            "minecraft:wooden_slab" => {
                match damage {
                    0 => ItemKind::Slab(SlabMaterial::Oak),
                    1 => ItemKind::Slab(SlabMaterial::Spruce),
                    2 => ItemKind::Slab(SlabMaterial::Birch),
                    3 => ItemKind::Slab(SlabMaterial::Jungle),
                    4 => ItemKind::Slab(SlabMaterial::Acacia),
                    5 => ItemKind::Slab(SlabMaterial::DarkOak),
                    _ => ItemKind::Unknown,
                }
            }
            "minecraft:wooden_sword" => ItemKind::Sword(ToolMaterial::Wood),
            "minecraft:wool" => ItemKind::Wool(Colour::from(damage as i32)),
            "minecraft:written_book" => ItemKind::WrittenBook,
            "minecraft:yellow_flower" => ItemKind::Flower(Flower::Dandelion),
            "minecraft:yellow_glazed_terracotta" => {
                ItemKind::GlazedTerracotta(Some(Colour::Yellow))
            }
            "minecraft:yellow_shulker_box" => {
                ItemKind::ShulkerBox(Some(Colour::Yellow))
            }
            _ => ItemKind::Unknown,
        };

        Self {
            custom_name: None, // TODO
            enchantments: Vec::new(), // TODO
            damage: None, // TODO
            kind,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BowlContents {
    BeetrootSoup,
    MushroomStew,
    RabbitStew,
    SuspiciousStew,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BucketContents {
    Empty,
    Lava,
    Milk,
    Water,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Recording {
    Blocks,
    Cat,
    Chirp,
    Eleven,
    Far,
    Mall,
    Mellohi,
    Stal,
    Strad,
    Thirteen,
    Ward,
    Wait,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MinecartContents {
    Chest,
    CommandBlock,
    Furnace,
    Hopper,
    TNT,
}

#[derive(Clone, Debug)]
pub enum ItemKind {
    // mcprogedit "items"
    Unknown,

    // "Non-block" items
    Apple,
    Arrow, // spectral, tipped (multiple),
    ArmorStand,
    Axe(ToolMaterial),
    BakedPotato,
    Banner,
    Bed(Colour),
    Beef, // raw, cooked
    Beetroot,
    BlazePowder,
    BlazeRod,
    Boat(WoodMaterial),
    Bone,
    BoneMeal,
    Book,
    BookAndQuill,
    Boots(ArmourMaterial),
    BottleOEnchanting,
    Bow,
    Bowl(Option<BowlContents>),
    Bread,
    BrewingStand,
    Brick,
    Bucket(Option<BucketContents>),
    CactusGreen,
    Cake,
    Carrot,
    CarrotOnAStick,
    Cauldron,
    Charcoal,
    Chestplate(ArmourMaterial),
    Chicken, // raw, cooked
    ChorusFruit, // popped?
    Clay,
    Clock,
    Coal,
    CocoaBeans,
    Compass,
    CookedChicken,
    CookedMutton,
    CookedRabbit,
    Cookie,
    CyanDye,
    DandelionYellow,
    Diamond,
    Door(DoorMaterial),
    DragonBreath,
    Dye, // colours
    Egg,
    Elytra,
    Emerald,
    EmptyMap,
    EnchantedBook,
    EnchantedGoldenApple,
    EndCrystal,
    EnderEye,
    EnderPearl,
    Feather,
    FermentedSpiderEye,
    FireCharge,
    FireworkRocket,
    Fireworks,
    FireworkStar,
    Fish, // TODO Raw, Cooked / type of fish
    FishingRod,
    Flint,
    FlintAndSteel,
    FlowerPot,
    GhastTear,
    GlassBottle,
    GlisteringMelon,
    GlowstoneDust,
    GoldenApple,
    GoldenCarrot,
    GrayDye,
    Gunpowder,
    Helmet(ArmourMaterial),
    Hoe(ToolMaterial),
    HorseArmor(HorseArmorMaterial), // Iron, Golden, Diamond
    Ingot(IngotMaterial),
    InkSac,
    ItemFrame,
    KnowledgeBook,
    LapisLazuli,
    Lead,
    Leather,
    Leggings(ArmourMaterial),
    LightBlueDye,
    LightGrayDye,
    LimeDye,
    MagentaDye,
    MagmaCream,
    Map, // empty? filled?
    Melon,
    Minecart(Option<MinecartContents>), // Chest, CommandBlock, Empty, Furnace, Hopper, TNT
    MobHead,
    Mutton, // raw, cooked
    NameTag,
    NetherBrick,
    NetherQuartz,
    NetherStar,
    NetherWart,
    Nugget(NuggetMaterial),
    OrangeDye,
    Painting,
    Paper,
    Pickaxe(ToolMaterial),
    PinkDye,
    PoppedChorusFruit,
    Porkchop{ cooked: bool },
    Potion, // type / splash? / lingering?
    Potato, // poisonous?
    PrismarineCrystals,
    PrismarineShard,
    PumpkinPie,
    PurpleDye,
    Rabbit, // raw, cooked
    RabbitFoot,
    RabbitHide,
    RawBeef,
    RawChicken,
    RawMutton,
    RawRabbit,
    Record(Recording),
    Redstone,
    RedstoneComparator,
    RedstoneRepeater,
    RoseRed,
    RottenFlesh,
    Saddle,
    Scull, // aka "mob head"
    Seeds(SeedMaterial), // wheat, melon, pumpkin, beetroot
    Shears,
    Shield,
    Shovel(ToolMaterial),
    ShulkerShell,
    Sign(WoodMaterial),
    Slimeball,
    Snowball,
    SpawnEgg, // type
    SpiderEye,
    Steak,
    Stick,
    String,
    Sugar,
    SugarCane,
    Sword(ToolMaterial),
    TotemOfUndying,
    Wheat,
    WrittenBook,

    // Blocks as items
    AcaciaLeaves,
    ActivatorRail,
    Andesite,
    Anvil(AnvilDamage),
    Barrier,
    Beacon,
    Bedrock,
    BirchLeaves,
    BlockOfCoal,
    BlockOfDiamond,
    BlockOfEmerald,
    BlockOfGold,
    BlockOfIron,
    BlockOfQuartz,
    BlockOfRedstone,
    BoneBlock,
    Bookshelf,
    Bricks,
    BrownMushroom,
    BrownMushroomBlock,
    Button(ButtonMaterial),
    Cactus,
    Carpet(Colour),
    ChainCommandBlock,
    Chest,
    ChiseledQuartzBlock,
    ChiseledRedSandstone,
    ChiseledSandstone,
    ChiseledStoneBricks,
    ChorusFlower,
    ChorusPlant,
    ClayBlock,
    CoalOre,
    CoarseDirt,
    Cobblestone,
    CobblestoneWall,
    Cobweb,
    CommandBlock,
    Concrete(Option<Colour>),
    ConcretePowder(Option<Colour>),
    CrackedStoneBricks,
    CraftingTable,
    CutRedSandstone,
    CutSandstone,
    DarkOakLeaves,
    DarkPrismarine,
    DaylightDetector,
    DeadBush,
    DetectorRail,
    DiamondOre,
    Diorite,
    Dirt,
    Dispenser,
    DoubleTallgrass,
    DragonEgg,
    Dropper,
    EmeraldOre,
    EnchantingTable,
    EndBricks,
    EnderChest,
    EndPortalFrame,
    EndRod,
    EndStone,
    Farmland,
    Fence(FenceMaterial),
    FenceGate(WoodMaterial),
    Flower(Flower),
    Furnace,
    Glass(Option<Colour>),
    GlassPane(Option<Colour>),
    GlazedTerracotta(Option<Colour>),
    Glowstone,
    GoldOre,
    Granite,
    Grass,
    GrassPath,
    GrassBlock,
    Gravel,
    HayBale,
    Hopper,
    Ice,
    InfestedChiseledStoneBricks,
    InfestedCobblestone,
    InfestedCrackedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedStone,
    InfestedStoneBricks,
    IronBars,
    IronOre,
    JackOLantern,
    JungleLeaves,
    Jukebox,
    Ladder,
    LapisLazuliBlock,
    LapisLazuliOre,
    LargeFern,
    Lever,
    LilyPad,
    Log(WoodMaterial),
    Magma,
    MelonBlock,
    MobSpawner,
    MossyCobblestone,
    MossyCobblestoneWall,
    MossyStoneBricks,
    Mycelium,
    NetherBrickBlock,
    Netherrack,
    NetherWartBlock,
    Noteblock,
    OakLeaves,
    Observer,
    Obsidian,
    PackedIce,
    PillarQuartzBlock,
    Piston,
    Planks(WoodMaterial),
    Podzol,
    PolishedAndesite,
    PolishedDiorite,
    PolishedGranite,
    PoweredRail,
    PressurePlate(PressurePlateMaterial),
    Prismarine,
    PrismarineBricks,
    Pumpkin,
    PurpurBlock,
    PurpurPillar,
    QuartzOre,
    Rail,
    RedMushroom,
    RedMushroomBlock,
    RedNetherBrick,
    RedSand,
    RedSandstone,
    RedstoneLamp,
    RedstoneOre,
    RedstoneTorch,
    RepeatingCommandBlock,
    Sand,
    Sandstone,
    Sapling(SaplingMaterial),
    SeaLantern,
    ShulkerBox(Option<Colour>),
    Slab(SlabMaterial),
    SlimeBlock,
    SmoothQuartzBlock,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStoneBricks,
    Snow,
    SnowLayer,
    SoulSand,
    Sponge,
    SpruceLeaves,
    Stairs(StairMaterial),
    StickyPiston,
    Stone,
    StoneBricks,
    StructureBlock,
    StructureVoid,
    Terracotta(Option<Colour>),
    TNT,
    Torch,
    Trapdoor(DoorMaterial),
    TrappedChest,
    TripwireHook,
    Vines,
    WetSponge,
    Wool(Colour),
}