use std::convert::TryFrom;

use mcprogedit::block::*;
use mcprogedit::bounded_ints::Int0Through7;
use mcprogedit::colour::Colour;
use mcprogedit::coordinates::BlockCoord;
use mcprogedit::material::Material;
use mcprogedit::positioning::*;
use mcprogedit::world_excerpt::WorldExcerpt;

fn load_excerpt(path: &str, at: (i64, i64, i64), size: (i64, i64, i64)) -> WorldExcerpt {
    let save_directory = std::path::Path::new(path);
    WorldExcerpt::from_save(
        at.into(),
        Into::<BlockCoord>::into(at) + Into::<BlockCoord>::into(size)
            - Into::<BlockCoord>::into((1, 1, 1)),
        save_directory,
    )
}

fn assert_block_eq(excerpt: &WorldExcerpt, at: (i64, i64, i64), block: &Block) {
    assert_eq!(excerpt.get_block_at(at.into()), Some(block));
}

#[rustfmt::skip]
fn check_bed(we: &WorldExcerpt, at: (i64, i64, i64), fac: Direction, col: Colour, end: BedEnd) {
    let block = we.get_block_at(at.into()).unwrap();
    let bed = Bed::try_from(block.clone()).unwrap();
    assert!(bed.has_facing_of(fac));
    assert!(bed.has_colour_of(col));
    assert!(bed.end == end);
}

fn check_stairs(we: &WorldExcerpt, at: (i64, i64, i64), dir: Direction, mat: Material) {
    let block = we.get_block_at(at.into()).unwrap();
    assert!(
        block.is_stairs()
        && block.has_facing_of(dir)
        && block.has_material_of(mat)
    );
}

#[test]
/// Import of blocks with id 0 through 15
fn v_1_12_2_block_group_1() {
    let excerpt = load_excerpt("tests/saves/1_12_2/", (0, 56, 0), (16, 2, 8));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::Air);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::Stone);
    assert_block_eq(&excerpt, (1, 0, 1), &Block::Granite);
    assert_block_eq(&excerpt, (1, 0, 2), &Block::PolishedGranite);
    assert_block_eq(&excerpt, (1, 0, 3), &Block::Diorite);
    assert_block_eq(&excerpt, (1, 0, 4), &Block::PolishedDiorite);
    assert_block_eq(&excerpt, (1, 0, 5), &Block::Andesite);
    assert_block_eq(&excerpt, (1, 0, 6), &Block::PolishedAndesite);

    assert_block_eq(&excerpt, (2, 0, 4), &Block::GrassBlock);

    assert_block_eq(&excerpt, (3, 0, 0), &Block::Dirt);
    assert_block_eq(&excerpt, (3, 0, 1), &Block::CoarseDirt);
    assert_block_eq(&excerpt, (3, 0, 2), &Block::Podzol);

    assert_block_eq(&excerpt, (4, 0, 0), &Block::Cobblestone);

    assert_block_eq(&excerpt, (5, 0, 0), &Block::oak_planks());
    assert_block_eq(&excerpt, (5, 0, 1), &Block::spruce_planks());
    assert_block_eq(&excerpt, (5, 0, 2), &Block::birch_planks());
    assert_block_eq(&excerpt, (5, 0, 3), &Block::jungle_planks());
    assert_block_eq(&excerpt, (5, 0, 4), &Block::acacia_planks());
    assert_block_eq(&excerpt, (5, 0, 5), &Block::dark_oak_planks());

    assert_block_eq(&excerpt, (6, 0, 0), &Block::oak_sapling());
    assert_block_eq(&excerpt, (6, 0, 1), &Block::spruce_sapling());
    assert_block_eq(&excerpt, (6, 0, 2), &Block::birch_sapling());
    assert_block_eq(&excerpt, (6, 0, 3), &Block::jungle_sapling());
    assert_block_eq(&excerpt, (6, 0, 4), &Block::acacia_sapling());
    assert_block_eq(&excerpt, (6, 0, 5), &Block::dark_oak_sapling());

    assert_block_eq(&excerpt, (7, 0, 0), &Block::Bedrock);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::water(7));
    assert_block_eq(&excerpt, (8, 0, 1), &Block::water(6));
    assert_block_eq(&excerpt, (8, 0, 2), &Block::water(5));
    assert_block_eq(&excerpt, (8, 0, 3), &Block::water(4));
    assert_block_eq(&excerpt, (8, 0, 4), &Block::water(3));
    assert_block_eq(&excerpt, (8, 0, 5), &Block::water(2));
    assert_block_eq(&excerpt, (8, 0, 6), &Block::water(1));
    assert_block_eq(&excerpt, (8, 0, 7), &Block::Air);

    assert_block_eq(&excerpt, (9, 0, 0), &Block::water(8));

    assert_block_eq(&excerpt, (10, 1, 0), &Block::lava(6));
    assert_block_eq(&excerpt, (10, 1, 1), &Block::lava(4));
    assert_block_eq(&excerpt, (10, 1, 2), &Block::lava(2));
    assert_block_eq(&excerpt, (10, 1, 3), &Block::Air);

    assert_block_eq(&excerpt, (11, 1, 0), &Block::lava(8));

    assert_block_eq(&excerpt, (12, 0, 0), &Block::Sand);
    assert_block_eq(&excerpt, (12, 0, 1), &Block::RedSand);

    assert_block_eq(&excerpt, (13, 0, 0), &Block::Gravel);

    assert_block_eq(&excerpt, (14, 0, 0), &Block::GoldOre);

    assert_block_eq(&excerpt, (15, 0, 0), &Block::IronOre);
}

#[test]
/// Import of blocks with id 16 through 31
fn v_1_12_2_block_group_2() {
    let excerpt = load_excerpt("tests/saves/1_12_2/", (16, 56, 0), (16, 5, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::CoalOre);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::oak_log(Axis3::Y));
    assert_block_eq(&excerpt, (1, 1, 0), &Block::oak_log(Axis3::Z));
    assert_block_eq(&excerpt, (1, 2, 0), &Block::oak_log(Axis3::X));
    assert_block_eq(&excerpt, (1, 0, 1), &Block::spruce_log(Axis3::Y));
    assert_block_eq(&excerpt, (1, 1, 1), &Block::spruce_log(Axis3::Z));
    assert_block_eq(&excerpt, (1, 2, 1), &Block::spruce_log(Axis3::X));
    assert_block_eq(&excerpt, (1, 0, 2), &Block::birch_log(Axis3::Y));
    assert_block_eq(&excerpt, (1, 1, 2), &Block::birch_log(Axis3::Z));
    assert_block_eq(&excerpt, (1, 2, 2), &Block::birch_log(Axis3::X));
    assert_block_eq(&excerpt, (1, 0, 3), &Block::jungle_log(Axis3::Y));
    assert_block_eq(&excerpt, (1, 1, 3), &Block::jungle_log(Axis3::Z));
    assert_block_eq(&excerpt, (1, 2, 3), &Block::jungle_log(Axis3::X));

    assert_block_eq(&excerpt, (2, 0, 0), &Block::oak_leaves(true));
    assert_block_eq(&excerpt, (2, 0, 1), &Block::spruce_leaves(true));
    assert_block_eq(&excerpt, (2, 0, 2), &Block::birch_leaves(true));
    assert_block_eq(&excerpt, (2, 0, 3), &Block::jungle_leaves(true));

    assert_block_eq(&excerpt, (3, 0, 0), &Block::Sponge);
    assert_block_eq(&excerpt, (3, 0, 1), &Block::WetSponge);

    assert_block_eq(&excerpt, (4, 0, 0), &Block::Glass { colour: None });

    assert_block_eq(&excerpt, (5, 0, 0), &Block::LapisLazuliOre);

    assert_block_eq(&excerpt, (6, 0, 0), &Block::LapisLazuliBlock);

    let block = excerpt.get_block_at((7, 0, 0).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::West));
    let block = excerpt.get_block_at((7, 0, 2).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::South));
    let block = excerpt.get_block_at((7, 0, 4).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::East));
    let block = excerpt.get_block_at((7, 0, 6).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::North));
    let block = excerpt.get_block_at((7, 0, 8).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::Up));
    let block = excerpt.get_block_at((7, 0, 10).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::Down));

    assert_block_eq(&excerpt, (8, 0, 0), &Block::Sandstone);
    assert_block_eq(&excerpt, (8, 0, 1), &Block::ChiseledSandstone);
    assert_block_eq(&excerpt, (8, 0, 2), &Block::SmoothSandstone);

    fn check_noteblock(excerpt: &WorldExcerpt, at: (i64, i64, i64), pitch: Pitch) {
        let block = excerpt.get_block_at(at.into()).unwrap();
        let noteblock = Noteblock::try_from(block.clone()).unwrap();
        assert!(noteblock.has_pitch_of(pitch));
    }
    // Lower row of noteblocks (above various blocks)
    check_noteblock(&excerpt, (9, 1, 0), Pitch::Fs0);
    check_noteblock(&excerpt, (9, 1, 1), Pitch::G0);
    check_noteblock(&excerpt, (9, 1, 2), Pitch::Gs0);
    check_noteblock(&excerpt, (9, 1, 3), Pitch::A0);
    check_noteblock(&excerpt, (9, 1, 4), Pitch::As0);
    check_noteblock(&excerpt, (9, 1, 5), Pitch::B0);
    check_noteblock(&excerpt, (9, 1, 6), Pitch::C1);
    check_noteblock(&excerpt, (9, 1, 7), Pitch::Cs1);
    check_noteblock(&excerpt, (9, 1, 8), Pitch::D1);
    check_noteblock(&excerpt, (9, 1, 9), Pitch::Ds1);
    check_noteblock(&excerpt, (9, 1, 10), Pitch::E1);
    check_noteblock(&excerpt, (9, 1, 11), Pitch::F1);
    check_noteblock(&excerpt, (9, 1, 12), Pitch::Fs1);
    check_noteblock(&excerpt, (9, 1, 13), Pitch::G1);
    check_noteblock(&excerpt, (9, 1, 14), Pitch::Gs1);
    check_noteblock(&excerpt, (9, 1, 15), Pitch::A1);
    // Upper row of noteblocks (all above emerald blocks)
    check_noteblock(&excerpt, (9, 4, 0), Pitch::As1);
    check_noteblock(&excerpt, (9, 4, 1), Pitch::B1);
    check_noteblock(&excerpt, (9, 4, 2), Pitch::C2);
    check_noteblock(&excerpt, (9, 4, 3), Pitch::Cs2);
    check_noteblock(&excerpt, (9, 4, 4), Pitch::D2);
    check_noteblock(&excerpt, (9, 4, 5), Pitch::Ds2);
    check_noteblock(&excerpt, (9, 4, 6), Pitch::E2);
    check_noteblock(&excerpt, (9, 4, 7), Pitch::F2);
    check_noteblock(&excerpt, (9, 4, 8), Pitch::Fs2);

    #[rustfmt::skip]
    check_bed(&excerpt, (10, 0, 0), Direction::North, Colour::Red, BedEnd::Head);
    #[rustfmt::skip]
    check_bed(&excerpt, (10, 0, 1), Direction::North, Colour::Red, BedEnd::Foot);

    #[rustfmt::skip]
    assert_block_eq(&excerpt, (11, 0, 0), &Block::powered_rail(RailShape::EastWest));

    #[rustfmt::skip]
    assert_block_eq(&excerpt, (12, 0, 0), &Block::detector_rail(RailShape::EastWest));

    let block = excerpt.get_block_at((13, 0, 0).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::West));
    let block = excerpt.get_block_at((13, 0, 2).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::South));
    let block = excerpt.get_block_at((13, 0, 4).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::East));
    let block = excerpt.get_block_at((13, 0, 6).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::North));
    let block = excerpt.get_block_at((13, 0, 8).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::Up));
    let block = excerpt.get_block_at((13, 0, 10).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::Down));

    assert_block_eq(&excerpt, (14, 0, 0), &Block::Cobweb);

    assert_block_eq(&excerpt, (15, 0, 1), &Block::Grass(Grass::Grass));
    assert_block_eq(&excerpt, (15, 0, 2), &Block::Grass(Grass::Fern));
}

#[test]
/// Import of blocks with id 32 through 47
fn v_1_12_2_block_group_3() {
    let excerpt = load_excerpt("tests/saves/1_12_2/", (32, 56, 0), (16, 2, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::DeadBush);

    let block = excerpt.get_block_at((1, 0, 0).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::West));
    let block = excerpt.get_block_at((1, 0, 2).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::South));
    let block = excerpt.get_block_at((1, 0, 4).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::East));
    let block = excerpt.get_block_at((1, 0, 6).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::North));
    let block = excerpt.get_block_at((1, 0, 8).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::Up));
    let block = excerpt.get_block_at((1, 0, 10).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::Down));

    let block = excerpt.get_block_at((2, 0, 4).into()).unwrap();
    assert!(block.is_piston_head() && block.has_facing_of(Direction::East));

    assert_block_eq(&excerpt, (3, 0, 0), &Block::Wool { colour: Colour::White });
    assert_block_eq(&excerpt, (3, 0, 1), &Block::Wool { colour: Colour::Orange });
    assert_block_eq(&excerpt, (3, 0, 2), &Block::Wool { colour: Colour::Magenta });
    assert_block_eq(&excerpt, (3, 0, 3), &Block::Wool { colour: Colour::LightBlue });
    assert_block_eq(&excerpt, (3, 0, 4), &Block::Wool { colour: Colour::Yellow });
    assert_block_eq(&excerpt, (3, 0, 5), &Block::Wool { colour: Colour::Lime });
    assert_block_eq(&excerpt, (3, 0, 6), &Block::Wool { colour: Colour::Pink });
    assert_block_eq(&excerpt, (3, 0, 7), &Block::Wool { colour: Colour::Gray });
    assert_block_eq(&excerpt, (3, 0, 8), &Block::Wool { colour: Colour::LightGray });
    assert_block_eq(&excerpt, (3, 0, 9), &Block::Wool { colour: Colour::Cyan });
    assert_block_eq(&excerpt, (3, 0, 10), &Block::Wool { colour: Colour::Purple });
    assert_block_eq(&excerpt, (3, 0, 11), &Block::Wool { colour: Colour::Blue });
    assert_block_eq(&excerpt, (3, 0, 12), &Block::Wool { colour: Colour::Brown });
    assert_block_eq(&excerpt, (3, 0, 13), &Block::Wool { colour: Colour::Green });
    assert_block_eq(&excerpt, (3, 0, 14), &Block::Wool { colour: Colour::Red });
    assert_block_eq(&excerpt, (3, 0, 15), &Block::Wool { colour: Colour::Black });

    // NB block with ID 36, which should be here at x position 4,
    // is not implemented and is not present in the save file.

    assert_block_eq(&excerpt, (5, 0, 0), &Block::Flower(Flower::Dandelion));

    assert_block_eq(&excerpt, (6, 0, 0), &Block::Flower(Flower::Poppy));
    assert_block_eq(&excerpt, (6, 0, 1), &Block::Flower(Flower::BlueOrchid));
    assert_block_eq(&excerpt, (6, 0, 2), &Block::Flower(Flower::Allium));
    assert_block_eq(&excerpt, (6, 0, 3), &Block::Flower(Flower::AzureBluet));
    assert_block_eq(&excerpt, (6, 0, 4), &Block::Flower(Flower::TulipRed));
    assert_block_eq(&excerpt, (6, 0, 5), &Block::Flower(Flower::TulipOrange));
    assert_block_eq(&excerpt, (6, 0, 6), &Block::Flower(Flower::TulipLightGray));
    assert_block_eq(&excerpt, (6, 0, 7), &Block::Flower(Flower::TulipPink));
    assert_block_eq(&excerpt, (6, 0, 8), &Block::Flower(Flower::OxeyeDaisy));

    assert_block_eq(&excerpt, (7, 0, 0), &Block::BrownMushroom);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::RedMushroom);

    assert_block_eq(&excerpt, (9, 0, 0), &Block::BlockOfGold);

    assert_block_eq(&excerpt, (10, 0, 0), &Block::BlockOfIron);

    assert_block_eq(&excerpt, (11, 0, 0), &Block::double_slab(Material::SmoothStone));
    assert_block_eq(&excerpt, (11, 0, 1), &Block::double_slab(Material::Sandstone));
    // No petrified wood slab in the test save, unfortunately.
    assert_block_eq(&excerpt, (11, 0, 3), &Block::double_slab(Material::Cobblestone));
    assert_block_eq(&excerpt, (11, 0, 4), &Block::double_slab(Material::Brick));
    assert_block_eq(&excerpt, (11, 0, 5), &Block::double_slab(Material::StoneBrick));
    assert_block_eq(&excerpt, (11, 0, 6), &Block::double_slab(Material::NetherBrick));
    assert_block_eq(&excerpt, (11, 0, 7), &Block::double_slab(Material::Quartz));

    assert_block_eq(&excerpt, (12, 0, 0), &Block::bottom_slab(Material::SmoothStone));
    assert_block_eq(&excerpt, (12, 1, 0), &Block::top_slab(Material::SmoothStone));
    assert_block_eq(&excerpt, (12, 0, 1), &Block::bottom_slab(Material::Sandstone));
    assert_block_eq(&excerpt, (12, 1, 1), &Block::top_slab(Material::Sandstone));
    // No petrified wood slab in the test save, unfortunately.
    assert_block_eq(&excerpt, (12, 0, 3), &Block::bottom_slab(Material::Cobblestone));
    assert_block_eq(&excerpt, (12, 1, 3), &Block::top_slab(Material::Cobblestone));
    assert_block_eq(&excerpt, (12, 0, 4), &Block::bottom_slab(Material::Brick));
    assert_block_eq(&excerpt, (12, 1, 4), &Block::top_slab(Material::Brick));
    assert_block_eq(&excerpt, (12, 0, 5), &Block::bottom_slab(Material::StoneBrick));
    assert_block_eq(&excerpt, (12, 1, 5), &Block::top_slab(Material::StoneBrick));
    assert_block_eq(&excerpt, (12, 0, 6), &Block::bottom_slab(Material::NetherBrick));
    assert_block_eq(&excerpt, (12, 1, 6), &Block::top_slab(Material::NetherBrick));
    assert_block_eq(&excerpt, (12, 0, 7), &Block::bottom_slab(Material::Quartz));
    assert_block_eq(&excerpt, (12, 1, 7), &Block::top_slab(Material::Quartz));

    assert_block_eq(&excerpt, (13, 0, 0), &Block::BrickBlock);

    assert_block_eq(&excerpt, (14, 0, 0), &Block::TNT);

    assert_block_eq(&excerpt, (15, 0, 0), &Block::Bookshelf);
}

#[test]
/// Import of blocks with id 48 through 63
fn v_1_12_2_block_group_4() {
    let excerpt = load_excerpt("tests/saves/1_12_2/", (48, 56, 0), (16, 5, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::MossyCobblestone);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::Obsidian);

    let block = excerpt.get_block_at((2, 0, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::Up));
    let block = excerpt.get_block_at((2, 1, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::West));
    let block = excerpt.get_block_at((2, 2, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::South));
    let block = excerpt.get_block_at((2, 3, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::East));
    let block = excerpt.get_block_at((2, 4, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::North));

    let mut fire_block = Block::fire();
    fire_block.set_age_to(15);
    assert_block_eq(&excerpt, (3, 0, 15), &fire_block);

    // NB Block ID 52 "mob spawner" should be here at x position 4,
    // but it is not present in the save file, and is also not implemented.

    check_stairs(&excerpt, (5, 0, 0), Direction::DownEast, Material::Oak);
    check_stairs(&excerpt, (5, 0, 1), Direction::UpEast, Material::Oak);
    check_stairs(&excerpt, (5, 0, 2), Direction::DownNorth, Material::Oak);
    check_stairs(&excerpt, (5, 0, 3), Direction::UpNorth, Material::Oak);
    check_stairs(&excerpt, (5, 0, 4), Direction::DownWest, Material::Oak);
    check_stairs(&excerpt, (5, 0, 5), Direction::UpWest, Material::Oak);
    check_stairs(&excerpt, (5, 0, 6), Direction::DownSouth, Material::Oak);
    check_stairs(&excerpt, (5, 0, 7), Direction::UpSouth, Material::Oak);

    let block = excerpt.get_block_at((6, 0, 0).into()).unwrap();
    assert!(block.is_chest() && block.has_facing_of(Direction::West));
    let block = excerpt.get_block_at((6, 1, 0).into()).unwrap();
    assert!(block.is_chest() && block.has_facing_of(Direction::South));
    let block = excerpt.get_block_at((6, 2, 0).into()).unwrap();
    assert!(block.is_chest() && block.has_facing_of(Direction::East));
    let block = excerpt.get_block_at((6, 3, 0).into()).unwrap();
    assert!(block.is_chest() && block.has_facing_of(Direction::North));

    // NB No check for signal strength on the redstone wire.
    assert_block_eq(&excerpt, (7, 0, 0), &Block::RedstoneWire);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::DiamondOre);

    assert_block_eq(&excerpt, (9, 0, 0), &Block::BlockOfDiamond);

    assert_block_eq(&excerpt, (10, 0, 0), &Block::CraftingTable);

    assert_block_eq(&excerpt, (11, 0, 0), &Block::wheat());

    assert_block_eq(&excerpt, (12, 0, 0), &Block::Farmland {
        wetness: Int0Through7::new(7).unwrap()
    });

    let block = excerpt.get_block_at((13, 0, 0).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::West));
    let block = excerpt.get_block_at((13, 0, 1).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::South));
    let block = excerpt.get_block_at((13, 0, 2).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::East));
    let block = excerpt.get_block_at((13, 0, 3).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::North));

    // NB Block ID 62 "lit furnace" should be here at x position 14,
    // but it is not present in the save file.

    let block = excerpt.get_block_at((15, 0, 0).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::South));
    let block = excerpt.get_block_at((15, 0, 1).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthSouthWest));
    let block = excerpt.get_block_at((15, 0, 2).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthWest));
    let block = excerpt.get_block_at((15, 0, 3).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::WestSouthWest));
    let block = excerpt.get_block_at((15, 0, 4).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::West));
    let block = excerpt.get_block_at((15, 0, 5).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::WestNorthWest));
    let block = excerpt.get_block_at((15, 0, 6).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthWest));
    let block = excerpt.get_block_at((15, 0, 7).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthNorthWest));
    let block = excerpt.get_block_at((15, 0, 8).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::North));
    let block = excerpt.get_block_at((15, 0, 9).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthNorthEast));
    let block = excerpt.get_block_at((15, 0, 10).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthEast));
    let block = excerpt.get_block_at((15, 0, 11).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::EastNorthEast));
    let block = excerpt.get_block_at((15, 0, 12).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::East));
    let block = excerpt.get_block_at((15, 0, 13).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::EastSouthEast));
    let block = excerpt.get_block_at((15, 0, 14).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthEast));
    let block = excerpt.get_block_at((15, 0, 15).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthSouthEast));
}
