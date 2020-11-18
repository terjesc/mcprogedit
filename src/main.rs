extern crate nbt;
extern crate serde_json;

use std::env;
use std::fs;
use std::process::exit;

use nbt::Blob;
use nbt::Result;

use mcprogedit::arguments;
//use mcprogedit::coordinates;

fn run() -> Result<()> {
    let matches = arguments::matches();

    let save_directory = matches.value_of("input_save").unwrap();
    let save_directory = std::path::Path::new(save_directory);
    let level_dat_path = save_directory.join("level.dat");

    let mut level_dat = fs::File::open(level_dat_path)?;
    println!("================================= NBT Contents =================================");
    let blob = Blob::from_gzip_reader(&mut level_dat)?;
    println!("{}", blob);
    println!("============================== JSON Representation =============================");
    match serde_json::to_string_pretty(&blob) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1)
        }
    }
    Ok(())
}

fn main() {
    println!(
        "Block enum size is {}",
        std::mem::size_of::<mcprogedit::block::Block>()
    );

    println!(
        "Slab size is {}",
        std::mem::size_of::<mcprogedit::block::Slab>()
    );
    println!(
        "Log size is {}",
        std::mem::size_of::<mcprogedit::block::Log>()
    );
    println!(
        "ChorusPlantConnections size is {}",
        std::mem::size_of::<mcprogedit::block::ChorusPlantConnections>()
    );
    println!(
        "FireFace size is {}",
        std::mem::size_of::<mcprogedit::block::FireFace>()
    );
    println!(
        "Int1Through4 size is {}",
        std::mem::size_of::<mcprogedit::bounded_ints::Int1Through4>()
    );
    println!(
        "Seagrass size is {}",
        std::mem::size_of::<mcprogedit::block::Seagrass>()
    );
    println!(
        "Sign size is {}",
        std::mem::size_of::<mcprogedit::block::Sign>()
    );
    println!(
        "Box<Sign> size is {}",
        std::mem::size_of::<Box<mcprogedit::block::Sign>>()
    );
    println!(
        "WallFloorFacing size is {}",
        std::mem::size_of::<mcprogedit::positioning::WallOrRotatedOnFloor>()
    );
    println!(
        "Int0Through15 size is {}",
        std::mem::size_of::<mcprogedit::bounded_ints::Int0Through15>()
    );

    if let Err(err) = run() {
        eprintln!("error: {}", err);
        exit(1)
    };
}
