extern crate nbt;
extern crate serde_json;

use std::env;
use std::fs;
use std::process::exit;

use nbt::Blob;
use nbt::Result;

//use mcprogedit::coordinates;

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.into_iter().skip(1).take(1).next() {
        let mut file = fs::File::open(&arg)?;
        println!(
            "================================= NBT Contents ================================="
        );
        let blob = Blob::from_gzip_reader(&mut file)?;
        println!("{}", blob);
        println!(
            "============================== JSON Representation ============================="
        );
        match serde_json::to_string_pretty(&blob) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("error: {}", e);
                exit(1)
            }
        }
        Ok(())
    } else {
        eprintln!("error: a filename is required.");
        exit(1)
    }
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
        "PickleCount size is {}",
        std::mem::size_of::<mcprogedit::block::PickleCount>()
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
        "Age16 size is {}",
        std::mem::size_of::<mcprogedit::block::Age16>()
    );

    if let Err(err) = run() {
        eprintln!("error: {}", err);
        exit(1)
    };
}
