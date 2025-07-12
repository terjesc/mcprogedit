//! This example is a port of Michael Green's "Cellular Automata Settlement Generator Example" [1],
//! which showcases the MCEdit filter submission option to the Generative Design in Minecraft
//! Competition [2].
//!
//! [1] https://github.com/mcgreentn/GDMC/blob/master/stock-filters/CASG_Example.py
//! [2] http://gendesignmc.engineering.nyu.edu/

extern crate clap;
extern crate mcprogedit;
extern crate rand;

use std::collections::VecDeque;
use std::path::Path;

use crate::rand::Rng;
use mcprogedit::block::*;
use mcprogedit::world_excerpt::WorldExcerpt;

fn main() {
    // Read arguments
    let matches = matches();
    let input_directory = matches.value_of("input_save").unwrap_or(".");
    let output_directory = matches.value_of("output_save").unwrap_or(input_directory);
    let x = matches.value_of("x").map(parse_i64_or_exit).unwrap();
    let y = matches.value_of("y").map(parse_i64_or_exit).unwrap_or(0);
    let z = matches.value_of("z").map(parse_i64_or_exit).unwrap();
    let x_len = matches.value_of("dx").map(parse_i64_or_exit).unwrap();
    let y_len = matches
        .value_of("dy")
        .map(parse_i64_or_exit)
        .unwrap_or(255 - y);
    let z_len = matches.value_of("dz").map(parse_i64_or_exit).unwrap();

    // Import the given area from the given save file directory
    println!("Importing from {:?}", input_directory);
    let mut excerpt = WorldExcerpt::from_save(
        (x, y, z).into(),
        (x + x_len - 1, y + y_len - 1, z + z_len - 1).into(),
        Path::new(input_directory),
    );
    println!("Imported world excerpt of dimensions {:?}", excerpt.dim());

    // Modify the world excerpt
    let bounding_box = BoundingBox {
        position: (0, 0, 0),
        size: excerpt.dim(),
    };
    let yards = bounding_box.into_binary_partition();
    for yard in yards {
        build_fence(&mut excerpt, yard);
        build_structure(&mut excerpt, yard);
    }

    // Export the modified world excerpt to the given save file directory
    println!("Exporting to {:?}", output_directory);
    excerpt.to_save((x, y, z).into(), Path::new(output_directory));
}

#[derive(Clone, Copy, Debug)]
struct BoundingBox {
    position: (i32, i32, i32),
    size: (usize, usize, usize),
}

impl BoundingBox {
    /// Splits the BoundingBox into a vector of smaller BoudningBoxes
    fn into_binary_partition(self) -> Vec<Self> {
        // We need a random generator for some of the steps in this function.
        let mut rng = rand::thread_rng();

        let mut partition = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while !queue.is_empty() {
            let split_me = queue.pop_front().unwrap();
            let (width, height, depth) = split_me.size;
            println!("Current partition {},{}", width, depth);
            // This bool lets me know which dimension I will be splitting on.
            // It matters when we create the new outer bound size.
            let mut is_width = false;
            // Find the larger dimension and divide in half.
            // If the larger dimension is < 10, then block this from being partitioned.
            let min_size = 12;

            let centre;
            if width > depth {
                // Stop if too small, or at random 1 % chance
                if depth < min_size || rng.gen_range(0..100) == 0 {
                    partition.push(split_me);
                    continue;
                }
                is_width = true;
                centre = width / 2;
            } else {
                if width < min_size || rng.gen_range(0..100) == 0 {
                    partition.push(split_me);
                    continue;
                }
                centre = depth / 2;
            }

            // A random modifier for binary splitting which is somewhere between 0 and 1/8 the total
            // box side length.
            let random_partition = rng.gen_range(0..(centre / 8) + 1);
            // Creating the new bound.
            let new_bound = centre + random_partition;
            // Creating the outer edge bounds.
            let outside_new_bounds = match is_width {
                true => width - new_bound - 1,
                false => depth - new_bound - 1,
            };

            // Creating the bounding boxes
            if is_width {
                queue.push_back(BoundingBox {
                    position: (
                        split_me.position.0,
                        split_me.position.1,
                        split_me.position.2,
                    ),
                    size: (new_bound - 1, height, depth),
                });
                queue.push_back(BoundingBox {
                    position: (
                        split_me.position.0 + new_bound as i32 + 1,
                        split_me.position.1,
                        split_me.position.2,
                    ),
                    size: (outside_new_bounds - 1, height, depth),
                });
            } else {
                queue.push_back(BoundingBox {
                    position: (
                        split_me.position.0,
                        split_me.position.1,
                        split_me.position.2,
                    ),
                    size: (width, height, new_bound - 1),
                });
                queue.push_back(BoundingBox {
                    position: (
                        split_me.position.0,
                        split_me.position.1,
                        split_me.position.2 + new_bound as i32 + 1,
                    ),
                    size: (width, height, outside_new_bounds - 1),
                });
            }
        }
        partition
    }
}

/// Builds a wooden fence around the perimeter of this yard, like this photo
///             Top - zmax
///       ----------------
///       |              |
///       |              |
///       |              |
/// Left  |              | Right
/// xmin  |              | xmax
///       |              |
///       |              |
///       ----------------
///           Bottom - zmin
fn build_fence(excerpt: &mut WorldExcerpt, yard: BoundingBox) {
    let minx = yard.position.0 as i64;
    let miny = yard.position.1 as i64;
    let minz = yard.position.2 as i64;
    let maxx = minx + yard.size.0 as i64;
    let maxy = miny + yard.size.1 as i64;
    let maxz = minz + yard.size.2 as i64;

    // Add top fence blocks
    for x in minx..maxx {
        for y in (miny..maxy).rev() {
            if let Some(block) = excerpt.block_at((x, y, maxz).into()) {
                if block.is_solid() {
                    excerpt.set_block_at((x, y + 1, maxz).into(), Block::oak_fence());
                    break;
                }
            }
        }
    }

    // Add bottom fence blocks
    for x in minx..maxx {
        for y in (miny..maxy).rev() {
            if let Some(block) = excerpt.block_at((x, y, minz).into()) {
                if block.is_solid() {
                    excerpt.set_block_at((x, y + 1, minz).into(), Block::oak_fence());
                    break;
                }
            }
        }
    }

    // Add left fence blocks
    for z in minz..maxz {
        for y in (miny..maxy).rev() {
            if let Some(block) = excerpt.block_at((minx, y, z).into()) {
                if block.is_solid() {
                    excerpt.set_block_at((minx, y + 1, z).into(), Block::oak_fence());
                    break;
                }
            }
        }
    }

    // Add right fence blocks
    for z in minz..maxz {
        for y in (miny..maxy).rev() {
            if let Some(block) = excerpt.block_at((maxx, y, z).into()) {
                if block.is_solid() {
                    excerpt.set_block_at((maxx, y + 1, z).into(), Block::oak_fence());
                    break;
                }
            }
        }
    }
}

/// Builds a structure (the material of which is specified by user in inputs) within the given box
/// 4 steps:
/// 1. decide the floor plan, map out the foundations of the building, build floor
/// 2. create corner pillars
/// 3. between each pair of pillars, use Cellular Automata to build a wall
/// 4. create celing at the ceiling level
fn build_structure(mut excerpt: &mut WorldExcerpt, yard: BoundingBox) {
    let floor = make_floor_plan(yard);

    let (y_average, y_start_coord_max, midpoint_floor_height) = create_pillars(&mut excerpt, floor);

    //generate_walls(&mut excerpt, floor);

    let ceiling_y = y_average + midpoint_floor_height;
    generate_ceiling(&mut excerpt, floor, ceiling_y);
}

fn make_floor_plan(yard: BoundingBox) -> BoundingBox {
    // We have to first figure out where in the box this is going to be.
    // Find the box dimensions
    let (width, height, depth) = yard.size;

    // Get sixths
    let fraction_width = width / 6;
    let fraction_depth = depth / 6;

    // Create the box boundaries
    let mut rng = rand::thread_rng();
    let rand_frac_x = rng.gen_range(0..(fraction_width + 1));
    let rand_frac_z = rng.gen_range(0..(fraction_depth + 1));
    let x_start = yard.position.0 + rand_frac_x as i32 + 2;
    let z_start = yard.position.2 + rand_frac_z as i32 + 2;
    let x_size = (width as f32 * 0.6) as usize - rand_frac_x;
    let z_size = (depth as f32 * 0.6) as usize - rand_frac_z;

    BoundingBox {
        position: (x_start, yard.position.1, z_start),
        size: (x_size, height, z_size),
    }
}

/// Create corners for hte walls.
/// Every building needs corners for stability...unless you are inventive... :)
fn create_pillars(excerpt: &mut WorldExcerpt, floor: BoundingBox) -> (i64, i64, i64) {
    let mut corner_block_starts = Vec::new();
    let mut y_coords = Vec::new();

    let minx = floor.position.0 as i64;
    let miny = floor.position.1 as i64;
    let minz = floor.position.2 as i64;
    let maxx = minx + floor.size.0 as i64;
    let maxy = miny + floor.size.1 as i64;
    let maxz = minz + floor.size.2 as i64;

    // Similarly to fences, we need to countdown on each of the four corners and find the block
    // where the ground starts, then start building pillars above that height.
    let mut midpoint_floor_height = 0;
    for y in (miny..maxy).rev() {
        if excerpt.block_at((minx, y, minz).into()) != Some(&Block::Air) {
            corner_block_starts.push((minx, y + 1, minz));
            break;
        }
    }
    for y in (miny..maxy).rev() {
        if excerpt.block_at((minx, y, maxz).into()) != Some(&Block::Air) {
            corner_block_starts.push((minx, y + 1, maxz));
            break;
        }
    }
    for y in (miny..maxy).rev() {
        if excerpt.block_at((maxx, y, minz).into()) != Some(&Block::Air) {
            corner_block_starts.push((maxx, y + 1, minz));
            break;
        }
    }
    for y in (miny..maxy).rev() {
        if excerpt.block_at((maxx, y, maxz).into()) != Some(&Block::Air) {
            corner_block_starts.push((maxx, y + 1, maxz));
            break;
        }
    }

    let mut y_start_coord_max = 0;
    for corner_stone in corner_block_starts {
        midpoint_floor_height += corner_stone.1;
        if corner_stone.1 > y_start_coord_max {
            y_start_coord_max = corner_stone.1;
        }
        let mut rng = rand::thread_rng();
        let pillar_height = rng.gen_range(5..45);
        for y in 0..pillar_height {
            excerpt.set_block_at(
                (corner_stone.0, corner_stone.1 + y, corner_stone.2).into(),
                Block::Cobblestone,
            );
        }
        y_coords.push(pillar_height - 1);
    }
    let mut all_ys = 0;
    for y_coord in y_coords {
        all_ys += y_coord;
    }
    let y_average = all_ys / 4;
    midpoint_floor_height /= 4;
    println!("Average pillar height: {}", y_average);

    (y_average, y_start_coord_max, midpoint_floor_height)
}

/// The walls of the building are generated each using independent ceullular automata.
/// We look at the immediate neighborhood and take action.
/// Cellular automata is done in 3 easy steps:
/// 1. Intitialize with random block placement in the space.
/// 2. Evaluate each cell, checking its neighbors to gauge changes.
/// 3. Repeat 2 until satisfied.
fn generate_walls(excerpt: &mut WorldExcerpt, floor: BoundingBox) {
    unimplemented!();
}

/// Puts a cap on the building in question.
/// Uses the floor to determine the ceiling size, and the buildingHeightInfo tuple
/// to place it at the right level.
fn generate_ceiling(excerpt: &mut WorldExcerpt, floor: BoundingBox, y: i64) {
    println!("Generating ceiling");
    let minx = floor.position.0 as i64;
    let maxx = minx + floor.size.0 as i64;
    let minz = floor.position.2 as i64;
    let maxz = minz + floor.size.2 as i64;

    for x in minx..=maxx {
        for z in minz..=maxz {
            excerpt.set_block_at((x, y, z).into(), Block::Cobblestone);
        }
    }
}

fn parse_i64_or_exit(string: &str) -> i64 {
    string.parse::<i64>().unwrap_or_else(|_| {
        eprintln!("Not an integer: {}", string);
        std::process::exit(1);
    })
}

fn matches() -> clap::ArgMatches<'static> {
    clap::App::new("casg - Cellular Automata Settlement Generator.")
        .set_term_width(80)
        .version(clap::crate_version!())
        .arg(
            clap::Arg::with_name("input_save")
                .short("-i")
                .long("input-directory")
                .value_name("DIRECTORY")
                .help("Input save directory. Set to working directory if not provided.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("output_save")
                .short("-o")
                .long("output-directory")
                .value_name("DIRECTORY")
                .help("Output save directory. Set to input directory if not provided.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("x")
                .short("-x")
                .long("x-coordinate")
                .value_name("block x")
                .help("Selection corner x coordinate.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("dx")
                .short("-X")
                .long("x-size")
                .value_name("block count")
                .help("Selection size along the x axis.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("y")
                .short("-y")
                .long("y-coordinate")
                .value_name("block y")
                .help("Selection corner y coordinate.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            clap::Arg::with_name("dy")
                .short("-Y")
                .long("y-size")
                .value_name("block count")
                .help("Selection size along the y axis.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            clap::Arg::with_name("z")
                .short("-z")
                .long("z-coordinate")
                .value_name("block z")
                .help("Selection corner z coordinate.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("dz")
                .short("-Z")
                .long("z-size")
                .value_name("block count")
                .help("Selection size along the z axis.")
                .takes_value(true)
                .required(true),
        )
        .get_matches()
}
