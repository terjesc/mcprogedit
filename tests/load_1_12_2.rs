use std::convert::TryFrom;

use mcprogedit::block::*;
use mcprogedit::bounded_ints::*;
use mcprogedit::colour::Colour;
use mcprogedit::coordinates::BlockCoord;
use mcprogedit::material::Material;
use mcprogedit::positioning::*;
use mcprogedit::world_excerpt::WorldExcerpt;

const INPUT_FILE: &str = "tests/saves/1_12_2/";
//const INPUT_FILE: &str = "tests/output/1_12_2/";

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
    assert_eq!(excerpt.block_at(at.into()), Some(block));
}

#[rustfmt::skip]
fn check_bed(we: &WorldExcerpt, at: (i64, i64, i64), fac: Direction, col: Colour, end: BedEnd) {
    let block = we.block_at(at.into()).unwrap();
    let bed = Bed::try_from(block.clone()).unwrap();
    assert!(bed.has_facing_of(fac));
    assert!(bed.has_colour_of(col));
    assert!(bed.end == end);
}

fn check_stairs_multiple(excerpt: &WorldExcerpt, at: (i64, i64, i64), material: Material) {
    let (x, y, z) = at;
    check_stairs(&excerpt, (x, y, z), Direction::DownEast, material);
    check_stairs(&excerpt, (x, y, z + 1), Direction::UpEast, material);
    check_stairs(&excerpt, (x, y, z + 2), Direction::DownNorth, material);
    check_stairs(&excerpt, (x, y, z + 3), Direction::UpNorth, material);
    check_stairs(&excerpt, (x, y, z + 4), Direction::DownWest, material);
    check_stairs(&excerpt, (x, y, z + 5), Direction::UpWest, material);
    check_stairs(&excerpt, (x, y, z + 6), Direction::DownSouth, material);
    check_stairs(&excerpt, (x, y, z + 7), Direction::UpSouth, material);
}

fn check_stairs(we: &WorldExcerpt, at: (i64, i64, i64), dir: Direction, mat: Material) {
    let block = we.block_at(at.into()).unwrap();
    assert!(block.is_stairs());
    assert!(block.has_facing_of(dir));
    assert!(block.has_material_of(mat));
}

#[rustfmt::skip]
fn check_closed_doors(excerpt: &WorldExcerpt, at: (i64, i64, i64), material: Material) {
    let (x, y, z) = at;
    check_door(&excerpt, (x, y, z + 0), Direction::West, material, Hinge::Left, true);
    check_door(&excerpt, (x, y, z + 1), Direction::West, material, Hinge::Right, true);
    check_door(&excerpt, (x, y, z + 2), Direction::East, material, Hinge::Right, true);
    check_door(&excerpt, (x, y, z + 3), Direction::East, material, Hinge::Left, true);
}

#[rustfmt::skip]
fn check_open_doors(excerpt: &WorldExcerpt, at: (i64, i64, i64), material: Material) {
    let (x, y, z) = at;
    check_door(&excerpt, (x, y, z + 0), Direction::West, material, Hinge::Left, false);
    check_door(&excerpt, (x, y, z + 1), Direction::West, material, Hinge::Right, false);
    check_door(&excerpt, (x, y, z + 2), Direction::East, material, Hinge::Right, false);
    check_door(&excerpt, (x, y, z + 3), Direction::East, material, Hinge::Left, false);
}

fn check_door(
    excerpt: &WorldExcerpt,
    at: (i64, i64, i64),
    fac: Direction,
    mat: Material,
    hinge: Hinge,
    closed: bool,
) {
    let at = at.into();
    let bottom_block = excerpt.block_at(at).unwrap();
    let top_block = excerpt.block_at(at + (0, 1, 0).into()).unwrap();
    let bottom_door = Door::try_from(bottom_block.clone()).unwrap();
    let top_door = Door::try_from(top_block.clone()).unwrap();
    assert!(bottom_door.has_material_of(&mat));
    assert!(top_door.has_material_of(&mat));
    assert!(bottom_door.has_facing_of(fac));
    assert!(top_door.has_facing_of(fac));
    assert!(bottom_door.is_hinged_at(&hinge));
    assert!(top_door.is_hinged_at(&hinge));
    assert!(bottom_door.is_bottom_half());
    assert!(top_door.is_top_half());
    assert_eq!(closed, bottom_door.is_closed());
    assert_eq!(closed, top_door.is_closed());
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 0 through 15
fn v_1_12_2_block_group_1() {
    let excerpt = load_excerpt(INPUT_FILE, (0, 56, 0), (16, 2, 8));

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
#[rustfmt::skip]
/// Import of blocks with id 16 through 31
fn v_1_12_2_block_group_2() {
    let excerpt = load_excerpt(INPUT_FILE, (16, 56, 0), (16, 5, 16));

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

    assert_block_eq(&excerpt, (4, 0, 0), &Block::glass());

    assert_block_eq(&excerpt, (5, 0, 0), &Block::LapisLazuliOre);

    assert_block_eq(&excerpt, (6, 0, 0), &Block::LapisLazuliBlock);

    let block = excerpt.block_at((7, 0, 0).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((7, 0, 2).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((7, 0, 4).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((7, 0, 6).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::North));
    let block = excerpt.block_at((7, 0, 8).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::Up));
    let block = excerpt.block_at((7, 0, 10).into()).unwrap();
    assert!(block.is_dispenser() && block.has_facing_of(Direction::Down));

    assert_block_eq(&excerpt, (8, 0, 0), &Block::Sandstone);
    assert_block_eq(&excerpt, (8, 0, 1), &Block::ChiseledSandstone);
    assert_block_eq(&excerpt, (8, 0, 2), &Block::SmoothSandstone);

    fn check_noteblock(excerpt: &WorldExcerpt, at: (i64, i64, i64), pitch: Pitch) {
        let block = excerpt.block_at(at.into()).unwrap();
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

    check_bed(&excerpt, (10, 0, 0), Direction::North, Colour::Red, BedEnd::Head);
    check_bed(&excerpt, (10, 0, 1), Direction::North, Colour::Red, BedEnd::Foot);

    assert_block_eq(&excerpt, (11, 0, 0), &Block::powered_rail(RailShape::EastWest));

    assert_block_eq(&excerpt, (12, 0, 0), &Block::detector_rail(RailShape::EastWest));

    let block = excerpt.block_at((13, 0, 0).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((13, 0, 2).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((13, 0, 4).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((13, 0, 6).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::North));
    let block = excerpt.block_at((13, 0, 8).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::Up));
    let block = excerpt.block_at((13, 0, 10).into()).unwrap();
    assert!(block.is_sticky_piston() && block.has_facing_of(Direction::Down));

    assert_block_eq(&excerpt, (14, 0, 0), &Block::Cobweb);

    assert_block_eq(&excerpt, (15, 0, 1), &Block::Grass(Grass::Grass));
    assert_block_eq(&excerpt, (15, 0, 2), &Block::Grass(Grass::Fern));
}

fn check_with_colour_multiple(
    excerpt: &WorldExcerpt,
    at: (i64, i64, i64),
    f: &dyn Fn(Colour) -> Block,
) {
    let (x, y, z) = at;
    assert_block_eq(&excerpt, (x, y, z + 0), &f(Colour::White));
    assert_block_eq(&excerpt, (x, y, z + 1), &f(Colour::Orange));
    assert_block_eq(&excerpt, (x, y, z + 2), &f(Colour::Magenta));
    assert_block_eq(&excerpt, (x, y, z + 3), &f(Colour::LightBlue));
    assert_block_eq(&excerpt, (x, y, z + 4), &f(Colour::Yellow));
    assert_block_eq(&excerpt, (x, y, z + 5), &f(Colour::Lime));
    assert_block_eq(&excerpt, (x, y, z + 6), &f(Colour::Pink));
    assert_block_eq(&excerpt, (x, y, z + 7), &f(Colour::Gray));
    assert_block_eq(&excerpt, (x, y, z + 8), &f(Colour::LightGray));
    assert_block_eq(&excerpt, (x, y, z + 9), &f(Colour::Cyan));
    assert_block_eq(&excerpt, (x, y, z + 10), &f(Colour::Purple));
    assert_block_eq(&excerpt, (x, y, z + 11), &f(Colour::Blue));
    assert_block_eq(&excerpt, (x, y, z + 12), &f(Colour::Brown));
    assert_block_eq(&excerpt, (x, y, z + 13), &f(Colour::Green));
    assert_block_eq(&excerpt, (x, y, z + 14), &f(Colour::Red));
    assert_block_eq(&excerpt, (x, y, z + 15), &f(Colour::Black));
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 32 through 47
fn v_1_12_2_block_group_3() {
    let excerpt = load_excerpt(INPUT_FILE, (32, 56, 0), (16, 2, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::DeadBush);

    let block = excerpt.block_at((1, 0, 0).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((1, 0, 2).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((1, 0, 4).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((1, 0, 6).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::North));
    let block = excerpt.block_at((1, 0, 8).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::Up));
    let block = excerpt.block_at((1, 0, 10).into()).unwrap();
    assert!(block.is_piston() && block.has_facing_of(Direction::Down));

    let block = excerpt.block_at((2, 0, 4).into()).unwrap();
    assert!(block.is_piston_head() && block.has_facing_of(Direction::East));

    check_with_colour_multiple(&excerpt, (3, 0, 0), &Block::wool_with_colour);

    // NB block with ID 36, which should be here at x position 4,
    // is not implemented and is not present in the save file.

    assert_block_eq(&excerpt, (5, 0, 0), &Block::Flower(Flower::Dandelion));

    assert_block_eq(&excerpt, (6, 0, 0), &Block::Flower(Flower::Poppy));
    assert_block_eq(&excerpt, (6, 0, 1), &Block::Flower(Flower::BlueOrchid));
    assert_block_eq(&excerpt, (6, 0, 2), &Block::Flower(Flower::Allium));
    assert_block_eq(&excerpt, (6, 0, 3), &Block::Flower(Flower::AzureBluet));
    assert_block_eq(&excerpt, (6, 0, 4), &Block::Flower(Flower::TulipRed));
    assert_block_eq(&excerpt, (6, 0, 5), &Block::Flower(Flower::TulipOrange));
    assert_block_eq(&excerpt, (6, 0, 6), &Block::Flower(Flower::TulipWhite));
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
#[rustfmt::skip]
/// Import of blocks with id 48 through 63
fn v_1_12_2_block_group_4() {
    let excerpt = load_excerpt(INPUT_FILE, (48, 56, 0), (16, 5, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::MossyCobblestone);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::Obsidian);

    let block = excerpt.block_at((2, 0, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::Up));
    let block = excerpt.block_at((2, 1, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((2, 2, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((2, 3, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((2, 4, 0).into()).unwrap();
    assert!(block.is_torch() && block.has_facing_of(Direction::North));

    let mut fire_block = Block::fire();
    fire_block.set_age_to(15);
    assert_block_eq(&excerpt, (3, 0, 15), &fire_block);

    // NB Block ID 52 "mob spawner" should be here at x position 4,
    // but it is not present in the save file, and is also not implemented.

    check_stairs_multiple(&excerpt, (5, 0, 0), Material::Oak);

    let block = excerpt.block_at((6, 0, 0).into()).unwrap();
    assert!(block.is_chest());
    assert!(block.has_facing_of(Direction::West));
    let block = excerpt.block_at((6, 1, 0).into()).unwrap();
    assert!(block.is_chest());
    assert!(block.has_facing_of(Direction::South));
    let block = excerpt.block_at((6, 2, 0).into()).unwrap();
    assert!(block.is_chest());
    assert!(block.has_facing_of(Direction::East));
    let block = excerpt.block_at((6, 3, 0).into()).unwrap();
    assert!(block.is_chest());
    assert!(block.has_facing_of(Direction::North));

    // NB No check for signal strength on the redstone wire.
    assert_block_eq(&excerpt, (7, 0, 0), &Block::RedstoneWire);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::DiamondOre);

    assert_block_eq(&excerpt, (9, 0, 0), &Block::BlockOfDiamond);

    assert_block_eq(&excerpt, (10, 0, 0), &Block::CraftingTable);

    assert_block_eq(&excerpt, (11, 0, 0), &Block::wheat());

    assert_block_eq(&excerpt, (12, 0, 0),
        &Block::Farmland { wetness: Int0Through7::new(7).unwrap() });

    let block = excerpt.block_at((13, 0, 0).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((13, 0, 1).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((13, 0, 2).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((13, 0, 3).into()).unwrap();
    assert!(block.is_furnace() && block.has_facing_of(Direction::North));

    // NB Block ID 62 "lit furnace" should be here at x position 14,
    // but it is not present in the save file.

    let block = excerpt.block_at((15, 0, 0).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((15, 0, 1).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthSouthWest));
    let block = excerpt.block_at((15, 0, 2).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthWest));
    let block = excerpt.block_at((15, 0, 3).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::WestSouthWest));
    let block = excerpt.block_at((15, 0, 4).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((15, 0, 5).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::WestNorthWest));
    let block = excerpt.block_at((15, 0, 6).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthWest));
    let block = excerpt.block_at((15, 0, 7).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthNorthWest));
    let block = excerpt.block_at((15, 0, 8).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::North));
    let block = excerpt.block_at((15, 0, 9).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthNorthEast));
    let block = excerpt.block_at((15, 0, 10).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::NorthEast));
    let block = excerpt.block_at((15, 0, 11).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::EastNorthEast));
    let block = excerpt.block_at((15, 0, 12).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((15, 0, 13).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::EastSouthEast));
    let block = excerpt.block_at((15, 0, 14).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthEast));
    let block = excerpt.block_at((15, 0, 15).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::SouthSouthEast));
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 64 through 79
fn v_1_12_2_block_group_5() {
    let excerpt = load_excerpt(INPUT_FILE, (64, 56, 0), (16, 11, 16));

    check_closed_doors(&excerpt, (0, 0, 0), Material::Oak);
    check_open_doors(&excerpt, (0, 0, 4), Material::Oak);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::ladder(Direction::West));
    assert_block_eq(&excerpt, (1, 1, 0), &Block::ladder(Direction::South));
    assert_block_eq(&excerpt, (1, 2, 0), &Block::ladder(Direction::East));
    assert_block_eq(&excerpt, (1, 3, 0), &Block::ladder(Direction::North));

    assert_block_eq(&excerpt, (2, 1, 0), &Block::rail(RailShape::NorthSouth));

    check_stairs_multiple(&excerpt, (3, 0, 0), Material::Cobblestone);

    let block = excerpt.block_at((4, 0, 0).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((4, 1, 0).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((4, 2, 0).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((4, 3, 0).into()).unwrap();
    assert!(block.is_sign() && block.has_facing_of(Direction::North));

    // Mounted on the bottom (attached to block below)
    assert_block_eq(&excerpt, (5, 0, 1), &Block::lever_off(Direction::UpEast));
    assert_block_eq(&excerpt, (5, 0, 2), &Block::lever_off(Direction::UpSouth));
    assert_block_eq(&excerpt, (5, 0, 3), &Block::lever_off(Direction::UpEast)); // east again
    assert_block_eq(&excerpt, (5, 0, 4), &Block::lever_off(Direction::UpSouth)); // south again
                                                                                 // Turned on, mounted on the bottom (attached to block below)
    assert_block_eq(&excerpt, (5, 0, 5), &Block::lever_on(Direction::UpEast));
    assert_block_eq(&excerpt, (5, 0, 6), &Block::lever_on(Direction::UpSouth));
    assert_block_eq(&excerpt, (5, 0, 7), &Block::lever_on(Direction::UpEast)); // east again
    assert_block_eq(&excerpt, (5, 0, 8), &Block::lever_on(Direction::UpSouth)); // south again
                                                                                // Mounted on a side
    assert_block_eq(&excerpt, (5, 1, 1), &Block::lever_off(Direction::West));
    assert_block_eq(&excerpt, (5, 1, 3), &Block::lever_off(Direction::South));
    assert_block_eq(&excerpt, (5, 1, 5), &Block::lever_off(Direction::East));
    assert_block_eq(&excerpt, (5, 1, 7), &Block::lever_off(Direction::North));
    // Mounted on the top (attached to block above)
    assert_block_eq(&excerpt, (5, 2, 1), &Block::lever_off(Direction::DownEast));
    assert_block_eq(&excerpt, (5, 2, 2), &Block::lever_off(Direction::DownSouth));
    assert_block_eq(&excerpt, (5, 2, 3), &Block::lever_off(Direction::DownEast)); // east again
    assert_block_eq(&excerpt, (5, 2, 1), &Block::lever_off(Direction::DownEast)); // south again

    assert_block_eq(&excerpt, (6, 0, 0), &Block::pressure_plate(Material::Stone));

    check_door(&excerpt, (7, 0, 0), Direction::West, Material::Iron, Hinge::Left, true);
    check_door(&excerpt, (7, 0, 1), Direction::West, Material::Iron, Hinge::Right, true);
    check_door(&excerpt, (7, 0, 2), Direction::East, Material::Iron, Hinge::Right, true);
    check_door(&excerpt, (7, 0, 3), Direction::East, Material::Iron, Hinge::Left, true);
    check_door(&excerpt, (7, 0, 4), Direction::West, Material::Iron, Hinge::Left, false);
    check_door(&excerpt, (7, 0, 5), Direction::West, Material::Iron, Hinge::Right, false);
    check_door(&excerpt, (7, 0, 6), Direction::East, Material::Iron, Hinge::Right, false);
    check_door(&excerpt, (7, 0, 7), Direction::East, Material::Iron, Hinge::Left, false);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::pressure_plate(Material::Oak));

    assert_block_eq(&excerpt, (9, 0, 0), &Block::RedstoneOre);

    // NB Block ID 74 "lit redstone ore" should be here at x position 10,
    // but it is not present in the save file, and lit status is not implemented.

    // NB Block ID 75 "unlit redstone torch" should be here at x position 11,
    // but is placed above the lit variant at x position 12 instead.

    // NB "lit" status of redstone torch is not implemented.
    // "unlit" variants are present in the save file, however, for future implementation.
    // Lit torches
    let block = excerpt.block_at((12, 0, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::Up));
    let block = excerpt.block_at((12, 1, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((12, 2, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((12, 3, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((12, 4, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::North));
    // Unlit torches
    let block = excerpt.block_at((12, 6, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::Up));
    let block = excerpt.block_at((12, 7, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((12, 8, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((12, 9, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((12, 10, 0).into()).unwrap();
    assert!(block.is_redstone_torch() && block.has_facing_of(Direction::North));

    assert_block_eq(&excerpt, (13, 0, 0), &Block::stone_button(Direction::Up));
    assert_block_eq(&excerpt, (13, 1, 4), &Block::stone_button(Direction::West));
    assert_block_eq(&excerpt, (13, 2, 4), &Block::stone_button(Direction::South));
    assert_block_eq(&excerpt, (13, 3, 4), &Block::stone_button(Direction::East));
    assert_block_eq(&excerpt, (13, 4, 4), &Block::stone_button(Direction::North));
    assert_block_eq(&excerpt, (13, 1, 5), &Block::stone_button(Direction::Down));

    assert_block_eq(&excerpt, (14, 0, 1), &Block::snow_layer());
    assert_block_eq(&excerpt, (14, 0, 2), &Block::snow_layers(2));
    assert_block_eq(&excerpt, (14, 0, 3), &Block::snow_layers(3));
    assert_block_eq(&excerpt, (14, 0, 4), &Block::snow_layers(4));
    assert_block_eq(&excerpt, (14, 0, 5), &Block::snow_layers(5));
    assert_block_eq(&excerpt, (14, 0, 6), &Block::snow_layers(6));
    assert_block_eq(&excerpt, (14, 0, 7), &Block::snow_layers(7));
    assert_block_eq(&excerpt, (14, 0, 8), &Block::snow_layers(8));

    assert_block_eq(&excerpt, (15, 0, 0), &Block::Ice);
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 80 through 95
fn v_1_12_2_block_group_6() {
    let excerpt = load_excerpt(INPUT_FILE, (80, 56, 0), (16, 3, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::SnowBlock);

    assert_block_eq(&excerpt, (1, 1, 0), &Block::cactus());

    assert_block_eq(&excerpt, (2, 0, 0), &Block::Clay);

    assert_block_eq(&excerpt, (3, 0, 0), &Block::sugar_cane());

    use mcprogedit::item::Recording;
    assert_block_eq(&excerpt, (4, 0, 0), &Block::jukebox());
    assert_block_eq(&excerpt, (4, 0, 1), &Block::jukebox_with_recording(Recording::Thirteen));
    assert_block_eq(&excerpt, (4, 0, 2), &Block::jukebox_with_recording(Recording::Cat));
    assert_block_eq(&excerpt, (4, 0, 3), &Block::jukebox_with_recording(Recording::Blocks));
    assert_block_eq(&excerpt, (4, 0, 4), &Block::jukebox_with_recording(Recording::Chirp));
    assert_block_eq(&excerpt, (4, 0, 5), &Block::jukebox_with_recording(Recording::Far));
    assert_block_eq(&excerpt, (4, 0, 6), &Block::jukebox_with_recording(Recording::Mall));
    assert_block_eq(&excerpt, (4, 0, 7), &Block::jukebox_with_recording(Recording::Mellohi));
    assert_block_eq(&excerpt, (4, 0, 8), &Block::jukebox_with_recording(Recording::Stal));
    assert_block_eq(&excerpt, (4, 0, 9), &Block::jukebox_with_recording(Recording::Strad));
    assert_block_eq(&excerpt, (4, 0, 10), &Block::jukebox_with_recording(Recording::Ward));
    assert_block_eq(&excerpt, (4, 0, 11), &Block::jukebox_with_recording(Recording::Eleven));
    assert_block_eq(&excerpt, (4, 0, 12), &Block::jukebox_with_recording(Recording::Wait));

    assert_block_eq(&excerpt, (5, 0, 0), &Block::oak_fence());

    assert_block_eq(&excerpt, (6, 0, 0), &Block::pumpkin(Direction::West));
    assert_block_eq(&excerpt, (6, 0, 2), &Block::pumpkin(Direction::South));
    assert_block_eq(&excerpt, (6, 0, 4), &Block::pumpkin(Direction::East));
    assert_block_eq(&excerpt, (6, 0, 6), &Block::pumpkin(Direction::North));

    assert_block_eq(&excerpt, (7, 0, 0), &Block::Netherrack);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::SoulSand);

    assert_block_eq(&excerpt, (9, 0, 0), &Block::Glowstone);

    // NB Alignment of nether portal blocks is not implemented.
    assert_block_eq(
        &excerpt,
        (10, 0, 0),
        &Block::NetherPortal { alignment: None });

    assert_block_eq(&excerpt, (11, 0, 0), &Block::jack_o_lantern(Direction::West));
    assert_block_eq(&excerpt, (11, 0, 2), &Block::jack_o_lantern(Direction::South));
    assert_block_eq(&excerpt, (11, 0, 4), &Block::jack_o_lantern(Direction::East));
    assert_block_eq(&excerpt, (11, 0, 6), &Block::jack_o_lantern(Direction::North));

    assert_block_eq(&excerpt, (12, 2, 0), &Block::cake());
    assert_block_eq(&excerpt, (12, 2, 1), &Block::cake_with_remaining_pieces(6));
    assert_block_eq(&excerpt, (12, 2, 2), &Block::cake_with_remaining_pieces(5));
    assert_block_eq(&excerpt, (12, 2, 3), &Block::cake_with_remaining_pieces(4));
    assert_block_eq(&excerpt, (12, 2, 4), &Block::cake_with_remaining_pieces(3));
    assert_block_eq(&excerpt, (12, 2, 5), &Block::cake_with_remaining_pieces(2));
    assert_block_eq(&excerpt, (12, 2, 6), &Block::cake_with_remaining_pieces(1));

    fn check_repeater(we: &WorldExcerpt, at: (i64, i64, i64), delay: i8, dir: Direction) {
        let block = we.block_at(at.into()).unwrap();
        let repeater = RedstoneRepeater::try_from(block.clone()).unwrap();
        assert!(repeater.has_facing_of(dir) && repeater.has_delay_of(delay));
    }

    // NB 13 "redstone repeater":
    // * the one at (13, 0, 0) is locked
    // * the one at (13, 0, 3) is powered
    check_repeater(&excerpt, (13, 0, 0), 1, Direction::East);
    check_repeater(&excerpt, (13, 0, 1), 2, Direction::East);
    check_repeater(&excerpt, (13, 0, 2), 3, Direction::East);
    check_repeater(&excerpt, (13, 0, 3), 4, Direction::East);
    check_repeater(&excerpt, (13, 0, 4), 1, Direction::North);
    check_repeater(&excerpt, (13, 0, 5), 2, Direction::North);
    check_repeater(&excerpt, (13, 0, 6), 3, Direction::North);
    check_repeater(&excerpt, (13, 0, 7), 4, Direction::North);
    check_repeater(&excerpt, (13, 0, 8), 1, Direction::West);
    check_repeater(&excerpt, (13, 0, 9), 2, Direction::West);
    check_repeater(&excerpt, (13, 0, 10), 3, Direction::West);
    check_repeater(&excerpt, (13, 0, 11), 4, Direction::West);
    check_repeater(&excerpt, (13, 0, 12), 1, Direction::South);
    check_repeater(&excerpt, (13, 0, 13), 2, Direction::South);
    check_repeater(&excerpt, (13, 0, 14), 3, Direction::South);
    check_repeater(&excerpt, (13, 0, 15), 4, Direction::South);

    // NB 14 "powered redstone repeater"
    // * the one at (14, 0, 3 is locked.
    check_repeater(&excerpt, (14, 0, 0), 1, Direction::South);
    check_repeater(&excerpt, (14, 0, 1), 2, Direction::South);
    check_repeater(&excerpt, (14, 0, 2), 3, Direction::South);
    check_repeater(&excerpt, (14, 0, 3), 4, Direction::South);

    check_with_colour_multiple(&excerpt, (15, 0, 0), &Block::glass_with_colour);
}

fn check_trapdoor_multiple(excerpt: &WorldExcerpt, at: (i64, i64, i64), material: Material) {
    let (x, y, z) = at;
    check_trapdoor(&excerpt, (x, y, z + 0), material, Edge8::DownEast, true);
    check_trapdoor(&excerpt, (x, y, z + 1), material, Edge8::DownNorth, true);
    check_trapdoor(&excerpt, (x, y, z + 2), material, Edge8::DownWest, true);
    check_trapdoor(&excerpt, (x, y, z + 3), material, Edge8::DownSouth, true);
    check_trapdoor(&excerpt, (x, y, z + 4), material, Edge8::UpEast, true);
    check_trapdoor(&excerpt, (x, y, z + 5), material, Edge8::UpNorth, true);
    check_trapdoor(&excerpt, (x, y, z + 6), material, Edge8::UpWest, true);
    check_trapdoor(&excerpt, (x, y, z + 7), material, Edge8::UpSouth, true);
    check_trapdoor(&excerpt, (x, y, z + 8), material, Edge8::DownEast, false);
    if material == Material::Iron {
        // There is an error in the save file, for Iron trapdoors.
        // Should be fixed in save file, then check DownNorth for all on this position.
        check_trapdoor(&excerpt, (x, y, z + 9), material, Edge8::UpNorth, false);
    } else {
        check_trapdoor(&excerpt, (x, y, z + 9), material, Edge8::DownNorth, false);
    }
    check_trapdoor(&excerpt, (x, y, z + 10), material, Edge8::DownWest, false);
    check_trapdoor(&excerpt, (x, y, z + 11), material, Edge8::DownSouth, false);
    check_trapdoor(&excerpt, (x, y, z + 12), material, Edge8::UpEast, false);
    check_trapdoor(&excerpt, (x, y, z + 13), material, Edge8::UpNorth, false);
    check_trapdoor(&excerpt, (x, y, z + 14), material, Edge8::UpWest, false);
    check_trapdoor(&excerpt, (x, y, z + 15), material, Edge8::UpSouth, false);
}

fn check_trapdoor(
    excerpt: &WorldExcerpt,
    at: (i64, i64, i64),
    material: Material,
    hinge_at: Edge8,
    closed: bool,
) {
    let block = excerpt.block_at(at.into()).unwrap();
    let trapdoor = Trapdoor::try_from(block.clone()).unwrap();
    assert!(trapdoor.has_hinge_at(hinge_at));
    assert!(trapdoor.has_material_of(material));
    assert_eq!(closed, trapdoor.is_closed());
}

fn check_vines(excerpt: &WorldExcerpt, at: (i64, i64, i64), direction: Direction) {
    let block = excerpt.block_at(at.into()).unwrap();
    let vines = Vines::try_from(block.clone()).unwrap();
    assert!(vines.is_touching_surface(direction));
}

fn check_fence_gate_multiple(
    excerpt: &WorldExcerpt,
    at: (i64, i64, i64),
    f: &dyn Fn(Direction) -> Block,
) {
    let (x, y, z) = at;
    assert_block_eq(&excerpt, (x, y, z + 0), &f(Direction::East));
    assert_block_eq(&excerpt, (x, y, z + 1), &f(Direction::North));
    assert_block_eq(&excerpt, (x, y, z + 2), &f(Direction::West));
    assert_block_eq(&excerpt, (x, y, z + 3), &f(Direction::South));
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 96 through 111
fn v_1_12_2_block_group_7() {
    let excerpt = load_excerpt(INPUT_FILE, (96, 56, 0), (16, 4, 16));

    check_trapdoor_multiple(&excerpt, (0, 0, 0), Material::Oak);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::InfestedStone);
    assert_block_eq(&excerpt, (1, 0, 1), &Block::InfestedCobblestone);
    assert_block_eq(&excerpt, (1, 0, 2), &Block::InfestedStoneBricks);
    assert_block_eq(&excerpt, (1, 0, 3), &Block::InfestedMossyStoneBricks);
    assert_block_eq(&excerpt, (1, 0, 4), &Block::InfestedCrackedStoneBricks);
    assert_block_eq(&excerpt, (1, 0, 5), &Block::InfestedChiseledStoneBricks);

    assert_block_eq(&excerpt, (2, 0, 0), &Block::StoneBricks);
    assert_block_eq(&excerpt, (2, 0, 1), &Block::MossyStoneBricks);
    assert_block_eq(&excerpt, (2, 0, 2), &Block::CrackedStoneBricks);
    assert_block_eq(&excerpt, (2, 0, 3), &Block::ChiseledStoneBricks);

    // NB 99 "brown mushroom block" not in save file
    // NB 100 "red mushroom block" not in save file

    assert_block_eq(&excerpt, (5, 0, 0), &Block::iron_bars());

    assert_block_eq(&excerpt, (6, 0, 0), &Block::glass_pane());

    assert_block_eq(&excerpt, (7, 0, 0), &Block::Melon);

    assert_block_eq(&excerpt, (8, 0, 0),
        &Block::PumpkinStem { state: StemState::Growing(Int0Through7::new_saturating(2)) });

    assert_block_eq(&excerpt, (9, 0, 0),
        &Block::MelonStem { state: StemState::Growing(Int0Through7::new_saturating(5)) });

    check_vines(&excerpt, (10, 0, 0), Direction::East);
    check_vines(&excerpt, (10, 1, 0), Direction::North);
    check_vines(&excerpt, (10, 2, 0), Direction::West);
    check_vines(&excerpt, (10, 3, 0), Direction::South);

    check_fence_gate_multiple(&excerpt, (11, 0, 2), &Block::oak_fence_gate);
    check_fence_gate_multiple(&excerpt, (11, 0, 6), &Block::oak_fence_gate_opened);

    check_stairs_multiple(&excerpt, (12, 0, 0), Material::Brick);

    check_stairs_multiple(&excerpt, (13, 0, 0), Material::StoneBrick);

    assert_block_eq(&excerpt, (14, 0, 0), &Block::Mycelium);

    assert_block_eq(&excerpt, (15, 0, 0), &Block::LilyPad);
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 112 through 127
fn v_1_12_2_block_group_8() {
    let excerpt = load_excerpt(INPUT_FILE, (112, 56, 0), (16, 4, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::NetherBricks);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::nether_brick_fence());

    check_stairs_multiple(&excerpt, (2, 0, 0), Material::NetherBrick);

    assert_block_eq(&excerpt, (3, 0, 0),
        &Block::NetherWart { growth_stage: Int0Through3::new_saturating(3) });

    assert!(excerpt.block_at((4, 0, 0).into()).unwrap().is_enchanting_table());

    // NB brewing stand has a number of fields that should be tested,
    // but in the current test map all of them are default.
    assert!(excerpt.block_at((5, 0, 0).into()).unwrap().is_brewing_stand());

    assert_block_eq(&excerpt, (6, 0, 0),
        &Block::Cauldron { water_level: Int0Through3::new_saturating(0) });
    assert_block_eq(&excerpt, (6, 0, 1),
        &Block::Cauldron { water_level: Int0Through3::new_saturating(1) });
    assert_block_eq(&excerpt, (6, 0, 2),
        &Block::Cauldron { water_level: Int0Through3::new_saturating(2) });
    assert_block_eq(&excerpt, (6, 0, 3),
        &Block::Cauldron { water_level: Int0Through3::new_saturating(3) });

    // NB 119 "end portal" not present in save file

    assert_block_eq(&excerpt, (8, 0, 0),
        &Block::EndPortalFrame { facing: Surface4::West, has_eye: false });
    assert_block_eq(&excerpt, (8, 1, 0),
        &Block::EndPortalFrame { facing: Surface4::South, has_eye: false });
    assert_block_eq(&excerpt, (8, 2, 0),
        &Block::EndPortalFrame { facing: Surface4::East, has_eye: false });
    assert_block_eq(&excerpt, (8, 3, 0),
        &Block::EndPortalFrame { facing: Surface4::North, has_eye: false });
    assert_block_eq(&excerpt, (8, 0, 1),
        &Block::EndPortalFrame { facing: Surface4::West, has_eye: true });
    assert_block_eq(&excerpt, (8, 1, 1),
        &Block::EndPortalFrame { facing: Surface4::South, has_eye: true });
    assert_block_eq(&excerpt, (8, 2, 1),
        &Block::EndPortalFrame { facing: Surface4::East, has_eye: true });
    assert_block_eq(&excerpt, (8, 3, 1),
        &Block::EndPortalFrame { facing: Surface4::North, has_eye: true });

    assert_block_eq(&excerpt, (9, 0, 0), &Block::EndStone);

    assert_block_eq(&excerpt, (10, 0, 0), &Block::DragonEgg);

    assert_block_eq(&excerpt, (11, 0, 0), &Block::RedstoneLamp);

    // NB 124 "lit redstone lamp" not tested: No on/off state stored.

    assert_block_eq(&excerpt, (13, 0, 0), &Block::double_slab(Material::Oak));
    assert_block_eq(&excerpt, (13, 0, 1), &Block::double_slab(Material::Spruce));
    assert_block_eq(&excerpt, (13, 0, 2), &Block::double_slab(Material::Birch));
    assert_block_eq(&excerpt, (13, 0, 3), &Block::double_slab(Material::Jungle));
    assert_block_eq(&excerpt, (13, 0, 4), &Block::double_slab(Material::Acacia));
    assert_block_eq(&excerpt, (13, 0, 5), &Block::double_slab(Material::DarkOak));

    assert_block_eq(&excerpt, (14, 0, 0), &Block::bottom_slab(Material::Oak));
    assert_block_eq(&excerpt, (14, 1, 0), &Block::top_slab(Material::Oak));
    assert_block_eq(&excerpt, (14, 0, 1), &Block::bottom_slab(Material::Spruce));
    assert_block_eq(&excerpt, (14, 1, 1), &Block::top_slab(Material::Spruce));
    assert_block_eq(&excerpt, (14, 0, 2), &Block::bottom_slab(Material::Birch));
    assert_block_eq(&excerpt, (14, 1, 2), &Block::top_slab(Material::Birch));
    assert_block_eq(&excerpt, (14, 0, 3), &Block::bottom_slab(Material::Jungle));
    assert_block_eq(&excerpt, (14, 1, 3), &Block::top_slab(Material::Jungle));
    assert_block_eq(&excerpt, (14, 0, 4), &Block::bottom_slab(Material::Acacia));
    assert_block_eq(&excerpt, (14, 1, 4), &Block::top_slab(Material::Acacia));
    assert_block_eq(&excerpt, (14, 0, 5), &Block::bottom_slab(Material::DarkOak));
    assert_block_eq(&excerpt, (14, 1, 5), &Block::top_slab(Material::DarkOak));

    assert_block_eq(&excerpt, (15, 0, 0),
        &Block::CocoaBeans {
            growth_stage: Int0Through2::new_saturating(2),
            facing: Surface4::South,
        },
    );
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 128 through 143
fn v_1_12_2_block_group_9() {
    let excerpt = load_excerpt(INPUT_FILE, (128, 56, 0), (16, 5, 16));

    check_stairs_multiple(&excerpt, (0, 0, 0), Material::Sandstone);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::EmeraldOre);

    let block = excerpt.block_at((2, 0, 0).into()).unwrap();
    assert!(block.is_ender_chest() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((2, 1, 0).into()).unwrap();
    assert!(block.is_ender_chest() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((2, 2, 0).into()).unwrap();
    assert!(block.is_ender_chest() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((2, 3, 0).into()).unwrap();
    assert!(block.is_ender_chest() && block.has_facing_of(Direction::North));

    // NB Tripwire hooks only have facing implemented at the moment, but there are
    // several additional tripwire hooks in the save file, in different states.
    assert_block_eq(&excerpt, (3, 0, 4), &Block::TripwireHook { facing: Surface4::West });
    assert_block_eq(&excerpt, (3, 1, 4), &Block::TripwireHook { facing: Surface4::South });
    assert_block_eq(&excerpt, (3, 2, 4), &Block::TripwireHook { facing: Surface4::East });
    assert_block_eq(&excerpt, (3, 3, 4), &Block::TripwireHook { facing: Surface4::North});

    assert_block_eq(&excerpt, (4, 0, 0), &Block::Tripwire);

    assert_block_eq(&excerpt, (5, 0, 0), &Block::BlockOfEmerald);

    check_stairs_multiple(&excerpt, (6, 0, 0), Material::Spruce);

    check_stairs_multiple(&excerpt, (7, 0, 0), Material::Birch);

    check_stairs_multiple(&excerpt, (8, 0, 0), Material::Jungle);

    // TODO 9: NB 137 "command block" not implemented and not in save file

    assert!(excerpt.block_at((10, 0, 0).into()).unwrap().is_beacon());

    let block = excerpt.block_at((11, 0, 0).into()).unwrap();
    assert!(block.is_wall() && block.has_material_of(Material::Cobblestone));
    let block = excerpt.block_at((11, 0, 1).into()).unwrap();
    assert!(block.is_wall() && block.has_material_of(Material::MossyCobblestone));

    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 0).into()).unwrap().clone());
    assert!(pot.unwrap().is_empty());
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 1).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::Poppy));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 2).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::Dandelion));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 3).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::OakSapling));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 4).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::SpruceSapling));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 5).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::BirchSapling));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 6).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::JungleSapling));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 7).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::RedMushroom));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 8).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::BrownMushroom));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 9).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::Cactus));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 10).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::DeadBush));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 11).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::Fern));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 12).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::AcaciaSapling));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 0, 13).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::DarkOakSapling));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 0).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::BlueOrchid));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 1).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::Allium));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 2).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::AzureBluet));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 3).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::TulipRed));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 4).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::TulipOrange));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 5).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::TulipWhite));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 6).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::TulipPink));
    let pot = FlowerPot::try_from(excerpt.block_at((12, 2, 7).into()).unwrap().clone());
    assert!(pot.unwrap().has_plant_of(PottedPlant::OxeyeDaisy));

    assert_block_eq(&excerpt, (13, 0, 0),
        &Block::Carrots { growth_stage: Int0Through7::new_saturating(0)});

    assert_block_eq(&excerpt, (14, 0, 0),
        &Block::Potatoes { growth_stage: Int0Through7::new_saturating(1) });

    assert_block_eq(&excerpt, (15, 0, 0), &Block::oak_button(Direction::Up));
    assert_block_eq(&excerpt, (15, 1, 4), &Block::oak_button(Direction::West));
    assert_block_eq(&excerpt, (15, 2, 4), &Block::oak_button(Direction::South));
    assert_block_eq(&excerpt, (15, 3, 4), &Block::oak_button(Direction::East));
    assert_block_eq(&excerpt, (15, 4, 4), &Block::oak_button(Direction::North));
    assert_block_eq(&excerpt, (15, 1, 5), &Block::oak_button(Direction::Down));
}

fn check_skull(
    excerpt: &WorldExcerpt,
    at: (i64, i64, i64),
    head_variant: HeadVariant,
    direction: Direction,
    is_on_floor: bool,
) {
    let block = excerpt.block_at(at.into()).unwrap();
    let head = Head::try_from(block.clone()).unwrap();
    assert!(head.has_variant_of(head_variant));
    assert!(head.has_facing_of(direction));
    assert_eq!(head.is_on_floor(), is_on_floor);
}

fn check_floor_skull(we: &WorldExcerpt, at: (i64, i64, i64), head: HeadVariant, dir: Direction) {
    check_skull(&we, at, head, dir, true);
}

fn check_wall_skull(we: &WorldExcerpt, at: (i64, i64, i64), head: HeadVariant, dir: Direction) {
    check_skull(&we, at, head, dir, false);
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 144 through 159
fn v_1_12_2_block_group_10() {
    let excerpt = load_excerpt(INPUT_FILE, (144, 56, 0), (16, 8, 16));

    {
        use Direction::*;
        use HeadVariant::*;
        check_floor_skull(&excerpt, (0, 0, 0), SkeletonSkull, South);
        check_floor_skull(&excerpt, (0, 0, 1), SkeletonSkull, SouthSouthEast);
        check_floor_skull(&excerpt, (0, 0, 2), SkeletonSkull, SouthEast);
        check_floor_skull(&excerpt, (0, 0, 3), SkeletonSkull, EastSouthEast);
        check_floor_skull(&excerpt, (0, 0, 4), WitherSkeletonSkull, East);
        check_floor_skull(&excerpt, (0, 0, 5), WitherSkeletonSkull, EastNorthEast);
        check_floor_skull(&excerpt, (0, 0, 6), WitherSkeletonSkull, NorthEast);
        check_floor_skull(&excerpt, (0, 0, 7), WitherSkeletonSkull, NorthNorthEast);
        check_floor_skull(&excerpt, (0, 0, 8), ZombieHead, North);
        check_floor_skull(&excerpt, (0, 0, 9), ZombieHead, NorthNorthWest);
        check_floor_skull(&excerpt, (0, 0, 10), ZombieHead, NorthWest);
        check_floor_skull(&excerpt, (0, 0, 11), ZombieHead, WestNorthWest);
        check_floor_skull(&excerpt, (0, 0, 12), CreeperHead, West);
        check_floor_skull(&excerpt, (0, 0, 13), CreeperHead, WestSouthWest);
        check_floor_skull(&excerpt, (0, 0, 14), CreeperHead, SouthWest);
        check_floor_skull(&excerpt, (0, 0, 15), CreeperHead, SouthSouthWest);
        check_floor_skull(&excerpt, (0, 2, 0), PlayerHead, South);
        check_floor_skull(&excerpt, (0, 2, 1), DragonHead, South);
        check_wall_skull(&excerpt, (0, 4, 0), SkeletonSkull, West);
        check_wall_skull(&excerpt, (0, 5, 0), SkeletonSkull, South);
        check_wall_skull(&excerpt, (0, 6, 0), SkeletonSkull, East);
        check_wall_skull(&excerpt, (0, 7, 0), SkeletonSkull, North);
    }

    assert_block_eq(&excerpt, (1, 0, 0),
        &Block::Anvil { facing: Surface4::South, damage: AnvilDamage::Intact });
    assert_block_eq(&excerpt, (1, 1, 0),
        &Block::Anvil { facing: Surface4::East, damage: AnvilDamage::Intact });
    assert_block_eq(&excerpt, (1, 2, 0),
        &Block::Anvil { facing: Surface4::North, damage: AnvilDamage::Intact });
    assert_block_eq(&excerpt, (1, 3, 0),
        &Block::Anvil { facing: Surface4::West, damage: AnvilDamage::Intact });
    assert_block_eq(&excerpt, (1, 0, 1),
        &Block::Anvil { facing: Surface4::South, damage: AnvilDamage::SlightlyDamaged });
    assert_block_eq(&excerpt, (1, 1, 1),
        &Block::Anvil { facing: Surface4::East, damage: AnvilDamage::SlightlyDamaged });
    assert_block_eq(&excerpt, (1, 2, 1),
        &Block::Anvil { facing: Surface4::North, damage: AnvilDamage::SlightlyDamaged });
    assert_block_eq(&excerpt, (1, 3, 1),
        &Block::Anvil { facing: Surface4::West, damage: AnvilDamage::SlightlyDamaged });
    assert_block_eq(&excerpt, (1, 0, 2),
        &Block::Anvil { facing: Surface4::South, damage: AnvilDamage::VeryDamaged });
    assert_block_eq(&excerpt, (1, 1, 2),
        &Block::Anvil { facing: Surface4::East, damage: AnvilDamage::VeryDamaged });
    assert_block_eq(&excerpt, (1, 2, 2),
        &Block::Anvil { facing: Surface4::North, damage: AnvilDamage::VeryDamaged });
    assert_block_eq(&excerpt, (1, 3, 2),
        &Block::Anvil { facing: Surface4::West, damage: AnvilDamage::VeryDamaged });

    let block = excerpt.block_at((2, 0, 0).into()).unwrap();
    assert!(block.is_trapped_chest());
    assert!(block.has_facing_of(Direction::West));
    let block = excerpt.block_at((2, 0, 2).into()).unwrap();
    assert!(block.is_trapped_chest());
    assert!(block.has_facing_of(Direction::South));
    let block = excerpt.block_at((2, 0, 4).into()).unwrap();
    assert!(block.is_trapped_chest());
    assert!(block.has_facing_of(Direction::East));
    let block = excerpt.block_at((2, 0, 6).into()).unwrap();
    assert!(block.is_trapped_chest());
    assert!(block.has_facing_of(Direction::North));

    assert_block_eq(&excerpt, (3, 0, 0), &Block::pressure_plate(Material::Gold));

    assert_block_eq(&excerpt, (4, 0, 0), &Block::pressure_plate(Material::Iron));

    assert_block_eq(&excerpt, (5, 0, 0),
        &Block::RedstoneComparator { facing: Surface4::East });
    assert_block_eq(&excerpt, (5, 0, 1),
        &Block::RedstoneSubtractor { facing: Surface4::East });
    assert_block_eq(&excerpt, (5, 0, 2),
        &Block::RedstoneComparator { facing: Surface4::North });
    assert_block_eq(&excerpt, (5, 0, 3),
        &Block::RedstoneSubtractor { facing: Surface4::North });
    assert_block_eq(&excerpt, (5, 0, 4),
        &Block::RedstoneComparator { facing: Surface4::West });
    assert_block_eq(&excerpt, (5, 0, 5),
        &Block::RedstoneSubtractor { facing: Surface4::West });
    assert_block_eq(&excerpt, (5, 0, 6),
        &Block::RedstoneComparator { facing: Surface4::South });
    assert_block_eq(&excerpt, (5, 0, 7),
        &Block::RedstoneSubtractor { facing: Surface4::South });

    assert_block_eq(&excerpt, (6, 0, 0),
        &Block::RedstoneComparator { facing: Surface4::South });
    assert_block_eq(&excerpt, (6, 0, 1),
        &Block::RedstoneSubtractor { facing: Surface4::South });

    assert_block_eq(&excerpt, (7, 0, 0), &Block::DaylightDetector);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::BlockOfRedstone);

    assert_block_eq(&excerpt, (9, 0, 0), &Block::QuartzOre);

    let block = excerpt.block_at((10, 0, 0).into()).unwrap();
    assert!(block.is_hopper() && block.has_facing_of(Direction::Down));
    let block = excerpt.block_at((10, 0, 2).into()).unwrap();
    assert!(block.is_hopper() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((10, 0, 4).into()).unwrap();
    assert!(block.is_hopper() && block.has_facing_of(Direction::North));
    let block = excerpt.block_at((10, 0, 6).into()).unwrap();
    assert!(block.is_hopper() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((10, 0, 8).into()).unwrap();
    assert!(block.is_hopper() && block.has_facing_of(Direction::South));

    assert_block_eq(&excerpt, (11, 0, 0), &Block::BlockOfQuartz);
    assert_block_eq(&excerpt, (11, 0, 1), &Block::ChiseledQuartzBlock);
    assert_block_eq(&excerpt, (11, 0, 2), &Block::QuartzPillar { alignment: Axis3::Y });
    assert_block_eq(&excerpt, (11, 1, 2), &Block::QuartzPillar { alignment: Axis3::Z });
    assert_block_eq(&excerpt, (11, 2, 2), &Block::QuartzPillar { alignment: Axis3::X });

    check_stairs_multiple(&excerpt, (12, 0, 0), Material::Quartz);

    assert_block_eq(&excerpt, (13, 0, 0), &Block::activator_rail(RailShape::NorthSouth));

    let block = excerpt.block_at((14, 0, 0).into()).unwrap();
    assert!(block.is_dropper() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((14, 0, 2).into()).unwrap();
    assert!(block.is_dropper() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((14, 0, 4).into()).unwrap();
    assert!(block.is_dropper() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((14, 0, 6).into()).unwrap();
    assert!(block.is_dropper() && block.has_facing_of(Direction::North));
    let block = excerpt.block_at((14, 0, 8).into()).unwrap();
    assert!(block.is_dropper() && block.has_facing_of(Direction::Up));
    let block = excerpt.block_at((14, 0, 10).into()).unwrap();
    assert!(block.is_dropper() && block.has_facing_of(Direction::Down));

    check_with_colour_multiple(&excerpt, (15, 0, 0), &Block::terracotta_with_colour);
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 160 through 175
fn v_1_12_2_block_group_11() {
    let excerpt = load_excerpt(INPUT_FILE, (160, 56, 0), (16, 3, 16));

    check_with_colour_multiple(&excerpt, (0, 0, 0), &Block::glass_pane_with_colour);

    assert_block_eq(&excerpt, (1, 0, 0), &Block::acacia_leaves(true));
    assert_block_eq(&excerpt, (1, 0, 1), &Block::dark_oak_leaves(true));

    assert_block_eq(&excerpt, (2, 0, 0), &Block::acacia_log(Axis3::Y));
    assert_block_eq(&excerpt, (2, 1, 0), &Block::acacia_log(Axis3::Z));
    assert_block_eq(&excerpt, (2, 2, 0), &Block::acacia_log(Axis3::X));
    assert_block_eq(&excerpt, (2, 0, 1), &Block::dark_oak_log(Axis3::Y));
    assert_block_eq(&excerpt, (2, 1, 1), &Block::dark_oak_log(Axis3::Z));
    assert_block_eq(&excerpt, (2, 2, 1), &Block::dark_oak_log(Axis3::X));

    check_stairs_multiple(&excerpt, (3, 0, 0), Material::Acacia);

    check_stairs_multiple(&excerpt, (4, 0, 0), Material::DarkOak);

    assert_block_eq(&excerpt, (5, 0, 0), &Block::BlockOfSlime);

    assert_block_eq(&excerpt, (6, 0, 0), &Block::Barrier);

    check_trapdoor_multiple(&excerpt, (7, 0, 0), Material::Iron);

    assert_block_eq(&excerpt, (8, 0, 0), &Block::Prismarine);
    assert_block_eq(&excerpt, (8, 0, 1), &Block::PrismarineBricks);
    assert_block_eq(&excerpt, (8, 0, 2), &Block::DarkPrismarine);

    assert_block_eq(&excerpt, (9, 0, 0), &Block::SeaLantern);

    assert_block_eq(&excerpt, (10, 0, 0), &Block::HayBale { alignment: Axis3::Y });
    assert_block_eq(&excerpt, (10, 1, 0), &Block::HayBale { alignment: Axis3::Z });
    assert_block_eq(&excerpt, (10, 2, 0), &Block::HayBale { alignment: Axis3::X });

    check_with_colour_multiple(&excerpt, (11, 0, 0), &Block::carpet_with_colour);

    assert_block_eq(&excerpt, (12, 0, 0), &Block::terracotta());

    assert_block_eq(&excerpt, (13, 0, 0), &Block::BlockOfCoal);

    assert_block_eq(&excerpt, (14, 0, 0), &Block::PackedIce);

    assert_block_eq(&excerpt, (15, 0, 0), &Block::Flower(Flower::SunflowerBottom));
    assert_block_eq(&excerpt, (15, 1, 0), &Block::Flower(Flower::SunflowerTop));
    assert_block_eq(&excerpt, (15, 0, 1), &Block::Flower(Flower::LilacBottom));
    assert_block_eq(&excerpt, (15, 1, 1), &Block::Flower(Flower::LilacTop));
    assert_block_eq(&excerpt, (15, 0, 2), &Block::Grass(Grass::TallGrassBottom));
    assert_block_eq(&excerpt, (15, 1, 2), &Block::Grass(Grass::TallGrassTop));
    assert_block_eq(&excerpt, (15, 0, 3), &Block::Grass(Grass::LargeFernBottom));
    assert_block_eq(&excerpt, (15, 1, 3), &Block::Grass(Grass::LargeFernTop));
    assert_block_eq(&excerpt, (15, 0, 4), &Block::Flower(Flower::RoseBushBottom));
    assert_block_eq(&excerpt, (15, 1, 4), &Block::Flower(Flower::RoseBushTop));
    assert_block_eq(&excerpt, (15, 0, 5), &Block::Flower(Flower::PeonyBottom));
    assert_block_eq(&excerpt, (15, 1, 5), &Block::Flower(Flower::PeonyTop));
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 176 through 191
fn v_1_12_2_block_group_12() {
    let excerpt = load_excerpt(INPUT_FILE, (176, 56, 0), (16, 4, 16));

    fn check_banner(we: &WorldExcerpt, at: (i64, i64, i64), colour: Colour, dir: Direction) {
        let block = we.block_at(at.into()).unwrap();
        let banner = Banner::try_from(block.clone()).unwrap();
        assert!(banner.has_colour_of(colour));
        assert!(banner.has_facing_of(dir));
    }

    check_banner(&excerpt, (0, 0, 0), Colour::White, Direction::South);
    check_banner(&excerpt, (0, 0, 1), Colour::Orange, Direction::SouthSouthWest);
    check_banner(&excerpt, (0, 0, 2), Colour::Magenta, Direction::SouthWest);
    check_banner(&excerpt, (0, 0, 3), Colour::LightBlue, Direction::WestSouthWest);
    check_banner(&excerpt, (0, 0, 4), Colour::Yellow, Direction::West);
    check_banner(&excerpt, (0, 0, 5), Colour::Lime, Direction::WestNorthWest);
    check_banner(&excerpt, (0, 0, 6), Colour::Pink, Direction::NorthWest);
    check_banner(&excerpt, (0, 0, 7), Colour::Gray, Direction::NorthNorthWest);
    check_banner(&excerpt, (0, 0, 8), Colour::LightGray, Direction::North);
    check_banner(&excerpt, (0, 0, 9), Colour::Cyan, Direction::NorthNorthEast);
    check_banner(&excerpt, (0, 0, 10), Colour::Purple, Direction::NorthEast);
    check_banner(&excerpt, (0, 0, 11), Colour::Blue, Direction::EastNorthEast);
    check_banner(&excerpt, (0, 0, 12), Colour::Brown, Direction::East);
    check_banner(&excerpt, (0, 0, 13), Colour::Green, Direction::EastSouthEast);
    check_banner(&excerpt, (0, 0, 14), Colour::Red, Direction::SouthEast);
    check_banner(&excerpt, (0, 0, 15), Colour::Black, Direction::SouthSouthEast);

    check_banner(&excerpt, (1, 0, 0), Colour::LightGray, Direction::West);
    check_banner(&excerpt, (1, 1, 0), Colour::LightGray, Direction::South);
    check_banner(&excerpt, (1, 2, 0), Colour::LightGray, Direction::East);
    check_banner(&excerpt, (1, 3, 0), Colour::LightGray, Direction::North);

    assert_block_eq(&excerpt, (2, 1, 0), &Block::InvertedDaylightDetector);

    assert_block_eq(&excerpt, (3, 0, 0), &Block::RedSandstone);
    assert_block_eq(&excerpt, (3, 0, 1), &Block::ChiseledRedSandstone);
    assert_block_eq(&excerpt, (3, 0, 2), &Block::SmoothRedSandstone);

    check_stairs_multiple(&excerpt, (4, 0, 0), Material::RedSandstone);

    assert_block_eq(&excerpt, (5, 0, 0), &Block::double_slab(Material::RedSandstone));

    assert_block_eq(&excerpt, (6, 0, 0), &Block::bottom_slab(Material::RedSandstone));
    assert_block_eq(&excerpt, (6, 1, 0), &Block::top_slab(Material::RedSandstone));

    check_fence_gate_multiple(&excerpt, (7, 0, 0), &Block::spruce_fence_gate);
    check_fence_gate_multiple(&excerpt, (7, 0, 4), &Block::spruce_fence_gate_opened);

    check_fence_gate_multiple(&excerpt, (8, 0, 0), &Block::birch_fence_gate);
    check_fence_gate_multiple(&excerpt, (8, 0, 4), &Block::birch_fence_gate_opened);

    check_fence_gate_multiple(&excerpt, (9, 0, 0), &Block::jungle_fence_gate);
    check_fence_gate_multiple(&excerpt, (9, 0, 4), &Block::jungle_fence_gate_opened);

    check_fence_gate_multiple(&excerpt, (10, 0, 0), &Block::dark_oak_fence_gate);
    check_fence_gate_multiple(&excerpt, (10, 0, 4), &Block::dark_oak_fence_gate_opened);

    check_fence_gate_multiple(&excerpt, (11, 0, 0), &Block::acacia_fence_gate);
    check_fence_gate_multiple(&excerpt, (11, 0, 4), &Block::acacia_fence_gate_opened);

    assert_block_eq(&excerpt, (12, 0, 0), &Block::spruce_fence());

    assert_block_eq(&excerpt, (13, 0, 0), &Block::birch_fence());

    assert_block_eq(&excerpt, (14, 0, 0), &Block::jungle_fence());

    assert_block_eq(&excerpt, (15, 0, 0), &Block::dark_oak_fence());
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 192 through 207
fn v_1_12_2_block_group_13() {
    let excerpt = load_excerpt(INPUT_FILE, (192, 56, 0), (16, 6, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::acacia_fence());

    check_door(&excerpt, (1, 0, 0), Direction::West, Material::Spruce, Hinge::Left, true);
    check_door(&excerpt, (1, 0, 1), Direction::West, Material::Spruce, Hinge::Right, true);

    check_door(&excerpt, (2, 0, 0), Direction::West, Material::Birch, Hinge::Left, true);
    check_door(&excerpt, (2, 0, 1), Direction::West, Material::Birch, Hinge::Right, true);

    check_door(&excerpt, (3, 0, 0), Direction::West, Material::Jungle, Hinge::Left, true);
    check_door(&excerpt, (3, 0, 1), Direction::West, Material::Jungle, Hinge::Right, true);

    check_door(&excerpt, (4, 0, 0), Direction::West, Material::Acacia, Hinge::Left, true);
    check_door(&excerpt, (4, 0, 1), Direction::West, Material::Acacia, Hinge::Right, true);

    check_door(&excerpt, (5, 0, 0), Direction::West, Material::DarkOak, Hinge::Left, true);
    check_door(&excerpt, (5, 0, 1), Direction::West, Material::DarkOak, Hinge::Right, true);

    assert_block_eq(&excerpt, (6, 0, 0), &Block::EndRod { facing: Surface6::Up });
    assert_block_eq(&excerpt, (6, 1, 0), &Block::EndRod { facing: Surface6::South });
    assert_block_eq(&excerpt, (6, 2, 0), &Block::EndRod { facing: Surface6::East });
    assert_block_eq(&excerpt, (6, 3, 0), &Block::EndRod { facing: Surface6::North });
    assert_block_eq(&excerpt, (6, 4, 0), &Block::EndRod { facing: Surface6::West });
    assert_block_eq(&excerpt, (6, 5, 0), &Block::EndRod { facing: Surface6::Down });

    // TODO chorus plant: Need to figure out how to parse connections...
    //fn check_chorus_plant(excerpt: &WorldExcerpt, at: (i64, i64, i64), direction: Direction) {
    //    let block = excerpt.block_at(at.into()).unwrap();
    //    let chorus_plant = ChorusPlant::try_from(block.clone()).unwrap();
    //    assert!(chorus_plant.is_touching_surface(direction));
    //}
    //check_chorus_plant(&excerpt, (7, 0, 0), Direction::Down);
    //check_chorus_plant(&excerpt, (7, 0, 0), Direction::East);

    assert_block_eq(&excerpt, (8, 0, 0),
        &Block::ChorusFlower { growth_stage: Int0Through5::new_saturating(0) });

    assert_block_eq(&excerpt, (9, 0, 0), &Block::PurpurBlock);

    // NB other alignments not present in savefile
    assert_block_eq(&excerpt, (10, 0, 0),
        &Block::PurpurPillar { alignment: Axis3::Y });

    check_stairs_multiple(&excerpt, (11, 0, 0), Material::Purpur);

    assert_block_eq(&excerpt, (12, 0, 0), &Block::double_slab(Material::Purpur));

    assert_block_eq(&excerpt, (13, 0, 0), &Block::bottom_slab(Material::Purpur));
    assert_block_eq(&excerpt, (13, 1, 0), &Block::top_slab(Material::Purpur));

    assert_block_eq(&excerpt, (14, 0, 0), &Block::EndStoneBricks);

    assert_block_eq(&excerpt, (15, 0, 0),
        &Block::Beetroots { growth_stage: Int0Through3::new_saturating(0) });
}

fn check_shulker_box_multiple(excerpt: &WorldExcerpt, at: (i64, i64, i64), colour: Colour) {
    let (x, y, z) = at;
    check_shulker_box(&excerpt, (x, y, z + 0), Direction::Up, colour);
    check_shulker_box(&excerpt, (x, y, z + 1), Direction::West, colour);
    check_shulker_box(&excerpt, (x, y, z + 2), Direction::South, colour);
    check_shulker_box(&excerpt, (x, y, z + 3), Direction::East, colour);
    check_shulker_box(&excerpt, (x, y, z + 4), Direction::North, colour);
    check_shulker_box(&excerpt, (x, y, z + 5), Direction::Down, colour);
}

fn check_shulker_box(
    excerpt: &WorldExcerpt,
    at: (i64, i64, i64),
    direction: Direction,
    colour: Colour,
) {
    let block = excerpt.block_at(at.into()).unwrap();
    let shulker_box = ShulkerBox::try_from(block.clone()).unwrap();
    assert!(shulker_box.has_facing_of(direction));
    assert!(shulker_box.has_colour_of(colour));
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 208 through 223
fn v_1_12_2_block_group_14() {
    // NB This block group uses an excerpt on a Y level one below the other excerpts!
    let excerpt = load_excerpt(INPUT_FILE, (208, 55, 0), (16, 4, 16));

    assert_block_eq(&excerpt, (0, 0, 0), &Block::GrassPath);

    // NB 209 "end gateway" is not in the savefile
    // NB 210 "repeating command block" is not in the savefile, and not implemented
    // NB 212 "chain command block" is not in the savefile, and not implemented

    assert_block_eq(&excerpt, (4, 1, 0), &Block::FrostedIce);

    assert_block_eq(&excerpt, (5, 1, 0), &Block::MagmaBlock);

    assert_block_eq(&excerpt, (6, 1, 0), &Block::NetherWartBlock);

    assert_block_eq(&excerpt, (7, 1, 0), &Block::RedNetherBricks);

    assert_block_eq(&excerpt, (8, 1, 0), &Block::BoneBlock { alignment: Axis3::Y });
    assert_block_eq(&excerpt, (8, 2, 0), &Block::BoneBlock { alignment: Axis3::Z });
    assert_block_eq(&excerpt, (8, 3, 0), &Block::BoneBlock { alignment: Axis3::X });

    // NB 218 "structure void" is not in the savefile

    let block = excerpt.block_at((10, 1, 0).into()).unwrap();
    assert!(block.is_observer() && block.has_facing_of(Direction::East));
    let block = excerpt.block_at((10, 1, 2).into()).unwrap();
    assert!(block.is_observer() && block.has_facing_of(Direction::North));
    let block = excerpt.block_at((10, 1, 4).into()).unwrap();
    assert!(block.is_observer() && block.has_facing_of(Direction::West));
    let block = excerpt.block_at((10, 1, 6).into()).unwrap();
    assert!(block.is_observer() && block.has_facing_of(Direction::South));
    let block = excerpt.block_at((10, 1, 8).into()).unwrap();
    assert!(block.is_observer() && block.has_facing_of(Direction::Down));
    let block = excerpt.block_at((10, 1, 10).into()).unwrap();
    assert!(block.is_observer() && block.has_facing_of(Direction::Up));

    check_shulker_box_multiple(&excerpt, (11, 1, 0), Colour::White);

    check_shulker_box_multiple(&excerpt, (12, 1, 0), Colour::Orange);

    check_shulker_box_multiple(&excerpt, (13, 1, 0), Colour::Magenta);

    check_shulker_box_multiple(&excerpt, (14, 1, 0), Colour::LightBlue);

    check_shulker_box_multiple(&excerpt, (15, 1, 0), Colour::Yellow);
}

fn check_glazed_terracotta_multiple(excerpt: &WorldExcerpt, at: (i64, i64, i64), colour: Colour) {
    let (x, y, z) = at;
    check_glazed_terracotta(&excerpt, (x, y, z + 0), Direction::West, colour);
    check_glazed_terracotta(&excerpt, (x, y, z + 1), Direction::South, colour);
    check_glazed_terracotta(&excerpt, (x, y, z + 2), Direction::East, colour);
    check_glazed_terracotta(&excerpt, (x, y, z + 3), Direction::North, colour);
}

fn check_glazed_terracotta(
    excerpt: &WorldExcerpt,
    at: (i64, i64, i64),
    direction: Direction,
    colour: Colour,
) {
    let block = excerpt.block_at(at.into()).unwrap();
    let glazed_terracotta = GlazedTerracotta::try_from(block.clone()).unwrap();
    assert!(glazed_terracotta.has_facing_of(direction));
    assert!(glazed_terracotta.has_colour_of(colour));
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 224 through 239
fn v_1_12_2_block_group_15() {
    let excerpt = load_excerpt(INPUT_FILE, (224, 56, 0), (16, 1, 16));

    check_shulker_box_multiple(&excerpt, (0, 0, 0), Colour::Lime);

    check_shulker_box_multiple(&excerpt, (1, 0, 0), Colour::Pink);

    check_shulker_box_multiple(&excerpt, (2, 0, 0), Colour::Gray);

    check_shulker_box_multiple(&excerpt, (3, 0, 0), Colour::LightGray);

    check_shulker_box_multiple(&excerpt, (4, 0, 0), Colour::Cyan);

    check_shulker_box_multiple(&excerpt, (5, 0, 0), Colour::Purple);

    check_shulker_box_multiple(&excerpt, (6, 0, 0), Colour::Blue);

    check_shulker_box_multiple(&excerpt, (7, 0, 0), Colour::Brown);

    check_shulker_box_multiple(&excerpt, (8, 0, 0), Colour::Green);

    check_shulker_box_multiple(&excerpt, (9, 0, 0), Colour::Red);

    check_shulker_box_multiple(&excerpt, (10, 0, 0), Colour::Black);

    check_glazed_terracotta_multiple(&excerpt, (11, 0, 0), Colour::White);

    check_glazed_terracotta_multiple(&excerpt, (12, 0, 0), Colour::Orange);

    check_glazed_terracotta_multiple(&excerpt, (13, 0, 0), Colour::Magenta);

    check_glazed_terracotta_multiple(&excerpt, (14, 0, 0), Colour::LightBlue);

    check_glazed_terracotta_multiple(&excerpt, (15, 0, 0), Colour::Yellow);
}

#[test]
#[rustfmt::skip]
/// Import of blocks with id 240 through 255
fn v_1_12_2_block_group_16() {
    let excerpt = load_excerpt(INPUT_FILE, (240, 56, 0), (16, 1, 16));

    check_glazed_terracotta_multiple(&excerpt, (0, 0, 0), Colour::Lime);

    check_glazed_terracotta_multiple(&excerpt, (1, 0, 0), Colour::Pink);

    check_glazed_terracotta_multiple(&excerpt, (2, 0, 0), Colour::Gray);

    check_glazed_terracotta_multiple(&excerpt, (3, 0, 0), Colour::LightGray);

    check_glazed_terracotta_multiple(&excerpt, (4, 0, 0), Colour::Cyan);

    check_glazed_terracotta_multiple(&excerpt, (5, 0, 0), Colour::Purple);

    check_glazed_terracotta_multiple(&excerpt, (6, 0, 0), Colour::Blue);

    check_glazed_terracotta_multiple(&excerpt, (7, 0, 0), Colour::Brown);

    check_glazed_terracotta_multiple(&excerpt, (8, 0, 0), Colour::Green);

    check_glazed_terracotta_multiple(&excerpt, (9, 0, 0), Colour::Red);

    check_glazed_terracotta_multiple(&excerpt, (10, 0, 0), Colour::Black);

    check_with_colour_multiple(&excerpt, (11, 0, 0), &Block::concrete_with_colour);

    check_with_colour_multiple(&excerpt, (12, 0, 0), &Block::concrete_powder_with_colour);

    // NB Block ID 253 is unused.
    // NB Block ID 254 is unused.
    // NB 255 "structure block" is not implemented, and not in save file.
}
