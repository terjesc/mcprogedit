# Implementing new blocks

Blocks are the basic building blocks of a Minecraft world. Some are cube shaped voxels, such as `Block::Dirt`, or `Block::Stone`. Others have various shapes, such as `Block::Cobweb`, or `Block::DeadBush`. Some have additional properties describing various states such as the block's orientation, variant or contents. Further, some of that state can be stored in convoluted ways by the game itself. This guide shows how to add to mcprogedit support for all kinds of new blocks; both trivial and more difficult blocks.

## Block storage format in mcprogedit

In mcprogedit, all blocks are variants of the `Block` enum found in _/src/block.rs_.

Many enum variants of `Block` correspond to individual Minecraft blocks as per Minecraft's block IDs. Some `Block` variants group similar or related Minecraft blocks into one single `Block` variant, and use data fields to distinguish between the various underlying Minecraft blocks. Examples of the former are `Block::Bedrock` and `Block::Dirt`. Examples of the latter are `Block::Glass { .. }` and `Block::Wall { .. }`, which in turn distinguish between subvariants through data fields `colour` and `material` respectively.

Yet some `Block` variants hold blocks with a lot of associated data. Those blocks are implemented as individual structures, with the `Block` variant simply `Box`ing the corresponding structure. `Block::Chest(Box<Chest>)` is such an example. Some `Block`s, such as `Block::Slab(Slab)`, even hold the full (non-`Box`ed) structure.

## Block storage format in Minecraft

* TODO: Before "the flattening". Minecraft block IDs. Block state. Block entities.
* TODO: Explain the `PaletteItem::Block()` schenanigans!

## Adding trivial blocks

For blocks without variants or block states, the `Block` enum variant doesn't need any data, and the rest of the implementation is pretty straight-forward.

`Block` variant implementation for the _bedrock_ block, in _/src/block.rs_:
```
pub enum Block {
    (...)
    Bedrock,
    (...)
```

Mapping _bedrock_ from `Block` to block ID, in _/src/chunk/palette.rs_:
```
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::Bedrock) => "minecraft:bedrock",
            (...)
```

Mapping _bedrock_ from block ID to `Block`, in _/src/chunk/palette.rs_:
```
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:bedrock" => block(Block::Bedrock),
            (...)
```

Trivial blocks don't have any block states or other data fields, so the main part of the implementation is complete at this point, having added only three lines of code. (\*TODO: Helper functions, lighting categories (transparence), etc., etc.)

## Adding blocks with different variants

Sometimes it makes sense to represent multiple similar or related blocks as one `Block` enum variant, and use variant data to distinguish the related blocks. A prominent example is the set of clear and stained _glass_ blocks. In addition to the uncoloured (clear) variant, there are 16 coloured (stained) variants corresponding to the 16 main colours.

`Block` variant implementation for all _glass block_ variants, in _/src/block.rs_:
```
pub enum Block {
    (...)
    Glass {
        colour: Option<Colour>,
    },
    (...)
```

For this implementation, the colour is represented with an `Option`, in order for `colour: None` to represent the clear glass block variant. The similar implementation for `Block::Concrete { colour: Colour }` doesn't have any uncoloured variant to map, and therefore uses `Colour` directly. Other block types, such as `Block::Wall { .. }`, use other fields for distinguishing between variants.

The conversion to and from block ID, while slightly more involved than for the trivial case, is still simply an act of matching between enum variants and strings.

Mapping _glass_ from `Block` to block IDs, in _/src/chunk/palette.rs_:
```
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::Glass { colour }) => match colour {
                None => "minecraft:glass",
                Some(Colour::White) => "minecraft:white_stained_glass",
                Some(Colour::Orange) => "minecraft:orange_stained_glass",
                Some(Colour::Magenta) => "minecraft:magenta_stained_glass",
                Some(Colour::LightBlue) => "minecraft:light_blue_stained_glass",
                Some(Colour::Yellow) => "minecraft:yellow_stained_glass",
                Some(Colour::Lime) => "minecraft:lime_stained_glass",
                Some(Colour::Pink) => "minecraft:pink_stained_glass",
                Some(Colour::Gray) => "minecraft:gray_stained_glass",
                Some(Colour::LightGray) => "minecraft:light_gray_stained_glass",
                Some(Colour::Cyan) => "minecraft:cyan_stained_glass",
                Some(Colour::Purple) => "minecraft:purple_stained_glass",
                Some(Colour::Blue) => "minecraft:blue_stained_glass",
                Some(Colour::Brown) => "minecraft:brown_stained_glass",
                Some(Colour::Green) => "minecraft:green_stained_glass",
                Some(Colour::Red) => "minecraft:red_stained_glass",
                Some(Colour::Black) => "minecraft:black_stained_glass",
            }
            (...)
```

Mapping _glass_ from block IDs to `Block`, in _/src/chunk/palette.rs_:
```
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:glass" => block(Block::Glass { colour: None }),
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
            (...)
```

If there are no block states or other data fields, then the main implementation is complete at this point, just as for implementing trivial blocks.

## Adding blocks with some (limited) _block state_

Some blocks come with additional state. Such state can be either fixed (e.g. rotational orientation), or the state can change during gameplay (e.g. hydration level of a block of farmland). For those blocks, in addition to ID conversion, it is also necessary to convert the _block state_ information. Block state information is held as data in the `Block` variant.

`Block` variant implementation for _farmland_, in _/src/block.rs_:
```
pub enum Block {
    (...)
    Farmland {
        wetness: Int0Through7,
    },
    (...)
```

The above definition of the `Farmland` variant should not be too surprising. _Farmland_ has a hydration level which can go from 0 to 7 inclusive, and `Int0Through7` is a data type which is bounded to that range. The below conversion to block ID is also straight-forward.

Mapping _farmland_ from `Block` to block ID, in _/src/chunk/palette.rs_:
```
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::Farmland { .. }) => "minecraft:farmland",
            (...)
```

The main difference from [Adding blocks with different variants], is that this time around the data of the `Block` variant comes from _block state_, and not from the block ID alone. Nor is the block ID alone sufficient for storing the block.

Storing block state involves the creation of NBT structures, which is the storage format used in Minecraft savefiles. For storing the block state, the `properties()` function of `PaletteItem` must return some `nbt::Value` containing that block state, on the match of a `PaletteItem` containing that `Block` variant. Below is the block state creation for _farmland_.

Storing the block state of _farmland_, in _/src/chunk/palette.rs_:
```
impl PaletteItem {
    (...)
    fn properties(&self) -> Option<nbt::Value> {
        match self {
            (...)
            PaletteItem::Block(Block::Farmland { wetness }) => {
                let mut properties = nbt::Map::new();
                properties.insert("moisture".into(), nbt::Value::String(wetness.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            (...)
```

Block states consist of an `nbt::Map` inside of an `nbt::Value::Compound`. The typical flow of generating the full block state NBT structure, is therefore to:

1. Create an `nbt::Map`, i.e. `let mut properties = nbt::Map::new();`
2. Fill it with the properties, e.g. `properties.insert("moisture".into(), nbt::Value::String(wetness.to_string()));`
3. Return the filled structure, packed in the correct NBT structure, i.e. `Some(nbt::Value::Compound(properties))`

In step 2. above, the _key_ of "moisture" comes from the official block state name for that block state, for the _farmland_ block. The _value_ of a block state is always an `nbt::Value::String`, in this case containing the number as a string using decimal notation.

Finally, for converting from block ID and block state, the NBT structure containing the block state is available at the needed location within `from_section()`, as a variable named `properties`. Block state values are fetched by lookup on the block state name, then converted to the corerct data type for inclusion in the returned `Block` variant instance.

Mapping _farmland_ from block ID to `Block`, in _/src/chunk/palette.rs_:
```
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:farmland" => block(Block::Farmland { wetness: moisture0_7(&properties) }),
            (...)
```

There is a whole mess of convenience functions for extracting values from the NBT structure. In the case of _farmland_, the wetness value, with the NBT tag "moisture", and possible value from 0 to 7 inclusive, is read using the convenience function `moisture0_7()`. This part of the library is in dire need of refactoring, but for the time being this is how the NBT values are extracted for `Block` variant creation.

For another example, below is the implementation of _End rod_.

`Block` variant implementation for the _End rod_ block, in _/src/block.rs_:
```
pub enum Block {
    (...)
    EndRod {
        facing: Surface6,
    },
    (...)
```

Mapping _End rod_ from `Block` to block ID, in _/src/chunk/palette.rs_:
```
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::EndRod { .. }) => "minecraft:end_rod",
            (...)
```

Storing the block state of _End rod_, in _/src/chunk/palette.rs_:
```
impl PaletteItem {
    (...)
    fn properties(&self) -> Option<nbt::Value> {
        match self {
            (...)
            PaletteItem::Block(Block::EndRod { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            (...)
```

Mapping _End rod_ from block ID to `Block`, in _/src/chunk/palette.rs_:
```
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:end_rod" => block(Block::EndRod { facing: facing_surface6(&properties)}),
            (...)
```

As for trivial blocks and blocks with different variants, this is the point where the main implementation of the block is complete.

## Adding blocks with many parameters

TODO: Blocks with struct. Example: Door

## Adding blocks with block entities

TODO: Blocks with struct and block entity. Example: Banner

## Adding remaining book-keeping, congregate functions, etc.

TODO: Lighting information (transparence, etc.) Direction, material, waterlogged, etc. congregate functions (polling for traits and values). Long term goal: Automate most of these. For now: Manual work.

## Some rambling about what is found in various files

### /src/block.rs

The `Block` enum contains all block types and variants, either directly or indirectly. Any new block implementation therefore starts by adding the new block to the `Block` enum, or through extending one of the existing variants.

For the implementation, some data types (defined elsewhere) are worth mentioning:

* Bounded ints (/src/bounded\_ints.rs): Data types with restricted ranges for integer values. Naming is on the form Int[min]Through[max], where [min] is the lower (inclusive) value, and [max] is the upper (inclusive) value that the integer data type may hold. E.g. an `Int0Through3` can hold the values 0, 1, 2 and 3, but no other integer values.
* Colour (/src/colour.rs): Minecraft has 16 default colours, represented by the `Colour` enum. Blocks which have these colour variants should have a field of type `Colour`, or alternatively `Option<Colour>` if there is also an uncoloured version of the block.
* Materials (/src/material.rs): Some materials are common for a large array of blocks. Typically each type of block, such as e.g. "Slab", "Stair", etc., has 

### /src/chunk/palette.rs




## Checklist

- [ ] /src/block.rs: Add new block to `Block` enum, or extend on one of the existing variants
- [ ] /src/bounded\_ints.rs, /src/colour.rs, /src/material.rs, etc.: Update if appropriate


- [ ] /src/chunk/palette.rs: ProtoBlock
- [ ] /src/chunk/palette.rs: ProtoBlock
- [ ] /src/chunk/palette.rs: ProtoBlock
