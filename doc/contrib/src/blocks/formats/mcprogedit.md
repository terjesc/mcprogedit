# Block storage format in mcprogedit

In mcprogedit, all blocks are variants of the `Block` enum found in _/src/block.rs_.

Many enum variants of `Block` correspond to individual Minecraft blocks as per Minecraft's block IDs. Some `Block` variants group similar or related Minecraft blocks into one single `Block` variant, and use data fields to distinguish between the various underlying Minecraft blocks. Examples of the former are `Block::Bedrock` and `Block::Dirt`. Examples of the latter are `Block::Glass { .. }` and `Block::Wall { .. }`, which in turn distinguish between subvariants through data fields `colour` and `material` respectively.

Yet some `Block` variants hold blocks with a lot of associated data. Those blocks are implemented as individual structures, with the `Block` variant simply `Box`ing the corresponding structure. `Block::Chest(Box<Chest>)` is such an example. Some blocks, such as `Block::Slab(Slab)`, even hold the full (non-`Box`ed) structure.

Below are excerpts from the `Block` enum definition, to illustrate the range of complexity for different variants:

```rust
pub enum Block {
    (...)
    Bedrock,
    (...)
    Glass {
        colour: Option<Colour>,
    },
    (...)
    GlassPane {
        colour: Option<Colour>,
        waterlogged: bool,
    },
    (...)
    Grass(Grass),
    (...)
    Hopper(Box<Hopper>),
    (...)
    Slab(Slab),
    (...)
}
```

In the above excerpt, `Bedrock` is a trivial block with no variants and no block state. `Glass` is a common enum variant for multiple _glass_ block variants. `GlassPane` comes with both block variants (`colour`) and block state (`waterlogged`). `Grass` directly holds an enum of the same name for holding the variants. `Hopper` holds a `Box`ed struct of the same name, which holds larger amounts of data, including entity data. That struct is even implemented in a separate file (_/block/hopper.rs_). `Slab` also holds a struct, but due to its small size that struct is held directly instead of in a `Box`.

TODO: Mention the convenience functions and/or other things related to blocks that might need updating when blocks are added.

