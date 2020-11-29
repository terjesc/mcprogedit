extern crate nbt;
extern crate serde_json;

use std::fs;
use std::process::exit;

use nbt::Blob;
use nbt::Result;

use mcprogedit::arguments;
use mcprogedit::mc_version::McVersion;
use mcprogedit::world_excerpt::WorldExcerpt;
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
    println!("================================================================================");

    if let nbt::Value::Compound(data) = blob.get("Data").expect("Could not read data.") {
        let level_name = data.get("LevelName").expect("Could not read level name.");
        println!("Reading level \"{}\"", level_name);
        let nbt_version = data.get("version").expect("Could not read NBT version.");
        let data_version = data
            .get("DataVersion")
            .expect("Could not read data version.");
        println!("NBT version {}, data version {}", nbt_version, data_version);

        if let nbt::Value::Compound(version_c) = data
            .get("Version")
            .expect("Could not read version compound.")
        {
            let id = version_c.get("Id").expect("Could not read id.");
            let is_snapshot = match version_c.get("Snapshot").expect("Could not read snapshot.") {
                nbt::Value::Byte(0) => false,
                _ => true,
            };
            let version_string = version_c
                .get("Name")
                .expect("Could not read version string.");
            let snapshot_string = match is_snapshot {
                true => "(is a snapshot)",
                false => "(not a snapshot)",
            };
            println!(
                "Minecraft {} {}, data version {}",
                version_string, snapshot_string, id
            );

            let data_version = match data_version {
                nbt::Value::Int(id) => id,
                _ => panic!("Not an Int nbt tag: {}", data_version),
            };
            let data_version = McVersion::from_id(*data_version);

            let version_string = match version_string {
                nbt::Value::String(name) => name,
                _ => panic!("Not a String nbt tag: {}", version_string),
            };

            let id = match id {
                nbt::Value::Int(id) => id,
                _ => panic!("Not an Int nbt tag: {}", id),
            };
            let id = McVersion::from_id(*id);

            // Is the version info internally consistent?
            assert_eq!(data_version, id);
            assert_eq!(id.name(), version_string);
        }
    }

    let excerpt = WorldExcerpt::from_save((-4, 50, 0).into(), (18, 80, -25).into(), save_directory);
    /*
    println!("============================== JSON Representation =============================");
    match serde_json::to_string_pretty(&blob) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1)
        }
    }
    */

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
