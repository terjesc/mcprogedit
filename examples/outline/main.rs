extern crate clap;
extern crate mcprogedit;

use std::path::Path;

use mcprogedit::block::*;
use mcprogedit::colour::Colour;
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
    let mut excerpt = WorldExcerpt::from_save(
        (x, y, z).into(),
        (x + x_len, y + y_len, z + z_len).into(),
        Path::new(input_directory),
    );

    // Modify the world excerpt:
    // Replace all solid blocks along the edge of the excerpt, with red concrete.
    fn replace_solid(excerpt: &mut WorldExcerpt, at: (i64, i64, i64), replacement: Block) {
        let (x, y, z) = at;
        if let Some(block) = excerpt.block_at((x, y, z).into()) {
            if block.is_solid() {
                excerpt.set_block_at((x, y, z).into(), replacement);
            }
        }
    }
    // Surfaces in the xy plane
    for x in 0..x_len {
        for y in 0..y_len {
            replace_solid(
                &mut excerpt,
                (x, y, 0),
                Block::concrete_with_colour(Colour::Red),
            );
            replace_solid(
                &mut excerpt,
                (x, y, z_len - 1),
                Block::concrete_with_colour(Colour::Red),
            );
        }
    }
    // Surfaces in the xz plane
    for x in 0..x_len {
        for z in 0..z_len {
            replace_solid(
                &mut excerpt,
                (x, 0, z),
                Block::concrete_with_colour(Colour::Red),
            );
            replace_solid(
                &mut excerpt,
                (x, y_len - 1, z),
                Block::concrete_with_colour(Colour::Red),
            );
        }
    }
    // Surfaces in the yz plane
    for y in 0..y_len {
        for z in 0..z_len {
            replace_solid(
                &mut excerpt,
                (0, y, z),
                Block::concrete_with_colour(Colour::Red),
            );
            replace_solid(
                &mut excerpt,
                (x_len - 1, y, z),
                Block::concrete_with_colour(Colour::Red),
            );
        }
    }

    // Export the modified world excerpt to the given save file directory
    excerpt.to_save((x, y, z).into(), Path::new(output_directory));
}

fn parse_i64_or_exit(string: &str) -> i64 {
    string.parse::<i64>().unwrap_or_else(|_| {
        eprintln!("Not an integer: {}", string);
        std::process::exit(1);
    })
}

fn matches() -> clap::ArgMatches<'static> {
    clap::App::new("outline - Mark the outer edge of a selection in a Minecraft save.")
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
