use nbt::Value;

use crate::block::*;
use crate::bounded_ints::*;
use crate::colour::Colour;
use crate::material::*;
use crate::nbt_lookup::*;
use crate::positioning::*;

/// Structure for storing palette data for blocks that also have parts of its data stored in
/// a block entity. Those blocks have individual block entities, but share the data stored
/// in the palette. When parsing the Palette the shared data is stored in this "proto block".
/// Later, when parsing BlockStates, the proto block gets combined with the block entities to
/// form the actual blocks.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum ProtoBlock {
    Banner { colour: Colour, placement: WallOrRotatedOnFloor },
    Beacon,
    BrewingStand,
    Chest { facing: Surface4, variant: Option<ChestVariant>, waterlogged: bool },
    Dispenser { facing: Surface6 },
    Dropper { facing: Surface6 },
    EnchantingTable,
    Furnace { facing: Surface4, lit: bool },
    Hopper { facing: Surface5 },
    Jukebox,
    ShulkerBox { colour: Option<Colour>, facing: Surface6 },
    Sign { material: WoodMaterial, placement: WallOrRotatedOnFloor, waterlogged: bool },
    TrappedChest { facing: Surface4, variant: Option<ChestVariant>, waterlogged: bool },
}

/// The palette will contain some fully parsed blocks, and some blocks for which the block
/// entity has not been matched yet. The latter contain only information from the Palette
/// NBT value, and are named "proto blocks". They will later be combined with block entities
/// to form full blocks.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum PaletteItem {
    Block(Block),
    ProtoBlock(ProtoBlock),
}

pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    // Import Palette. It contains a list of compounds, each with a Name:String (Namespaced block ID)
    // and optionally a Properties:Compound which contains pairs of Name:String, value (for
    // e.g. facing.) Essentially holding all info previously encoded in blocks + add + data.
    let mut palette: Vec<PaletteItem> = Vec::new();
    let raw_palette = if let Ok(p) = nbt_value_lookup_list(section, "Palette") { p } else { return None };
    //println!("Section {}: {}", section_y_index, section);
    println!("Raw palette: {:#?}", raw_palette);
    for raw_block in raw_palette {
        let name = nbt_value_lookup_string(&raw_block, "Name").unwrap();
        let properties = nbt_value_lookup(&raw_block, "Properties").ok();

        // Source for (hopefully exhaustive) list of IDs: https://minecraftitemids.com
        let palette_item = match name.as_str() {
            "minecraft:air" => block(Block::Air),
            "minecraft:stone" => block(Block::Stone),
            "minecraft:granite" => block(Block::Granite),
            "minecraft:polished_granite" => block(Block::PolishedGranite),
            "minecraft:diorite" => block(Block::Diorite),
            "minecraft:polished_diorite" => block(Block::PolishedDiorite),
            "minecraft:andesite" => block(Block::Andesite),
            "minecraft:polished_andesite" => block(Block::PolishedAndesite),
            "minecraft:grass_block" => block(Block::GrassBlock),
            "minecraft:dirt" => block(Block::Dirt),
            "minecraft:coarse_dirt" => block(Block::CoarseDirt),
            "minecraft:podzol" => block(Block::Podzol),
            "minecraft:cobblestone" => block(Block::Cobblestone),
            "minecraft:oak_planks" => block(Block::Planks { material: WoodMaterial::Oak }),
            "minecraft:spruce_planks" => block(Block::Planks { material: WoodMaterial::Spruce }),
            "minecraft:birch_planks" => block(Block::Planks { material: WoodMaterial::Birch }),
            "minecraft:jungle_planks" => block(Block::Planks { material: WoodMaterial::Jungle }),
            "minecraft:acacia_planks" => block(Block::Planks { material: WoodMaterial::Acacia }),
            "minecraft:dark_oak_planks" => block(Block::Planks { material: WoodMaterial::DarkOak }),
            "minecraft:oak_sapling" => block(sapling(SaplingMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_sapling" => block(sapling(SaplingMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_sapling" => block(sapling(SaplingMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_sapling" => block(sapling(SaplingMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_sapling" => block(sapling(SaplingMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_sapling" => block(sapling(SaplingMaterial::DarkOak, &properties.unwrap())),
            "minecraft:bedrock" => block(Block::Bedrock),
            "minecraft:water" => block(water(&properties.unwrap())),
            "minecraft:lava" => block(lava(&properties.unwrap())),
            "minecraft:sand" => block(Block::Sand),
            "minecraft:red_sand" => block(Block::RedSand),
            "minecraft:gravel" => block(Block::Gravel),
            "minecraft:gold_ore" => block(Block::GoldOre),
            "minecraft:iron_ore" => block(Block::IronOre),
            "minecraft:coal_ore" => block(Block::CoalOre),
            "minecraft:oak_log" => block(log(WoodMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_log" => block(log(WoodMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_log" => block(log(WoodMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_log" => block(log(WoodMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_log" => block(log(WoodMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_log" => block(log(WoodMaterial::DarkOak, &properties.unwrap())),
            "minecraft:stripped_oak_log" => block(stripped_log(WoodMaterial::Oak, &properties.unwrap())),
            "minecraft:stripped_spruce_log" => block(stripped_log(WoodMaterial::Spruce, &properties.unwrap())),
            "minecraft:stripped_birch_log" => block(stripped_log(WoodMaterial::Birch, &properties.unwrap())),
            "minecraft:stripped_jungle_log" => block(stripped_log(WoodMaterial::Jungle, &properties.unwrap())),
            "minecraft:stripped_acacia_log" => block(stripped_log(WoodMaterial::Acacia, &properties.unwrap())),
            "minecraft:stripped_dark_oak_log" => block(stripped_log(WoodMaterial::DarkOak, &properties.unwrap())),
            "minecraft:oak_wood" => block(wood(WoodMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_wood" => block(wood(WoodMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_wood" => block(wood(WoodMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_wood" => block(wood(WoodMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_wood" => block(wood(WoodMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_wood" => block(wood(WoodMaterial::DarkOak, &properties.unwrap())),
            "minecraft:stripped_oak_wood" => block(stripped_wood(WoodMaterial::Oak, &properties.unwrap())),
            "minecraft:stripped_spruce_wood" => block(stripped_wood(WoodMaterial::Spruce, &properties.unwrap())),
            "minecraft:stripped_birch_wood" => block(stripped_wood(WoodMaterial::Birch, &properties.unwrap())),
            "minecraft:stripped_jungle_wood" => block(stripped_wood(WoodMaterial::Jungle, &properties.unwrap())),
            "minecraft:stripped_acacia_wood" => block(stripped_wood(WoodMaterial::Acacia, &properties.unwrap())),
            "minecraft:stripped_dark_oak_wood" => block(stripped_wood(WoodMaterial::DarkOak, &properties.unwrap())),
            "minecraft:oak_leaves" => block(leaves(LeavesMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_leaves" => block(leaves(LeavesMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_leaves" => block(leaves(LeavesMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_leaves" => block(leaves(LeavesMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_leaves" => block(leaves(LeavesMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_leaves" => block(leaves(LeavesMaterial::DarkOak, &properties.unwrap())),
            "minecraft:sponge" => block(Block::Sponge),
            "minecraft:wet_sponge" => block(Block::WetSponge),
            "minecraft:glass" => block(Block::Glass { colour: None }),
            "minecraft:lapis_ore" => block(Block::LapisLazuliOre),
            "minecraft:lapis_block" => block(Block::LapisLazuliBlock),
            "minecraft:dispenser" => proto(proto_dispenser(&properties.unwrap())),
            "minecraft:sandstone" => block(Block::Sandstone),
            "minecraft:chiseled_sandstone" => block(Block::ChiseledSandstone),
            "minecraft:smooth_sandstone" => block(Block::SmoothSandstone),
            "minecraft:cut_sandstone" => block(Block::CutSandstone),
            "minecraft:note_block" => block(noteblock(&properties.unwrap())),
            "minecraft:powered_rail" => block(rail(RailType::Powered, &properties.unwrap())),
            "minecraft:detector_rail" => block(rail(RailType::Detector, &properties.unwrap())),
            "minecraft:rail" => block(rail(RailType::Normal, &properties.unwrap())),
            "minecraft:activator_rail" => block(rail(RailType::Activator, &properties.unwrap())),
            "minecraft:sticky_piston" => block(piston(true, &properties.unwrap())),
            "minecraft:piston_head" => block(piston_head(&properties.unwrap())),
            "minecraft:piston" => block(piston(false, &properties.unwrap())),
            "minecraft:cobweb" => block(Block::Cobweb),
            "minecraft:grass" => block(Block::Grass(Grass::Grass)),
            "minecraft:fern" => block(Block::Grass(Grass::Fern)),
            "minecraft:dead_bush" => block(Block::DeadBush),
            "minecraft:white_wool" => block(Block::Wool { colour: Colour::White }),
            "minecraft:orange_wool" => block(Block::Wool { colour: Colour::Orange }),
            "minecraft:magenta_wool" => block(Block::Wool { colour: Colour::Magenta }),
            "minecraft:light_blue_wool" => block(Block::Wool { colour: Colour::LightBlue }),
            "minecraft:yellow_wool" => block(Block::Wool { colour: Colour::Yellow }),
            "minecraft:lime_wool" => block(Block::Wool { colour: Colour::Lime }),
            "minecraft:pink_wool" => block(Block::Wool { colour: Colour::Pink }),
            "minecraft:gray_wool" => block(Block::Wool { colour: Colour::Gray }),
            "minecraft:light_gray_wool" => block(Block::Wool { colour: Colour::LightGray }),
            "minecraft:cyan_wool" => block(Block::Wool { colour: Colour::Cyan }),
            "minecraft:purple_wool" => block(Block::Wool { colour: Colour::Purple }),
            "minecraft:blue_wool" => block(Block::Wool { colour: Colour::Blue }),
            "minecraft:brown_wool" => block(Block::Wool { colour: Colour::Brown }),
            "minecraft:green_wool" => block(Block::Wool { colour: Colour::Green }),
            "minecraft:red_wool" => block(Block::Wool { colour: Colour::Red }),
            "minecraft:black_wool" => block(Block::Wool { colour: Colour::Black }),
            // TODO block 36 piston_extension ("Block moved by Piston")
            "minecraft:dandelion" => block(Block::Flower(Flower::Dandelion)),
            "minecraft:poppy" => block(Block::Flower(Flower::Poppy)),
            "minecraft:blue_orchid" => block(Block::Flower(Flower::BlueOrchid)),
            "minecraft:allium" => block(Block::Flower(Flower::Allium)),
            "minecraft:azure_bluet" => block(Block::Flower(Flower::AzureBluet)),
            "minecraft:red_tulip" => block(Block::Flower(Flower::TulipRed)),
            "minecraft:orange_tulip" => block(Block::Flower(Flower::TulipOrange)),
            "minecraft:white_tulip" => block(Block::Flower(Flower::TulipWhite)),
            "minecraft:pink_tulip" => block(Block::Flower(Flower::TulipPink)),
            "minecraft:oxeye_daisy" => block(Block::Flower(Flower::OxeyeDaisy)),
            "minecraft:cornflower" => block(Block::Flower(Flower::Cornflower)),
            "minecraft:lily_of_the_valley" => block(Block::Flower(Flower::LilyOfTheValley)),
            "minecraft:wither_rose" => block(Block::Flower(Flower::WitherRose)),
            "minecraft:brown_mushroom" => block(Block::BrownMushroom),
            "minecraft:red_mushroom" => block(Block::RedMushroom),
            "minecraft:gold_block" => block(Block::BlockOfGold),
            "minecraft:iron_block" => block(Block::BlockOfIron),
            "minecraft:oak_slab" => block(slab(SlabMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_slab" => block(slab(SlabMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_slab" => block(slab(SlabMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_slab" => block(slab(SlabMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_slab" => block(slab(SlabMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_slab" => block(slab(SlabMaterial::DarkOak, &properties.unwrap())),
            "minecraft:stone_slab" => block(slab(SlabMaterial::Stone, &properties.unwrap())),
            "minecraft:smooth_stone_slab" => block(slab(SlabMaterial::SmoothStone, &properties.unwrap())),
            "minecraft:sandstone_slab" => block(slab(SlabMaterial::Sandstone, &properties.unwrap())),
            "minecraft:petrified_oak_slab" => block(slab(SlabMaterial::PetrifiedOak, &properties.unwrap())),
            "minecraft:cobblestone_slab" => block(slab(SlabMaterial::Cobblestone, &properties.unwrap())),
            "minecraft:brick_slab" => block(slab(SlabMaterial::Brick, &properties.unwrap())),
            "minecraft:stone_brick_slab" => block(slab(SlabMaterial::StoneBrick, &properties.unwrap())),
            "minecraft:nether_brick_slab" => block(slab(SlabMaterial::NetherBrick, &properties.unwrap())),
            "minecraft:quartz_slab" => block(slab(SlabMaterial::Quartz, &properties.unwrap())),
            "minecraft:red_sandstone_slab" => block(slab(SlabMaterial::RedSandstone, &properties.unwrap())),
            "minecraft:purpur_slab" => block(slab(SlabMaterial::Purpur, &properties.unwrap())),
            "minecraft:prismarine_slab" => block(slab(SlabMaterial::Prismarine, &properties.unwrap())),
            "minecraft:prismarine_brick_slab" => block(slab(SlabMaterial::PrismarineBrick, &properties.unwrap())),
            "minecraft:dark_prismarine_slab" => block(slab(SlabMaterial::DarkPrismarine, &properties.unwrap())),
            "minecraft:smooth_quartz" => block(Block::SmoothQuartz),
            "minecraft:smooth_stone" => block(Block::SmoothStone),
            "minecraft:bricks" => block(Block::BrickBlock),
            "minecraft:tnt" => block(Block::TNT),
            "minecraft:bookshelf" => block(Block::Bookshelf),
            "minecraft:mossy_cobblestone" => block(Block::MossyCobblestone),
            "minecraft:obsidian" => block(Block::Obsidian),
            "minecraft:torch" => block(Block::Torch { attached: Surface5::Down }),
            "minecraft:wall_torch" => block(wall_torch(&properties.unwrap())),
            "minecraft:redstone_torch" => block(Block::RedstoneTorch { attached: Surface5::Down }),
            "minecraft:redstone_wall_torch" => block(redstone_wall_torch(&properties.unwrap())),
            "minecraft:soul_torch" => block(Block::SoulTorch { attached: Surface5::Down }),
            "minecraft:soul_wall_torch" => block(soul_wall_torch(&properties.unwrap())),
            "minecraft:fire" => block(fire(&properties.unwrap())),
            // TODO block 52 / minecraft:spawner / mob spawner
            "minecraft:oak_stairs" => block(stairs(StairMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_stairs" => block(stairs(StairMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_stairs" => block(stairs(StairMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_stairs" => block(stairs(StairMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_stairs" => block(stairs(StairMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_stairs" => block(stairs(StairMaterial::DarkOak, &properties.unwrap())),
            "minecraft:cobblestone_stairs" => block(stairs(StairMaterial::Cobblestone, &properties.unwrap())),
            "minecraft:brick_stairs" => block(stairs(StairMaterial::Brick, &properties.unwrap())),
            "minecraft:stone_brick_stairs" => block(stairs(StairMaterial::StoneBrick, &properties.unwrap())),
            "minecraft:nether_brick_stairs" => block(stairs(StairMaterial::NetherBrick, &properties.unwrap())),
            "minecraft:sandstone_stairs" => block(stairs(StairMaterial::Sandstone, &properties.unwrap())),
            "minecraft:quartz_stairs" => block(stairs(StairMaterial::Quartz, &properties.unwrap())),
            "minecraft:red_sandstone_stairs" => block(stairs(StairMaterial::RedSandstone, &properties.unwrap())),
            "minecraft:purpur_stairs" => block(stairs(StairMaterial::Purpur, &properties.unwrap())),
            "minecraft:dark_prismarine_stairs" => block(stairs(StairMaterial::DarkPrismarine, &properties.unwrap())),
            "minecraft:smooth_sandstone_stairs" => block(stairs(StairMaterial::SmoothSandstone, &properties.unwrap())),
            "minecraft:polished_blackstone_brick_stairs" => block(stairs(StairMaterial::PolishedBlackstoneBrick, &properties.unwrap())),
            "minecraft:prismarine_brick_stairs" => block(stairs(StairMaterial::PrismarineBrick, &properties.unwrap())),
            "minecraft:stone_stairs" => block(stairs(StairMaterial::Stone, &properties.unwrap())),
            "minecraft:polished_blackstone_stairs" => block(stairs(StairMaterial::PolishedBlackstone, &properties.unwrap())),
            "minecraft:prismarine_stairs" => block(stairs(StairMaterial::Prismarine, &properties.unwrap())),
            "minecraft:end_stone_brick_stairs" => block(stairs(StairMaterial::EndStoneBrick, &properties.unwrap())),
            "minecraft:blackstone_stairs" => block(stairs(StairMaterial::Blackstone, &properties.unwrap())),
            "minecraft:mossy_cobblestone_stairs" => block(stairs(StairMaterial::MossyCobblestone, &properties.unwrap())),
            "minecraft:diorite_stairs" => block(stairs(StairMaterial::Diorite, &properties.unwrap())),
            "minecraft:polished_diorite_stairs" => block(stairs(StairMaterial::PolishedDiorite, &properties.unwrap())),
            "minecraft:polished_andesite_stairs" => block(stairs(StairMaterial::PolishedAndesite, &properties.unwrap())),
            "minecraft:mossy_stone_brick_stairs" => block(stairs(StairMaterial::MossyStoneBrick, &properties.unwrap())),
            "minecraft:red_nether_brick_stairs" => block(stairs(StairMaterial::RedNetherBrick, &properties.unwrap())),
            "minecraft:warped_stairs" => block(stairs(StairMaterial::Warped, &properties.unwrap())),
            "minecraft:smooth_red_sandstone_stairs" => block(stairs(StairMaterial::SmoothRedSandstone, &properties.unwrap())),
            "minecraft:andesite_stairs" => block(stairs(StairMaterial::Andesite, &properties.unwrap())),
            "minecraft:crimson_stairs" => block(stairs(StairMaterial::Crimson, &properties.unwrap())),
            "minecraft:polished_granite_stairs" => block(stairs(StairMaterial::PolishedGranite, &properties.unwrap())),
            "minecraft:granite_stairs" => block(stairs(StairMaterial::Granite, &properties.unwrap())),
            "minecraft:smooth_quartz_stairs" => block(stairs(StairMaterial::SmoothQuartz, &properties.unwrap())),
            "minecraft:chest" => proto(proto_chest(&properties.unwrap())),
            "minecraft:redstone_wire" => block(Block::RedstoneWire),
            "minecraft:diamond_ore" => block(Block::DiamondOre),
            "minecraft:diamond_block" => block(Block::BlockOfDiamond),
            "minecraft:crafting_table" => block(Block::CraftingTable),
            "minecraft:wheat" => block(Block::Wheat { growth_stage: age0_7(&properties.unwrap()) }),
            "minecraft:farmland" => block(Block::Farmland { wetness: moisture0_7(&properties.unwrap()) }),
            "minecraft:furnace" => proto(proto_furnace(&properties.unwrap())),
            "minecraft:oak_sign" => proto(proto_sign(WoodMaterial::Oak, &properties.unwrap())),
            "minecraft:oak_wall_sign" => proto(proto_wall_sign(WoodMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_sign" => proto(proto_sign(WoodMaterial::Spruce, &properties.unwrap())),
            "minecraft:spruce_wall_sign" => proto(proto_wall_sign(WoodMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_sign" => proto(proto_sign(WoodMaterial::Birch, &properties.unwrap())),
            "minecraft:birch_wall_sign" => proto(proto_wall_sign(WoodMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_sign" => proto(proto_sign(WoodMaterial::Jungle, &properties.unwrap())),
            "minecraft:jungle_wall_sign" => proto(proto_wall_sign(WoodMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_sign" => proto(proto_sign(WoodMaterial::Acacia, &properties.unwrap())),
            "minecraft:acacia_wall_sign" => proto(proto_wall_sign(WoodMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_sign" => proto(proto_sign(WoodMaterial::DarkOak, &properties.unwrap())),
            "minecraft:dark_oak_wall_sign" => proto(proto_wall_sign(WoodMaterial::DarkOak, &properties.unwrap())),
            "minecraft:crimson_sign" => proto(proto_sign(WoodMaterial::Crimson, &properties.unwrap())),
            "minecraft:crimson_wall_sign" => proto(proto_wall_sign(WoodMaterial::Crimson, &properties.unwrap())),
            "minecraft:warped_sign" => proto(proto_sign(WoodMaterial::Warped, &properties.unwrap())),
            "minecraft:warped_wall_sign" => proto(proto_wall_sign(WoodMaterial::Warped, &properties.unwrap())),
            "minecraft:oak_door" => block(door(DoorMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_door" => block(door(DoorMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_door" => block(door(DoorMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_door" => block(door(DoorMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_door" => block(door(DoorMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_door" => block(door(DoorMaterial::DarkOak, &properties.unwrap())),
            "minecraft:crimson_door" => block(door(DoorMaterial::Crimson, &properties.unwrap())),
            "minecraft:warped_door" => block(door(DoorMaterial::Warped, &properties.unwrap())),
            "minecraft:iron_door" => block(door(DoorMaterial::Iron, &properties.unwrap())),
            "minecraft:ladder" => block(ladder(&properties.unwrap())),
            "minecraft:lever" => block(lever(&properties.unwrap())),
            "minecraft:oak_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Oak)),
            "minecraft:spruce_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Spruce)),
            "minecraft:birch_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Birch)),
            "minecraft:jungle_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Jungle)),
            "minecraft:acacia_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Acacia)),
            "minecraft:dark_oak_pressure_plate" => block(pressure_plate(PressurePlateMaterial::DarkOak)),
            "minecraft:crimson_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Crimson)),
            "minecraft:warped_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Warped)),
            "minecraft:stone_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Stone)),
            "minecraft:polished_blackstone_pressure_plate" => block(pressure_plate(PressurePlateMaterial::PolishedBlackstone)),
            "minecraft:heavy_weighted_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Iron)),
            "minecraft:light_weighted_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Gold)),
            "minecraft:redstone_ore" => block(Block::RedstoneOre),
            "minecraft:oak_button" => block(button(ButtonMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_button" => block(button(ButtonMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_button" => block(button(ButtonMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_button" => block(button(ButtonMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_button" => block(button(ButtonMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_button" => block(button(ButtonMaterial::DarkOak, &properties.unwrap())),
            "minecraft:crimson_button" => block(button(ButtonMaterial::Crimson, &properties.unwrap())),
            "minecraft:warped_button" => block(button(ButtonMaterial::Warped, &properties.unwrap())),
            "minecraft:stone_button" => block(button(ButtonMaterial::Stone, &properties.unwrap())),
            "minecraft:polished_blackstone_button" => block(button(ButtonMaterial::PolishedBlackstone, &properties.unwrap())),
            "minecraft:snow" => block(snow(&properties.unwrap())),
            "minecraft:ice" => block(Block::Ice),
            "minecraft:packed_ice" => block(Block::PackedIce),
            "minecraft:blue_ice" => block(Block::BlueIce),
            "minecraft:snow_block" => block(Block::SnowBlock),
            "minecraft:cactus" => block(cactus(&properties.unwrap())),
            "minecraft:clay" => block(Block::Clay),
            "minecraft:sugar_cane" => block(sugar_cane(&properties.unwrap())),
            "minecraft:jukebox" => jukebox(&properties.unwrap()),
            "minecraft:oak_fence" => block(fence(FenceMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_fence" => block(fence(FenceMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_fence" => block(fence(FenceMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_fence" => block(fence(FenceMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_fence" => block(fence(FenceMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_fence" => block(fence(FenceMaterial::DarkOak, &properties.unwrap())),
            "minecraft:crimson_fence" => block(fence(FenceMaterial::Crimson, &properties.unwrap())),
            "minecraft:warped_fence" => block(fence(FenceMaterial::Warped, &properties.unwrap())),
            "minecraft:nether_brick_fence" => block(fence(FenceMaterial::NetherBrick, &properties.unwrap())),
            "minecraft:pumpkin" => block(Block::Pumpkin),
            "minecraft:carved_pumpkin" => block(carved_pumpkin(&properties.unwrap())),
            "minecraft:netherrack" => block(Block::Netherrack),
            "minecraft:soul_sand" => block(Block::SoulSand),
            "minecraft:glowstone" => block(Block::Glowstone),
            "minecraft:nether_portal" => block(nether_portal(&properties.unwrap())),
            "minecraft:jack_o_lantern" => block(jack_o_lantern(&properties.unwrap())),
            "minecraft:cake" => block(cake(&properties.unwrap())),
            "minecraft:repeater" => block(repeater(&properties.unwrap())),
            "minecraft:white_stained_glass" => block(Block::Glass { colour: Some(Colour::White )}),
            "minecraft:orange_stained_glass" => block(Block::Glass { colour: Some(Colour::Orange )}),
            "minecraft:magenta_stained_glass" => block(Block::Glass { colour: Some(Colour::Magenta )}),
            "minecraft:light_blue_stained_glass" => block(Block::Glass { colour: Some(Colour::LightBlue )}),
            "minecraft:yellow_stained_glass" => block(Block::Glass { colour: Some(Colour::Yellow )}),
            "minecraft:lime_stained_glass" => block(Block::Glass { colour: Some(Colour::Lime )}),
            "minecraft:pink_stained_glass" => block(Block::Glass { colour: Some(Colour::Pink )}),
            "minecraft:gray_stained_glass" => block(Block::Glass { colour: Some(Colour::Gray )}),
            "minecraft:light_gray_stained_glass" => block(Block::Glass { colour: Some(Colour::LightGray )}),
            "minecraft:cyan_stained_glass" => block(Block::Glass { colour: Some(Colour::Cyan )}),
            "minecraft:purple_stained_glass" => block(Block::Glass { colour: Some(Colour::Purple )}),
            "minecraft:blue_stained_glass" => block(Block::Glass { colour: Some(Colour::Blue )}),
            "minecraft:brown_stained_glass" => block(Block::Glass { colour: Some(Colour::Brown )}),
            "minecraft:green_stained_glass" => block(Block::Glass { colour: Some(Colour::Green )}),
            "minecraft:red_stained_glass" => block(Block::Glass { colour: Some(Colour::Red )}),
            "minecraft:black_stained_glass" => block(Block::Glass { colour: Some(Colour::Black )}),
            "minecraft:oak_trapdoor" => block(trapdoor(DoorMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_trapdoor" => block(trapdoor(DoorMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_trapdoor" => block(trapdoor(DoorMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_trapdoor" => block(trapdoor(DoorMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_trapdoor" => block(trapdoor(DoorMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_trapdoor" => block(trapdoor(DoorMaterial::DarkOak, &properties.unwrap())),
            "minecraft:crimson_trapdoor" => block(trapdoor(DoorMaterial::Crimson, &properties.unwrap())),
            "minecraft:warped_trapdoor" => block(trapdoor(DoorMaterial::Warped, &properties.unwrap())),
            "minecraft:iron_trapdoor" => block(trapdoor(DoorMaterial::Iron, &properties.unwrap())),
            "minecraft:infested_stone" => block(Block::InfestedStone),
            "minecraft:infested_cobblestone" => block(Block::InfestedCobblestone),
            "minecraft:infested_stone_bricks" => block(Block::InfestedStoneBricks),
            "minecraft:infested_mossy_stone_bricks" => block(Block::InfestedMossyStoneBricks),
            "minecraft:infested_cracked_stone_bricks" => block(Block::InfestedCrackedStoneBricks),
            "minecraft:infested_chiseled_stone_bricks" => block(Block::InfestedChiseledStoneBricks),
            "minecraft:stone_bricks" => block(Block::StoneBricks),
            "minecraft:mossy_stone_bricks" => block(Block::MossyStoneBricks),
            "minecraft:cracked_stone_bricks" => block(Block::CrackedStoneBricks),
            "minecraft:chiseled_stone_bricks" => block(Block::ChiseledStoneBricks),
            "minecraft:brown_mushroom_block" => block(brown_mushroom_block(&properties.unwrap())),
            "minecraft:red_mushroom_block" => block(red_mushroom_block(&properties.unwrap())),
            "minecraft:mushroom_stem" => block(mushroom_stem(&properties.unwrap())),
            "minecraft:iron_bars" => block(Block::IronBars { waterlogged: waterlogged(&properties.unwrap()) }),
            "minecraft:glass_pane" => block(glass_pane(&properties.unwrap())),
            "minecraft:melon" => block(Block::Melon),
            "minecraft:pumpkin_stem" => block(pumpkin_stem(&properties.unwrap())),
            "minecraft:attached_pumpkin_stem" => block(attached_pumpkin_stem(&properties.unwrap())),
            "minecraft:melon_stem" => block(melon_stem(&properties.unwrap())),
            "minecraft:attached_melon_stem" => block(attached_melon_stem(&properties.unwrap())),
            "minecraft:vine" => block(vine(&properties.unwrap())),
            "minecraft:oak_fence_gate" => block(fence_gate(WoodMaterial::Oak, &properties.unwrap())),
            "minecraft:spruce_fence_gate" => block(fence_gate(WoodMaterial::Spruce, &properties.unwrap())),
            "minecraft:birch_fence_gate" => block(fence_gate(WoodMaterial::Birch, &properties.unwrap())),
            "minecraft:jungle_fence_gate" => block(fence_gate(WoodMaterial::Jungle, &properties.unwrap())),
            "minecraft:acacia_fence_gate" => block(fence_gate(WoodMaterial::Acacia, &properties.unwrap())),
            "minecraft:dark_oak_fence_gate" => block(fence_gate(WoodMaterial::DarkOak, &properties.unwrap())),
            "minecraft:crimson_fence_gate" => block(fence_gate(WoodMaterial::Crimson, &properties.unwrap())),
            "minecraft:warped_fence_gate" => block(fence_gate(WoodMaterial::Warped, &properties.unwrap())),
            "minecraft:mycelium" => block(Block::Mycelium),
            "minecraft:lily_pad" => block(Block::LilyPad),
            "minecraft:nether_bricks" => block(Block::NetherBricks),
            "minecraft:nether_wart" => block(nether_wart(&properties.unwrap())),
            "minecraft:enchanting_table" => proto(ProtoBlock::EnchantingTable),
            "minecraft:brewing_stand" => proto(ProtoBlock::BrewingStand),
            "minecraft:cauldron" => block(cauldron(&properties.unwrap())),
            "minecraft:end_portal" => block(Block::EndPortal),
            "minecraft:end_portal_frame" => block(end_portal_frame(&properties.unwrap())),
            "minecraft:end_stone" => block(Block::EndStone),
            "minecraft:dragon_egg" => block(Block::DragonEgg),
            "minecraft:redstone_lamp" => block(Block::RedstoneLamp),
            "minecraft:cocoa" => block(cocoa(&properties.unwrap())),
            "minecraft:emerald_ore" => block(Block::EmeraldOre),
            "minecraft:ender_chest" => block(ender_chest(&properties.unwrap())),
            "minecraft:tripwire_hook" => block(tripwire_hook(&properties.unwrap())),
            "minecraft:tripwire" => block(Block::Tripwire),
            "minecraft:emerald_block" => block(Block::BlockOfEmerald),
            // TODO 137 command block // Deferred for now, too complicated
            "minecraft:beacon" => proto(ProtoBlock::Beacon),
            "minecraft:cobblestone_wall" => block(wall(WallMaterial::Cobblestone, &properties.unwrap())),
            "minecraft:mossy_cobblestone_wall" => block(wall(WallMaterial::MossyCobblestone, &properties.unwrap())),
            "minecraft:brick_wall" => block(wall(WallMaterial::Brick, &properties.unwrap())),
            "minecraft:prismarine_wall" => block(wall(WallMaterial::Prismarine, &properties.unwrap())),
            "minecraft:red_sandstone_wall" => block(wall(WallMaterial::RedSandstone, &properties.unwrap())),
            "minecraft:mossy_stone_brick_wall" => block(wall(WallMaterial::MossyStoneBrick, &properties.unwrap())),
            "minecraft:granite_wall" => block(wall(WallMaterial::Granite, &properties.unwrap())),
            "minecraft:stone_brick_wall" => block(wall(WallMaterial::StoneBrick, &properties.unwrap())),
            "minecraft:nether_brick_wall" => block(wall(WallMaterial::NetherBrick, &properties.unwrap())),
            "minecraft:andesite_wall" => block(wall(WallMaterial::Andesite, &properties.unwrap())),
            "minecraft:red_nether_brick_wall" => block(wall(WallMaterial::RedNetherBrick, &properties.unwrap())),
            "minecraft:sandstone_wall" => block(wall(WallMaterial::Sandstone, &properties.unwrap())),
            "minecraft:end_stone_brick_wall" => block(wall(WallMaterial::EndStoneBrick, &properties.unwrap())),
            "minecraft:diorite_wall" => block(wall(WallMaterial::Diorite, &properties.unwrap())),
            "minecraft:blackstone_wall" => block(wall(WallMaterial::Blackstone, &properties.unwrap())),
            "minecraft:polished_blackstone_wall" => block(wall(WallMaterial::PolishedBlackstone, &properties.unwrap())),
            "minecraft:polished_blackstone_brick_wall" => block(wall(WallMaterial::PolishedBlackstoneBrick, &properties.unwrap())),
            "minecraft:flower_pot" => block(Block::FlowerPot(FlowerPot { plant: None })),
            "minecraft:potted_dandelion" => block(potted_plant(PottedPlant::Dandelion)),
            "minecraft:potted_poppy" => block(potted_plant(PottedPlant::Poppy)),
            "minecraft:potted_blue_orchid" => block(potted_plant(PottedPlant::BlueOrchid)),
            "minecraft:potted_allium" => block(potted_plant(PottedPlant::Allium)),
            "minecraft:potted_azure_bluet" => block(potted_plant(PottedPlant::AzureBluet)),
            "minecraft:potted_red_tulip" => block(potted_plant(PottedPlant::TulipRed)),
            "minecraft:potted_orange_tulip" => block(potted_plant(PottedPlant::TulipOrange)),
            "minecraft:potted_white_tulip" => block(potted_plant(PottedPlant::TulipWhite)),
            "minecraft:potted_pink_tulip" => block(potted_plant(PottedPlant::TulipPink)),
            "minecraft:potted_oxeye_daisy" => block(potted_plant(PottedPlant::OxeyeDaisy)),
            "minecraft:potted_cornflower" => block(potted_plant(PottedPlant::Cornflower)),
            "minecraft:potted_lily_of_the_valley" => block(potted_plant(PottedPlant::LilyOfTheValley)),
            "minecraft:potted_wither_rose" => block(potted_plant(PottedPlant::WitherRose)),
            "minecraft:potted_oak_sapling" => block(potted_plant(PottedPlant::OakSapling)),
            "minecraft:potted_spruce_sapling" => block(potted_plant(PottedPlant::SpruceSapling)),
            "minecraft:potted_birch_sapling" => block(potted_plant(PottedPlant::BirchSapling)),
            "minecraft:potted_jungle_sapling" => block(potted_plant(PottedPlant::JungleSapling)),
            "minecraft:potted_acacia_sapling" => block(potted_plant(PottedPlant::AcaciaSapling)),
            "minecraft:potted_dark_oak_sapling" => block(potted_plant(PottedPlant::DarkOakSapling)),
            "minecraft:potted_red_mushroom" => block(potted_plant(PottedPlant::RedMushroom)),
            "minecraft:potted_brown_mushroom" => block(potted_plant(PottedPlant::BrownMushroom)),
            "minecraft:potted_fern" => block(potted_plant(PottedPlant::Fern)),
            "minecraft:potted_dead_bush" => block(potted_plant(PottedPlant::DeadBush)),
            "minecraft:potted_cactus" => block(potted_plant(PottedPlant::Cactus)),
            "minecraft:potted_bamboo" => block(potted_plant(PottedPlant::Bamboo)),
            "minecraft:potted_crimson_fungus" => block(potted_plant(PottedPlant::CrimsonFungus)),
            "minecraft:potted_warped_fungus" => block(potted_plant(PottedPlant::WarpedFungus)),
            "minecraft:potted_crimson_roots" => block(potted_plant(PottedPlant::CrimsonRoots)),
            "minecraft:potted_warped_roots" => block(potted_plant(PottedPlant::WarpedRoots)),
            "minecraft:carrots" => block(Block::Carrots { growth_stage: age0_7(&properties.unwrap()) }),
            "minecraft:potatoes" => block(Block::Potatoes { growth_stage: age0_7(&properties.unwrap()) }),
            "minecraft:skeleton_skull" => block(head(HeadVariant::SkeletonSkull, &properties.unwrap())),
            "minecraft:wither_skeleton_skull" => block(head(HeadVariant::WitherSkeletonSkull, &properties.unwrap())),
            "minecraft:player_head" => block(head(HeadVariant::PlayerHead, &properties.unwrap())),
            "minecraft:zombie_head" => block(head(HeadVariant::ZombieHead, &properties.unwrap())),
            "minecraft:creeper_head" => block(head(HeadVariant::CreeperHead, &properties.unwrap())),
            "minecraft:dragon_head" => block(head(HeadVariant::DragonHead, &properties.unwrap())),
            "minecraft:skeleton_wall_skull" => block(wall_head(HeadVariant::SkeletonSkull, &properties.unwrap())),
            "minecraft:wither_skeleton_wall_skull" => block(wall_head(HeadVariant::WitherSkeletonSkull, &properties.unwrap())),
            "minecraft:player_wall_head" => block(wall_head(HeadVariant::PlayerHead, &properties.unwrap())),
            "minecraft:zombie_wall_head" => block(wall_head(HeadVariant::ZombieHead, &properties.unwrap())),
            "minecraft:creeper_wall_head" => block(wall_head(HeadVariant::CreeperHead, &properties.unwrap())),
            "minecraft:dragon_wall_head" => block(wall_head(HeadVariant::DragonHead, &properties.unwrap())),
            "minecraft:anvil" => block(anvil(AnvilDamage::Intact, &properties.unwrap())),
            "minecraft:chipped_anvil" => block(anvil(AnvilDamage::SlightlyDamaged, &properties.unwrap())),
            "minecraft:damaged_anvil" => block(anvil(AnvilDamage::VeryDamaged, &properties.unwrap())),
            "minecraft:trapped_chest" => proto(proto_trapped_chest(&properties.unwrap())),
            "minecraft:comparator" => block(comparator(&properties.unwrap())),
            "minecraft:daylight_detector" => block(daylight_detector(&properties.unwrap())),
            "minecraft:redstone_block" => block(Block::BlockOfRedstone),
            "minecraft:nether_quartz_ore" => block(Block::QuartzOre),
            "minecraft:hopper" => proto(proto_hopper(&properties.unwrap())),
            "minecraft:quartz_block" => block(Block::BlockOfQuartz),
            "minecraft:chiseled_quartz_block" => block(Block::ChiseledQuartzBlock),
            "minecraft:quartz_pillar" => block(quartz_pillar(&properties.unwrap())),
            "minecraft:dropper" => proto(proto_dropper(&properties.unwrap())),
            "minecraft:terracotta" => block(Block::Terracotta { colour: None }),
            "minecraft:white_terracotta" => block(Block::Terracotta { colour: Some(Colour::White) }),
            "minecraft:orange_terracotta" => block(Block::Terracotta { colour: Some(Colour::Orange) }),
            "minecraft:magenta_terracotta" => block(Block::Terracotta { colour: Some(Colour::Magenta) }),
            "minecraft:light_blue_terracotta" => block(Block::Terracotta { colour: Some(Colour::LightBlue) }),
            "minecraft:yellow_terracotta" => block(Block::Terracotta { colour: Some(Colour::Yellow) }),
            "minecraft:lime_terracotta" => block(Block::Terracotta { colour: Some(Colour::Lime) }),
            "minecraft:pink_terracotta" => block(Block::Terracotta { colour: Some(Colour::Pink) }),
            "minecraft:gray_terracotta" => block(Block::Terracotta { colour: Some(Colour::Gray) }),
            "minecraft:light_gray_terracotta" => block(Block::Terracotta { colour: Some(Colour::LightGray) }),
            "minecraft:cyan_terracotta" => block(Block::Terracotta { colour: Some(Colour::Cyan) }),
            "minecraft:purple_terracotta" => block(Block::Terracotta { colour: Some(Colour::Purple) }),
            "minecraft:blue_terracotta" => block(Block::Terracotta { colour: Some(Colour::Blue) }),
            "minecraft:brown_terracotta" => block(Block::Terracotta { colour: Some(Colour::Brown) }),
            "minecraft:green_terracotta" => block(Block::Terracotta { colour: Some(Colour::Green) }),
            "minecraft:red_terracotta" => block(Block::Terracotta { colour: Some(Colour::Red) }),
            "minecraft:black_terracotta" => block(Block::Terracotta { colour: Some(Colour::Black) }),
            "minecraft:white_stained_glass_pane" => block(stained_glass_pane(Colour::White, &properties.unwrap())),
            "minecraft:orange_stained_glass_pane" => block(stained_glass_pane(Colour::Orange, &properties.unwrap())),
            "minecraft:magenta_stained_glass_pane" => block(stained_glass_pane(Colour::Magenta, &properties.unwrap())),
            "minecraft:light_blue_stained_glass_pane" => block(stained_glass_pane(Colour::LightBlue, &properties.unwrap())),
            "minecraft:yellow_stained_glass_pane" => block(stained_glass_pane(Colour::Yellow, &properties.unwrap())),
            "minecraft:lime_stained_glass_pane" => block(stained_glass_pane(Colour::Lime, &properties.unwrap())),
            "minecraft:pink_stained_glass_pane" => block(stained_glass_pane(Colour::Pink, &properties.unwrap())),
            "minecraft:gray_stained_glass_pane" => block(stained_glass_pane(Colour::Gray, &properties.unwrap())),
            "minecraft:light_gray_stained_glass_pane" => block(stained_glass_pane(Colour::LightGray, &properties.unwrap())),
            "minecraft:cyan_stained_glass_pane" => block(stained_glass_pane(Colour::Cyan, &properties.unwrap())),
            "minecraft:purple_stained_glass_pane" => block(stained_glass_pane(Colour::Purple, &properties.unwrap())),
            "minecraft:blue_stained_glass_pane" => block(stained_glass_pane(Colour::Blue, &properties.unwrap())),
            "minecraft:brown_stained_glass_pane" => block(stained_glass_pane(Colour::Brown, &properties.unwrap())),
            "minecraft:green_stained_glass_pane" => block(stained_glass_pane(Colour::Green, &properties.unwrap())),
            "minecraft:red_stained_glass_pane" => block(stained_glass_pane(Colour::Red, &properties.unwrap())),
            "minecraft:black_stained_glass_pane" => block(stained_glass_pane(Colour::Black, &properties.unwrap())),
            "minecraft:slime_block" => block(Block::BlockOfSlime),
            "minecraft:barrier" => block(Block::Barrier),
            "minecraft:prismarine" => block(Block::Prismarine),
            "minecraft:prismarine_bricks" => block(Block::PrismarineBricks),
            "minecraft:dark_prismarine" => block(Block::DarkPrismarine),
            "minecraft:sea_lantern" => block(Block::SeaLantern),
            "minecraft:hay_block" => block(hay_bale(&properties.unwrap())),
            "minecraft:white_carpet" => block(Block::Carpet { colour: Colour::White }),
            "minecraft:orange_carpet" => block(Block::Carpet { colour: Colour::Orange }),
            "minecraft:magenta_carpet" => block(Block::Carpet { colour: Colour::Magenta }),
            "minecraft:light_blue_carpet" => block(Block::Carpet { colour: Colour::LightBlue }),
            "minecraft:yellow_carpet" => block(Block::Carpet { colour: Colour::Yellow }),
            "minecraft:lime_carpet" => block(Block::Carpet { colour: Colour::Lime }),
            "minecraft:pink_carpet" => block(Block::Carpet { colour: Colour::Pink }),
            "minecraft:gray_carpet" => block(Block::Carpet { colour: Colour::Gray }),
            "minecraft:light_gray_carpet" => block(Block::Carpet { colour: Colour::LightGray }),
            "minecraft:cyan_carpet" => block(Block::Carpet { colour: Colour::Cyan }),
            "minecraft:purple_carpet" => block(Block::Carpet { colour: Colour::Purple }),
            "minecraft:blue_carpet" => block(Block::Carpet { colour: Colour::Blue }),
            "minecraft:brown_carpet" => block(Block::Carpet { colour: Colour::Brown }),
            "minecraft:green_carpet" => block(Block::Carpet { colour: Colour::Green }),
            "minecraft:red_carpet" => block(Block::Carpet { colour: Colour::Red }),
            "minecraft:black_carpet" => block(Block::Carpet { colour: Colour::Black }),
            "minecraft:coal_block" => block(Block::BlockOfCoal),
            "minecraft:sunflower" => block(tall("sunflower", &properties.unwrap())),
            "minecraft:lilac" => block(tall("lilac", &properties.unwrap())),
            "minecraft:rose_bush" => block(tall("rose_bush", &properties.unwrap())),
            "minecraft:peony" => block(tall("peony", &properties.unwrap())),
            "minecraft:tall_grass" => block(tall("tall_grass", &properties.unwrap())),
            "minecraft:large_fern" => block(tall("large_fern", &properties.unwrap())),
            "minecraft:white_bed" => block(bed(Colour::White, &properties.unwrap())),
            "minecraft:orange_bed" => block(bed(Colour::Orange, &properties.unwrap())),
            "minecraft:magenta_bed" => block(bed(Colour::Magenta, &properties.unwrap())),
            "minecraft:light_blue_bed" => block(bed(Colour::LightBlue, &properties.unwrap())),
            "minecraft:yellow_bed" => block(bed(Colour::Yellow, &properties.unwrap())),
            "minecraft:lime_bed" => block(bed(Colour::Lime, &properties.unwrap())),
            "minecraft:pink_bed" => block(bed(Colour::Pink, &properties.unwrap())),
            "minecraft:gray_bed" => block(bed(Colour::Gray, &properties.unwrap())),
            "minecraft:light_gray_bed" => block(bed(Colour::LightGray, &properties.unwrap())),
            "minecraft:cyan_bed" => block(bed(Colour::Cyan, &properties.unwrap())),
            "minecraft:purple_bed" => block(bed(Colour::Purple, &properties.unwrap())),
            "minecraft:blue_bed" => block(bed(Colour::Blue, &properties.unwrap())),
            "minecraft:brown_bed" => block(bed(Colour::Brown, &properties.unwrap())),
            "minecraft:green_bed" => block(bed(Colour::Green, &properties.unwrap())),
            "minecraft:red_bed" => block(bed(Colour::Red, &properties.unwrap())),
            "minecraft:black_bed" => block(bed(Colour::Black, &properties.unwrap())),
            "minecraft:white_banner" => proto(proto_banner(Colour::White, &properties.unwrap())),
            "minecraft:orange_banner" => proto(proto_banner(Colour::Orange, &properties.unwrap())),
            "minecraft:magenta_banner" => proto(proto_banner(Colour::Magenta, &properties.unwrap())),
            "minecraft:light_blue_banner" => proto(proto_banner(Colour::LightBlue, &properties.unwrap())),
            "minecraft:yellow_banner" => proto(proto_banner(Colour::Yellow, &properties.unwrap())),
            "minecraft:lime_banner" => proto(proto_banner(Colour::Lime, &properties.unwrap())),
            "minecraft:pink_banner" => proto(proto_banner(Colour::Pink, &properties.unwrap())),
            "minecraft:gray_banner" => proto(proto_banner(Colour::Gray, &properties.unwrap())),
            "minecraft:light_gray_banner" => proto(proto_banner(Colour::LightGray, &properties.unwrap())),
            "minecraft:cyan_banner" => proto(proto_banner(Colour::Cyan, &properties.unwrap())),
            "minecraft:purple_banner" => proto(proto_banner(Colour::Purple, &properties.unwrap())),
            "minecraft:blue_banner" => proto(proto_banner(Colour::Blue, &properties.unwrap())),
            "minecraft:brown_banner" => proto(proto_banner(Colour::Brown, &properties.unwrap())),
            "minecraft:green_banner" => proto(proto_banner(Colour::Green, &properties.unwrap())),
            "minecraft:red_banner" => proto(proto_banner(Colour::Red, &properties.unwrap())),
            "minecraft:black_banner" => proto(proto_banner(Colour::Black, &properties.unwrap())),
            "minecraft:white_wall_banner" => proto(proto_wall_banner(Colour::White, &properties.unwrap())),
            "minecraft:orange_wall_banner" => proto(proto_wall_banner(Colour::Orange, &properties.unwrap())),
            "minecraft:magenta_wall_banner" => proto(proto_wall_banner(Colour::Magenta, &properties.unwrap())),
            "minecraft:light_blue_wall_banner" => proto(proto_wall_banner(Colour::LightBlue, &properties.unwrap())),
            "minecraft:yellow_wall_banner" => proto(proto_wall_banner(Colour::Yellow, &properties.unwrap())),
            "minecraft:lime_wall_banner" => proto(proto_wall_banner(Colour::Lime, &properties.unwrap())),
            "minecraft:pink_wall_banner" => proto(proto_wall_banner(Colour::Pink, &properties.unwrap())),
            "minecraft:gray_wall_banner" => proto(proto_wall_banner(Colour::Gray, &properties.unwrap())),
            "minecraft:light_gray_wall_banner" => proto(proto_wall_banner(Colour::LightGray, &properties.unwrap())),
            "minecraft:cyan_wall_banner" => proto(proto_wall_banner(Colour::Cyan, &properties.unwrap())),
            "minecraft:purple_wall_banner" => proto(proto_wall_banner(Colour::Purple, &properties.unwrap())),
            "minecraft:blue_wall_banner" => proto(proto_wall_banner(Colour::Blue, &properties.unwrap())),
            "minecraft:brown_wall_banner" => proto(proto_wall_banner(Colour::Brown, &properties.unwrap())),
            "minecraft:green_wall_banner" => proto(proto_wall_banner(Colour::Green, &properties.unwrap())),
            "minecraft:red_wall_banner" => proto(proto_wall_banner(Colour::Red, &properties.unwrap())),
            "minecraft:black_wall_banner" => proto(proto_wall_banner(Colour::Black, &properties.unwrap())),
            "minecraft:red_sandstone" => block(Block::RedSandstone),
            "minecraft:chiseled_red_sandstone" => block(Block::ChiseledRedSandstone),
            "minecraft:smooth_red_sandstone" => block(Block::SmoothRedSandstone),
            "minecraft:cut_red_sandstone" => block(Block::CutRedSandstone),
            "minecraft:end_rod" => block(Block::EndRod { facing: facing_surface6(&properties.unwrap())}),
            "minecraft:chorus_plant" => block(Block::ChorusPlant),
            "minecraft:chorus_flower" => block(chorus_flower(&properties.unwrap())),
            "minecraft:purpur_block" => block(Block::PurpurBlock),
            "minecraft:purpur_pillar" => block(purpur_pillar(&properties.unwrap())),
            "minecraft:end_stone_bricks" => block(Block::EndStoneBricks),
            "minecraft:beetroots" => block(Block::Beetroots { growth_stage: age0_3(&properties.unwrap()) }),
            "minecraft:grass_path" => block(Block::GrassPath),
            // TODO 209 EndGateway
            // TODO 210 repeating command block
            // TODO 211 chain command block
            // TODO 212 FrostedIce
            "minecraft:magma_block" => block(Block::MagmaBlock),
            "minecraft:nether_wart_block" => block(Block::NetherWartBlock),
            "minecraft:red_nether_bricks" => block(Block::RedNetherBricks),
            "minecraft:bone_block" => block(bone_block(&properties.unwrap())),
            // TODO 217 StructureVoid
            "minecraft:observer" => block(observer(&properties.unwrap())),
            "minecraft:shulker_box" => proto(proto_shulker_box(None, &properties.unwrap())),
            "minecraft:white_shulker_box" => proto(proto_shulker_box(Some(Colour::White), &properties.unwrap())),
            "minecraft:orange_shulker_box" => proto(proto_shulker_box(Some(Colour::Orange), &properties.unwrap())),
            "minecraft:magenta_shulker_box" => proto(proto_shulker_box(Some(Colour::Magenta), &properties.unwrap())),
            "minecraft:light_blue_shulker_box" => proto(proto_shulker_box(Some(Colour::LightBlue), &properties.unwrap())),
            "minecraft:yellow_shulker_box" => proto(proto_shulker_box(Some(Colour::Yellow), &properties.unwrap())),
            "minecraft:lime_shulker_box" => proto(proto_shulker_box(Some(Colour::Lime), &properties.unwrap())),
            "minecraft:pink_shulker_box" => proto(proto_shulker_box(Some(Colour::Pink), &properties.unwrap())),
            "minecraft:gray_shulker_box" => proto(proto_shulker_box(Some(Colour::Gray), &properties.unwrap())),
            "minecraft:light_gray_shulker_box" => proto(proto_shulker_box(Some(Colour::LightGray), &properties.unwrap())),
            "minecraft:cyan_shulker_box" => proto(proto_shulker_box(Some(Colour::Cyan), &properties.unwrap())),
            "minecraft:purple_shulker_box" => proto(proto_shulker_box(Some(Colour::Purple), &properties.unwrap())),
            "minecraft:blue_shulker_box" => proto(proto_shulker_box(Some(Colour::Blue), &properties.unwrap())),
            "minecraft:brown_shulker_box" => proto(proto_shulker_box(Some(Colour::Brown), &properties.unwrap())),
            "minecraft:green_shulker_box" => proto(proto_shulker_box(Some(Colour::Green), &properties.unwrap())),
            "minecraft:red_shulker_box" => proto(proto_shulker_box(Some(Colour::Red), &properties.unwrap())),
            "minecraft:black_shulker_box" => proto(proto_shulker_box(Some(Colour::Black), &properties.unwrap())),
            "minecraft:white_glazed_terracotta" => block(glazed_terracotta(Colour::White, &properties.unwrap())),
            "minecraft:orange_glazed_terracotta" => block(glazed_terracotta(Colour::Orange, &properties.unwrap())),
            "minecraft:magenta_glazed_terracotta" => block(glazed_terracotta(Colour::Magenta, &properties.unwrap())),
            "minecraft:light_blue_glazed_terracotta" => block(glazed_terracotta(Colour::LightBlue, &properties.unwrap())),
            "minecraft:yellow_glazed_terracotta" => block(glazed_terracotta(Colour::Yellow, &properties.unwrap())),
            "minecraft:lime_glazed_terracotta" => block(glazed_terracotta(Colour::Lime, &properties.unwrap())),
            "minecraft:pink_glazed_terracotta" => block(glazed_terracotta(Colour::Pink, &properties.unwrap())),
            "minecraft:gray_glazed_terracotta" => block(glazed_terracotta(Colour::Gray, &properties.unwrap())),
            "minecraft:light_gray_glazed_terracotta" => block(glazed_terracotta(Colour::LightGray, &properties.unwrap())),
            "minecraft:cyan_glazed_terracotta" => block(glazed_terracotta(Colour::Cyan, &properties.unwrap())),
            "minecraft:purple_glazed_terracotta" => block(glazed_terracotta(Colour::Purple, &properties.unwrap())),
            "minecraft:blue_glazed_terracotta" => block(glazed_terracotta(Colour::Blue, &properties.unwrap())),
            "minecraft:brown_glazed_terracotta" => block(glazed_terracotta(Colour::Brown, &properties.unwrap())),
            "minecraft:green_glazed_terracotta" => block(glazed_terracotta(Colour::Green, &properties.unwrap())),
            "minecraft:red_glazed_terracotta" => block(glazed_terracotta(Colour::Red, &properties.unwrap())),
            "minecraft:black_glazed_terracotta" => block(glazed_terracotta(Colour::Black, &properties.unwrap())),
            "minecraft:white_concrete" => block(concrete(Colour::White)),
            "minecraft:orange_concrete" => block(concrete(Colour::Orange)),
            "minecraft:magenta_concrete" => block(concrete(Colour::Magenta)),
            "minecraft:light_blue_concrete" => block(concrete(Colour::LightBlue)),
            "minecraft:yellow_concrete" => block(concrete(Colour::Yellow)),
            "minecraft:lime_concrete" => block(concrete(Colour::Lime)),
            "minecraft:pink_concrete" => block(concrete(Colour::Pink)),
            "minecraft:gray_concrete" => block(concrete(Colour::Gray)),
            "minecraft:light_gray_concrete" => block(concrete(Colour::LightGray)),
            "minecraft:cyan_concrete" => block(concrete(Colour::Cyan)),
            "minecraft:purple_concrete" => block(concrete(Colour::Purple)),
            "minecraft:blue_concrete" => block(concrete(Colour::Blue)),
            "minecraft:brown_concrete" => block(concrete(Colour::Brown)),
            "minecraft:green_concrete" => block(concrete(Colour::Green)),
            "minecraft:red_concrete" => block(concrete(Colour::Red)),
            "minecraft:black_concrete" => block(concrete(Colour::Black)),
            "minecraft:white_concrete_powder" => block(concrete_powder(Colour::White)),
            "minecraft:orange_concrete_powder" => block(concrete_powder(Colour::Orange)),
            "minecraft:magenta_concrete_powder" => block(concrete_powder(Colour::Magenta)),
            "minecraft:light_blue_concrete_powder" => block(concrete_powder(Colour::LightBlue)),
            "minecraft:yellow_concrete_powder" => block(concrete_powder(Colour::Yellow)),
            "minecraft:lime_concrete_powder" => block(concrete_powder(Colour::Lime)),
            "minecraft:pink_concrete_powder" => block(concrete_powder(Colour::Pink)),
            "minecraft:gray_concrete_powder" => block(concrete_powder(Colour::Gray)),
            "minecraft:light_gray_concrete_powder" => block(concrete_powder(Colour::LightGray)),
            "minecraft:cyan_concrete_powder" => block(concrete_powder(Colour::Cyan)),
            "minecraft:purple_concrete_powder" => block(concrete_powder(Colour::Purple)),
            "minecraft:blue_concrete_powder" => block(concrete_powder(Colour::Blue)),
            "minecraft:brown_concrete_powder" => block(concrete_powder(Colour::Brown)),
            "minecraft:green_concrete_powder" => block(concrete_powder(Colour::Green)),
            "minecraft:red_concrete_powder" => block(concrete_powder(Colour::Red)),
            "minecraft:black_concrete_powder" => block(concrete_powder(Colour::Black)),
            // TODO 255 structure block
            _ => block(Block::Unknown(None)),
        };
        palette.push(palette_item);
    }
    println!("Palette: {:#?}", palette);

    Some(palette)
}

/// Convenience function for wrapping a Block in a PaletteItem.
fn block(block: Block) -> PaletteItem {
    PaletteItem::Block(block)
}

/// Convenience function for wrapping a ProtoBlock into a PaletteItem.
fn proto(proto_block: ProtoBlock) -> PaletteItem {
    PaletteItem::ProtoBlock(proto_block)
}

//
// Convenience functions for blocks
//

fn sapling(material: SaplingMaterial, properties: &Value) -> Block {
    Block::Sapling {
        material,
        growth_stage: Int0Through1::new(
            nbt_value_lookup_string(properties, "stage").unwrap().parse::<i8>().unwrap(),
        ).unwrap(),
    }
}

fn water(properties: &Value) -> Block {
    let raw_level = fluid_raw_level(properties);

    #[allow(clippy::verbose_bit_mask)]
    if (raw_level & 0x7) == 0x0 {
        Block::WaterSource
    } else {
        Block::Water {
            falling: fluid_falling(raw_level),
            level: fluid_level(raw_level),
        }
    }
}

fn lava(properties: &Value) -> Block {
    let raw_level = fluid_raw_level(properties);

    #[allow(clippy::verbose_bit_mask)]
    if (raw_level & 0x7) == 0x0 {
        Block::LavaSource
    } else {
        Block::Lava {
            falling: fluid_falling(raw_level),
            level: fluid_level(raw_level),
        }
    }
}

fn fluid_raw_level(properties: &Value) -> i8 {
    nbt_value_lookup_string(properties, "level").unwrap().parse::<i8>().unwrap()
}

fn fluid_falling(raw_level: i8) -> bool {
    (raw_level & 0x8) == 0x8
}

fn fluid_level(raw_level: i8) -> Int1Through7 {
    Int1Through7::new(8 - (raw_level & 0x7)).unwrap()
}

fn log(material: WoodMaterial, properties: &Value) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: false,
        bark_on_all_sides: false,
    })
}

fn stripped_log(material: WoodMaterial, properties: &Value) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: true,
        bark_on_all_sides: false,
    })
}

fn wood(material: WoodMaterial, properties: &Value) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: false,
        bark_on_all_sides: true,
    })
}

fn stripped_wood(material: WoodMaterial, properties: &Value) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: true,
        bark_on_all_sides: true,
    })
}

fn leaves(material: LeavesMaterial, properties: &Value) -> Block {
    Block::Leaves {
        material,
        distance_to_trunk: Int0Through7::new(
                nbt_value_lookup_string(properties, "distance")
                .unwrap()
                .as_str()
                .parse::<i8>()
                .unwrap()),
        persistent: nbt_value_lookup_string(properties, "persistent").unwrap().as_str().parse().unwrap()
    }
}

fn noteblock(properties: &Value) -> Block {
    let noteblock = Noteblock {
        pitch: Pitch::from_value(nbt_value_lookup_string(properties, "note").unwrap().as_str().parse().unwrap())
    };
    Block::Noteblock(noteblock)
}

fn rail(rail_type: RailType, properties: &Value) -> Block {
    Block::Rail {
        variant: rail_type,
        shape: match nbt_value_lookup_string(properties, "shape").unwrap().as_str() {
            "north_south" => RailShape::NorthSouth,
            "east_west" => RailShape::EastWest,
            "north_east" => RailShape::NorthEast,
            "north_west" => RailShape::NorthWest,
            "south_east" => RailShape::SouthEast,
            "south_west" => RailShape::SouthWest,
            "ascending_north" => RailShape::AscendingNorth,
            "ascending_south" => RailShape::AscendingSouth,
            "ascending_east" => RailShape::AscendingEast,
            "ascending_west" => RailShape::AscendingWest,
            _ => unreachable!(),
        },
    }
}

fn piston(sticky: bool, properties: &Value) -> Block {
    let facing = facing_surface6(properties);
    let extended = nbt_value_lookup_string(properties, "extended").unwrap().as_str().parse().unwrap();

    if sticky {
        Block::StickyPiston { facing, extended }
    } else {
        Block::Piston { facing, extended }
    }
}

fn piston_head(properties: &Value) -> Block {
    let facing = facing_surface6(properties);

    match nbt_value_lookup_string(properties, "type").unwrap().as_str() {
        "sticky" => Block::StickyPistonHead { facing },
        "normal" => Block::PistonHead { facing },
        _ => unreachable!(),
    }
}

fn slab(material: SlabMaterial, properties: &Value) -> Block {
    Block::Slab(Slab {
        material,
        position: match nbt_value_lookup_string(properties, "type").unwrap().as_str() {
            "bottom" => SlabVariant::Bottom,
            "double" => SlabVariant::Double,
            "top" => SlabVariant::Top,
            _ => unreachable!(),
        },
        waterlogged: waterlogged(properties),
    })
}

fn wall_torch(properties: &Value) -> Block {
    Block::Torch { attached: wall_torch_attached(properties) }
}

fn redstone_wall_torch(properties: &Value) -> Block {
    Block::RedstoneTorch { attached: wall_torch_attached(properties) }
}

fn soul_wall_torch(properties: &Value) -> Block {
    Block::SoulTorch { attached: wall_torch_attached(properties) }
}

fn wall_torch_attached(properties: &Value) -> Surface5 {
    match nbt_value_lookup_string(properties, "facing").unwrap().as_str() {
        "north" => Surface5::South,
        "south" => Surface5::North,
        "east" => Surface5::West,
        "west" => Surface5::East,
        _ => unreachable!(),
    }
}

fn fire(properties: &Value) -> Block {
    Block::Fire { age: age0_15(properties) }
}

fn stairs(material: StairMaterial, properties: &Value) -> Block {
    let facing_str = nbt_value_lookup_string(properties, "facing").unwrap();
    let half_str = nbt_value_lookup_string(properties, "half").unwrap();

    Block::Stairs(Stair {
        material,
        position: match (half_str.as_str(), facing_str.as_str()) {
            ("bottom", "north") => Edge8::DownNorth,
            ("bottom", "south") => Edge8::DownSouth,
            ("bottom", "east") => Edge8::DownEast,
            ("bottom", "west") => Edge8::DownWest,
            ("top", "north") => Edge8::DownNorth,
            ("top", "south") => Edge8::DownSouth,
            ("top", "east") => Edge8::DownEast,
            ("top", "west") => Edge8::DownWest,
            _ => unreachable!(),
        },
        waterlogged: waterlogged(properties),
    })
}

fn door(material: DoorMaterial, properties: &Value) -> Block {

    Block::Door(Door {
        material,
        facing: facing_surface4(properties),
        half: door_half(properties),
        hinged_at: door_hinge(properties),
        open: open(properties),
    })
}

fn ladder(properties: &Value) -> Block {
    Block::Ladder {
        facing: facing_surface4(properties),
        waterlogged: waterlogged(properties),
    }
}

fn lever(properties: &Value) -> Block {
    let surface_rotation = surface_rotation12(properties);
    let on_off_state = if powered(properties) { OnOffState::On } else { OnOffState::Off };

    Block::Lever(surface_rotation, on_off_state)
}

fn button(material: ButtonMaterial, properties: &Value) -> Block {
    Block::Button(material, surface_rotation12(properties))
}

fn pressure_plate(material: PressurePlateMaterial) -> Block {
    Block::PressurePlate { material }
}

fn snow(properties: &Value) -> Block {
    Block::Snow { thickness: layers1_8(properties) }
}

fn cactus(properties: &Value) -> Block {
    Block::Cactus { growth_stage: age0_15(properties) }
}

fn sugar_cane(properties: &Value) -> Block {
    Block::SugarCane { growth_stage: age0_15(properties) }
}

fn fence(material: FenceMaterial, properties: &Value) -> Block {
    Block::Fence { material, waterlogged: waterlogged(properties) }
}

fn carved_pumpkin(properties: &Value) -> Block {
    Block::CarvedPumpkin { facing: facing_surface4(properties) }
}

fn jack_o_lantern(properties: &Value) -> Block {
    Block::JackOLantern { facing: facing_surface4(properties) }
}

fn cake(properties: &Value) -> Block {
    Block::Cake { pieces: pieces1_7(properties) }
}

fn nether_portal(properties: &Value) -> Block {
    Block::NetherPortal { alignment: Some(portal_alignment(properties)) }
}

fn repeater(properties: &Value) -> Block {
    Block::RedstoneRepeater(RedstoneRepeater {
            facing: facing_surface4(properties),
            delay: delay1_4(properties),
    })
}

fn trapdoor(material: DoorMaterial, properties: &Value) -> Block {
    Block::Trapdoor(Trapdoor {
        material: material,
        hinge_at: trapdoor_hinge(properties),
        open: open(properties),
        waterlogged: waterlogged(properties),
    })
}

fn brown_mushroom_block(properties: &Value) -> Block {
    Block::BrownMushroomBlock { cap_directions: direction_flags6(properties) }
}

fn red_mushroom_block(properties: &Value) -> Block {
    Block::RedMushroomBlock { cap_directions: direction_flags6(properties) }
}

fn mushroom_stem(properties: &Value) -> Block {
    Block::MushroomStem { stem_directions: direction_flags6(properties) }
}

fn glass_pane(properties: &Value) -> Block {
    Block::GlassPane { colour: None, waterlogged: waterlogged(properties) }
}

fn pumpkin_stem(properties: &Value) -> Block {
    Block::PumpkinStem { state: StemState::Growing(age0_7(properties)) }
}

fn attached_pumpkin_stem(properties: &Value) -> Block {
    Block::PumpkinStem { state: StemState::Attached(facing_surface4(properties)) }
}

fn melon_stem(properties: &Value) -> Block {
    Block::MelonStem { state: StemState::Growing(age0_7(properties)) }
}

fn attached_melon_stem(properties: &Value) -> Block {
    Block::MelonStem { state: StemState::Attached(facing_surface4(properties)) }
}

fn vine(properties: &Value) -> Block {
    Block::Vines(Vines { anchored_at: direction_flags5(properties) })
}

fn fence_gate(material: WoodMaterial, properties: &Value) -> Block {
    Block::FenceGate {
        material,
        facing: facing_surface4(properties),
        open: open(properties),
    }
}

fn nether_wart(properties: &Value) -> Block {
    Block::NetherWart { growth_stage: age0_3(properties) }
}

fn cauldron(properties: &Value) -> Block {
    Block::Cauldron { water_level: level0_3(properties) }
}

fn end_portal_frame(properties: &Value) -> Block {
    Block::EndPortalFrame {
        facing: facing_surface4(properties),
        has_eye: boolean(properties, "eye"),
    }
}

fn cocoa(properties: &Value) -> Block {
    Block::Cocoa {
        growth_stage: age0_2(properties),
        facing: facing_surface4(properties),
    }
}

fn ender_chest(properties: &Value) -> Block {
    Block::EnderChest {
        facing: facing_surface4(properties),
        waterlogged: waterlogged(properties),
    }
}

fn tripwire_hook(properties: &Value) -> Block {
    Block::TripwireHook {
        facing: facing_surface4(properties),
    }
}

fn wall(material: WallMaterial, properties: &Value) -> Block {
    Block::Wall { material, waterlogged: waterlogged(properties) }
}

fn potted_plant(plant: PottedPlant) -> Block {
    Block::FlowerPot(FlowerPot { plant: Some(plant) })
}

fn head(variant: HeadVariant, properties: &Value) -> Block {
    Block::Head(Head {
        variant,
        placement: WallOrRotatedOnFloor::Floor(floor_sign_facing_direction16(properties)),
        waterlogged: false,
    })
}

fn wall_head(variant: HeadVariant, properties: &Value) -> Block {
    Block::Head(Head {
        variant,
        placement: WallOrRotatedOnFloor::Wall(wall_sign_facing_surface4(properties)),
        waterlogged: false,
    })
}

fn anvil(damage: AnvilDamage, properties: &Value) -> Block {
    Block::Anvil {
        facing: facing_surface4(properties),
        damage,
    }
}

fn comparator(properties: &Value) -> Block {
    let facing = facing_surface4(properties);
    match nbt_value_lookup_string(properties, "mode").unwrap().as_str() {
        "subtract" => Block::RedstoneSubtractor { facing },
        "compare" => Block::RedstoneComparator { facing },
        _ => unreachable!(),
    }
}

fn daylight_detector(properties: &Value) -> Block {
    if boolean(properties, "inverted") {
        Block::InvertedDaylightDetector
    } else {
        Block::DaylightDetector
    }
}

fn quartz_pillar(properties: &Value) -> Block {
    Block::QuartzPillar { alignment: wood_alignment(&properties) }
}

fn stained_glass_pane(colour: Colour, properties: &Value) -> Block {
    Block::GlassPane { colour: Some(colour), waterlogged: waterlogged(properties) }
}

fn hay_bale(properties: &Value) -> Block {
    Block::HayBale { alignment: wood_alignment(&properties) }
}

fn tall(name: &'static str, properties: &Value) -> Block {
    match name {
        "sunflower" => tall_helper(
                Block::Flower(Flower::SunflowerTop),
                Block::Flower(Flower::SunflowerBottom),
                properties,
            ),
        "lilac" => tall_helper(
                Block::Flower(Flower::LilacTop),
                Block::Flower(Flower::LilacBottom),
                properties,
            ),
        "peony" => tall_helper(
                Block::Flower(Flower::PeonyTop),
                Block::Flower(Flower::PeonyBottom),
                properties,
            ),
        "rose_bush" => tall_helper(
                Block::Flower(Flower::RoseBushTop),
                Block::Flower(Flower::RoseBushBottom),
                properties,
            ),
        "tall_grass" => tall_helper(
                Block::Grass(Grass::TallGrassTop),
                Block::Grass(Grass::TallGrassBottom),
                properties,
            ),
        "large_fern" => tall_helper(
                Block::Grass(Grass::LargeFernTop),
                Block::Grass(Grass::LargeFernBottom),
                properties,
            ),
        _ => unreachable!(),
    }
}

fn tall_helper(top: Block, bottom: Block, properties: &Value) -> Block {
    match door_half(properties) {
        DoorHalf::Upper => top,
        DoorHalf::Lower => bottom,
    }
}

fn bed(colour: Colour, properties: &Value) -> Block {
    Block::Bed(Bed {
        colour,
        facing: facing_surface4(properties),
        end: bed_part(properties),
    })
}

fn chorus_flower(properties: &Value) -> Block {
    Block::ChorusFlower { growth_stage: age0_5(properties) }
}

fn purpur_pillar(properties: &Value) -> Block {
    Block::PurpurPillar { alignment: wood_alignment(&properties) }
}

fn bone_block(properties: &Value) -> Block {
    Block::BoneBlock { alignment: wood_alignment(&properties) }
}

fn observer(properties: &Value) -> Block {
    Block::Observer { facing: facing_surface6(properties) }
}

fn glazed_terracotta(colour: Colour, properties: &Value) -> Block {
    Block::GlazedTerracotta(GlazedTerracotta {
        colour,
        facing: facing_surface4(properties),
    })
}

fn concrete(colour: Colour) -> Block {
    Block::Concrete { colour }
}

fn concrete_powder(colour: Colour) -> Block {
    Block::ConcretePowder { colour }
}

//
// Convenience functions for proto blocks
//

fn proto_banner(colour: Colour, properties: &Value) -> ProtoBlock {
    ProtoBlock::Banner {
        colour,
        placement: WallOrRotatedOnFloor::Floor(floor_sign_facing_direction16(properties)),
    }
}

fn proto_wall_banner(colour: Colour, properties: &Value) -> ProtoBlock {
    ProtoBlock::Banner {
        colour,
        placement: WallOrRotatedOnFloor::Wall(wall_sign_facing_surface4(properties)),
    }
}

fn proto_dispenser(properties: &Value) -> ProtoBlock {
    ProtoBlock::Dispenser { facing: facing_surface6(properties) }
}

fn proto_dropper(properties: &Value) -> ProtoBlock {
    ProtoBlock::Dropper { facing: facing_surface6(properties) }
}

fn proto_chest(properties: &Value) -> ProtoBlock {
    ProtoBlock::Chest {
        facing: facing_surface4(properties),
        variant: chest_variant(properties),
        waterlogged: waterlogged(properties),
    }
}

fn proto_trapped_chest(properties: &Value) -> ProtoBlock {
    ProtoBlock::TrappedChest {
        facing: facing_surface4(properties),
        variant: chest_variant(properties),
        waterlogged: waterlogged(properties),
    }
}

fn proto_furnace(properties: &Value) -> ProtoBlock {
    ProtoBlock::Furnace {
        facing: facing_surface4(properties),
        lit: lit(properties),
    }
}

fn proto_shulker_box(colour: Option<Colour>, properties: &Value) -> ProtoBlock {
    ProtoBlock::ShulkerBox {
        colour,
        facing: facing_surface6(properties),
    }
}

fn proto_sign(material: WoodMaterial, properties: &Value) -> ProtoBlock {
    ProtoBlock::Sign {
        material,
        placement: WallOrRotatedOnFloor::Floor(floor_sign_facing_direction16(properties)),
        waterlogged: waterlogged(properties),
    }
}

fn proto_wall_sign(material: WoodMaterial, properties: &Value) -> ProtoBlock {
    ProtoBlock::Sign {
        material,
        placement: WallOrRotatedOnFloor::Wall(wall_sign_facing_surface4(properties)),
        waterlogged: waterlogged(properties),
    }
}

fn proto_hopper(properties: &Value) -> ProtoBlock {
    ProtoBlock::Hopper { facing: facing_surface5(properties) }
}

//
// Convenience functions for undecided block/proto block
//

fn jukebox(properties: &Value) -> PaletteItem {
    let has_record = boolean(properties, "has_record");

    if has_record {
        PaletteItem::ProtoBlock(ProtoBlock::Jukebox)
    } else {
        PaletteItem::Block(Block::Jukebox(Box::new(Jukebox { record: None })))
    }
}

//
// Convenience functions for value import
//

// TODO check, and consider using the facing of the sign, instead of its attachment
// surface.
fn wall_sign_facing_surface4(properties: &Value) -> Surface4 {
    match nbt_value_lookup_string(properties, "facing").unwrap().as_str() {
        "north" => Surface4::South,
        "south" => Surface4::North,
        "east" => Surface4::West,
        "west" => Surface4::East,
        _ => unreachable!(),
    }
}

fn direction_flags5(properties: &Value) -> DirectionFlags5 {
    DirectionFlags5 {
        east: boolean(properties, "east"),
        north: boolean(properties, "north"),
        south: boolean(properties, "south"),
        up: boolean(properties, "up"),
        west: boolean(properties, "west"),
    }
}

fn direction_flags6(properties: &Value) -> DirectionFlags6 {
    DirectionFlags6 {
        east: boolean(properties, "east"),
        down: boolean(properties, "down"),
        north: boolean(properties, "north"),
        south: boolean(properties, "south"),
        up: boolean(properties, "up"),
        west: boolean(properties, "west"),
    }
}

fn floor_sign_facing_direction16(properties: &Value) -> Direction16 {
    nbt_value_lookup_string(properties, "rotation").unwrap().as_str().parse::<i8>().unwrap().into()
}

fn age0_2(properties: &Value) -> Int0Through2 {
    Int0Through2::new(
            nbt_value_lookup_string(properties, "age").unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn age0_3(properties: &Value) -> Int0Through3 {
    i0_3(properties, "age")
}

fn level0_3(properties: &Value) -> Int0Through3 {
    i0_3(properties, "level")
}

fn i0_3(properties: &Value, name: &'static str) -> Int0Through3 {
    Int0Through3::new(
            nbt_value_lookup_string(properties, name).unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn age0_5(properties: &Value) -> Int0Through5 {
    Int0Through5::new(
            nbt_value_lookup_string(properties, "age").unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn age0_7(properties: &Value) -> Int0Through7 {
    i0_7(properties, "age")
}

fn moisture0_7(properties: &Value) -> Int0Through7 {
    i0_7(properties, "moisture")
}

fn i0_7(properties: &Value, name: &'static str) -> Int0Through7 {
    Int0Through7::new(
            nbt_value_lookup_string(properties, name).unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn age0_15(properties: &Value) -> Int0Through15 {
    Int0Through15::new(
            nbt_value_lookup_string(properties, "age").unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn delay1_4(properties: &Value) -> Int1Through4 {
    Int1Through4::new(
            nbt_value_lookup_string(properties, "delay").unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn layers1_8(properties: &Value) -> Int1Through8 {
    Int1Through8::new(
            nbt_value_lookup_string(properties, "layers").unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn pieces1_7(properties: &Value) -> Int1Through7 {
    Int1Through7::new(
            7 - nbt_value_lookup_string(properties, "bites").unwrap().as_str().parse::<i8>().unwrap()
    ).unwrap()
}

fn chest_variant(properties: &Value) -> Option<ChestVariant> {
    match nbt_value_lookup_string(properties, "type").unwrap().as_str() {
        "single" => Some(ChestVariant::Single),
        "left" => Some(ChestVariant::Left),
        "right" => Some(ChestVariant::Right),
        _ => None,
    }
}

enum Face3 {
    Ceiling,
    Floor,
    Wall,
}

fn surface_rotation12(properties: &Value) -> SurfaceRotation12 {
    let face = face3(properties);
    let facing = facing_surface4(properties);

    match (face, facing) {
        (Face3::Floor, Surface4::North) => SurfaceRotation12::DownFacingNorth,
        (Face3::Floor, Surface4::South) => SurfaceRotation12::DownFacingSouth,
        (Face3::Floor, Surface4::East) => SurfaceRotation12::DownFacingEast,
        (Face3::Floor, Surface4::West) => SurfaceRotation12::DownFacingWest,
        (Face3::Wall, Surface4::North) => SurfaceRotation12::North,
        (Face3::Wall, Surface4::South) => SurfaceRotation12::South,
        (Face3::Wall, Surface4::East) => SurfaceRotation12::East,
        (Face3::Wall, Surface4::West) => SurfaceRotation12::West,
        (Face3::Ceiling, Surface4::North) => SurfaceRotation12::UpFacingNorth,
        (Face3::Ceiling, Surface4::South) => SurfaceRotation12::UpFacingSouth,
        (Face3::Ceiling, Surface4::East) => SurfaceRotation12::UpFacingEast,
        (Face3::Ceiling, Surface4::West) => SurfaceRotation12::UpFacingWest,
    }
}

fn face3(properties: &Value) -> Face3 {
    match nbt_value_lookup_string(properties, "face").unwrap().as_str() {
        "ceiling" => Face3::Ceiling,
        "floor" => Face3::Floor,
        "wall" => Face3::Wall,
        _ => unreachable!(),
    }
}

fn facing_surface4(properties: &Value) -> Surface4 {
    match nbt_value_lookup_string(properties, "facing").unwrap().as_str() {
        "north" => Surface4::North,
        "south" => Surface4::South,
        "east" => Surface4::East,
        "west" => Surface4::West,
        _ => unreachable!(),
    }
}

fn facing_surface5(properties: &Value) -> Surface5 {
    match nbt_value_lookup_string(properties, "facing").unwrap().as_str() {
        "north" => Surface5::North,
        "south" => Surface5::South,
        "east" => Surface5::East,
        "west" => Surface5::West,
        "down" => Surface5::Down,
        _ => unreachable!(),
    }
}

fn facing_surface6(properties: &Value) -> Surface6 {
    match nbt_value_lookup_string(properties, "facing").unwrap().as_str() {
        "up" => Surface6::Up,
        "down" => Surface6::Down,
        "north" => Surface6::North,
        "south" => Surface6::South,
        "east" => Surface6::East,
        "west" => Surface6::West,
        _ => unreachable!(),
    }
}

fn trapdoor_hinge(properties: &Value) -> Edge8 {
    let half = nbt_value_lookup_string(properties, "half").unwrap();
    let facing = facing_surface4(properties);

    match (half.as_str(), facing) {
        ("top", Surface4::North) => Edge8::UpSouth,
        ("top", Surface4::South) => Edge8::UpNorth,
        ("top", Surface4::East) => Edge8::UpWest,
        ("top", Surface4::West) => Edge8::UpEast,
        ("bottom", Surface4::North) => Edge8::DownSouth,
        ("bottom", Surface4::South) => Edge8::DownNorth,
        ("bottom", Surface4::East) => Edge8::DownWest,
        ("bottom", Surface4::West) => Edge8::DownEast,
        _ => unreachable!(),
    }
}

fn door_hinge(properties: &Value) -> Hinge {
    match nbt_value_lookup_string(properties, "hinge").unwrap().as_str() {
        "left" => Hinge::Left,
        "right" => Hinge::Right,
        _ => unreachable!(),
    }
}

fn door_half(properties: &Value) -> DoorHalf {
    match nbt_value_lookup_string(properties, "half").unwrap().as_str() {
        "upper" => DoorHalf::Upper,
        "lower" => DoorHalf::Lower,
        _ => unreachable!(),
    }
}

fn open(properties: &Value) -> bool {
    boolean(properties, "open")
}

fn waterlogged(properties: &Value) -> bool {
    boolean(properties, "waterlogged")
}

fn lit(properties: &Value) -> bool {
    boolean(properties, "lit")
}

fn powered(properties: &Value) -> bool {
    boolean(properties, "powered")
}

fn boolean(properties: &Value, name: &'static str) -> bool {
    nbt_value_lookup_string(properties, name).unwrap().as_str().parse().unwrap()
}

fn portal_alignment(properties: &Value) -> Axis2 {
    match nbt_value_lookup_string(properties, "axis").unwrap().as_str() {
        "x" => Axis2::X,
        "z" => Axis2::Z,
        _ => unreachable!(),
    }
}

fn wood_alignment(properties: &Value) -> Axis3 {
    match nbt_value_lookup_string(properties, "axis").unwrap().as_str() {
        "x" => Axis3::X,
        "y" => Axis3::Y,
        "z" => Axis3::Z,
        _ => unreachable!(),
    }
}

fn bed_part(properties: &Value) -> BedEnd {
    match nbt_value_lookup_string(properties, "part").unwrap().as_str() {
        "foot" => BedEnd::Foot,
        "head" => BedEnd::Head,
        _ => unreachable!(),
    }
}
