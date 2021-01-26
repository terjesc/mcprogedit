#[macro_use]
extern crate bounded_integer;
#[macro_use]
extern crate static_assertions;

pub mod arguments;
pub mod biome;
pub mod block;
mod block_cuboid;
mod block_entity;
pub mod bounded_ints;
pub mod bounds;
mod chunk;
pub mod colour;
pub mod coordinates;
pub mod enchantment;
pub mod height_map;
pub mod inventory;
pub mod item;
pub mod material;
pub mod mc_version;
mod nbt_lookup;
pub mod positioning;
pub mod region;
pub mod status_effect;
mod utils;
pub mod world_excerpt;
