# Implementing new blocks

Blocks are the basic building blocks of a Minecraft world. Some are cube shaped voxels, such as _dirt_ and _stone_. Others have various shapes, such as _cobweb_ and _dead bush_. Some have additional properties describing various states such as the block's orientation, variant or contents (e.g. _torch_, _lever_). Further, some of that state can be stored in convoluted ways by the game itself (e.g. _chest_, _furnace_). This guide aims to show how to add support to mcprogedit for all kinds of new blocks, regardless of complexity.

## The Minecraft block storage format

### Block ID, block states, and block data

All Minecraft blocks are identified through a textual _block ID_, such as `"minecraft:Air"` or `"minecraft:Dirt"`.

Many blocks also come with _block states_, describing properties of a particular instance of the block in the Minecraft world. Such block states can describe e.g. the rotational orientation of a chest, the power level of a redstone wire, the on/off status of a lever, or the growth stage of a beetroot. Each block state has a _name_ used for lookup, a set of _allowed values_, and a _default value_. As an example, _wheat crops_ has a block state whose name is _age_, holding a value in the range 0 through 7, with a default value of 0. This state encodes the growth stage of the wheat crops, from newly sown (0) to ripe (7).

A small number of blocks have additional data in the form of a _block entity_. A typical example is a _chest_, which can hold a large inventory of items. The block entity is used for storing complex data, and is formatted using an _NBT structure_. In this structurre, some nodes are present for all block entities, while others are specific to the block entity of one particular block ID. Basic information is still stored as block states, even for blocks associated with a block entity.

Formats and values for block ID, block states and block entity data are documented under the _Data values_ section of the Minecraft wiki entry for each block. Block ID is listed as _ID_ or _identifier_. For blocks having block states, those are listed under _Block states_, and for blocks with a block entity the full NBT structure is documented under _Block data_.

### The blocks within a Minecraft world

In the current save format, blocks are stored within _chunks_, which are smaller sections of the Minecraft world. Each chunk holds 16 x 16 blocks in the horizontal plane, and extend to the full height and depth of the world. The chunk is further subdivided into _sections_, each 16 blocks tall, leaving a cube of 16 x 16 x 16 blocks, 4096 blocks in total, to each section.

Among other data, each section holds an array of 4096 blocks, with each position in that array mapping to a location within the 16^3 cube. Instead of storing the block's ID and states directly, the block array stores _indexes_ into a different array; the _palette_. The palette in turn holds _each distinct block ID + block states combination_ from within the section. Any block entities come in addition to the palette, with each block entity mapping to a single block within the section.

The use of palettes saves storage space, since sections typically hold a lot of repeating blocks. It does however come at the expense of added complexity. Nevertheless, in practical terms this has implications only for _where_ the different parts of a block implementation go. Luckliy the main _complexity_ of chunks, sections and palettes can be abstracted away in the respective modules. To a large extent the same goes for block entities.

Please note that the Minecraft storage format _has_ changed in the past, and it _might_ do so again in the future. For instance, prior to Minecraft 1.13, block ID was an 8 bit integer, further bits were used for block states, and there was no palette. The switch from numeric IDs was so impactful that the change got its own name: [_The Flattening_](https://minecraft.wiki/w/Java_Edition_Flattening).

## Block storage format in mcprogedit

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

## Adding trivial blocks

For blocks without variants or block states, the `Block` enum variant doesn't need any data, and the rest of the implementation is pretty straight-forward.

First, we must add the `Bedrock` variant to the `Block` definition, in _/src/block.rs_:

```rust
pub enum Block {
    (...)
    Bedrock,
    (...)
```

Second, we must add the mapping from `Block` variant to block ID, in _/src/chunk/palette.rs_. Note that for this mapping we are working on a `PaletteItem`, which is simply an enum with a variant named `Block`, which in turn holds the `Block` enum. The mapping is added to the `match self` in the function `name()` of the `PaletteItem` implementation, and it maps from `PaletteItem::Block(Block::Bedrock)` to the textual block ID string:

```rust
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::Bedrock) => "minecraft:bedrock",
            (...)
```

Third, we must add the mapping back from the block ID to the `Block` variant, also in _/src/chunk/palette.rs_. For this, the convenience function `block()` conveniently creates a `PaletteItem` from a `Block`, leaving us with a short and easy addition inside the `let palette_item = match name.as_str()` inside of the `from_section` function:

```rust
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:bedrock" => block(Block::Bedrock),
            (...)
```

Trivial blocks don't have any block states or other data fields, so the main part of the implementation is complete at this point, having added only three lines of code.

TODO: Helper functions, lighting categories (transparence), etc.

## Adding blocks with different variants

Sometimes it makes sense to represent multiple similar or related blocks as one `Block` enum variant, and use variant data to distinguish the related blocks. A prominent example is the set of clear and stained _glass_ blocks. In addition to the uncoloured (clear) variant, there are 16 coloured (stained) variants corresponding to the 16 main colours.

`Block` variant implementation for all _glass block_ variants, in _/src/block.rs_:
```rust
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
```rust
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
```rust
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
```rust
pub enum Block {
    (...)
    Farmland {
        wetness: Int0Through7,
    },
    (...)
```

The above definition of the `Farmland` variant shouldn't be too surprising. _Farmland_ has a hydration level which can go from 0 to 7 inclusive, and `Int0Through7` is a data type which is bounded to that range. The below conversion to block ID is also straight-forward.

Mapping _farmland_ from `Block` to block ID, in _/src/chunk/palette.rs_:
```rust
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::Farmland { .. }) => "minecraft:farmland",
            (...)
```

The main difference from [TODO: Link to "Adding blocks with different variants"], is that this time some of the data of the `Block` variant comes from _block state_, and not from the block ID alone. Nor is the block ID alone sufficient for the opposite conversion.

Storing block state involves the creation of an NBT structure. Specifically, the `properties()` function of `PaletteItem` must return some `nbt::Value` containing the block's block state, on the match of a `PaletteItem` containing the given `Block` variant. Below is the block state creation for _farmland_.

Composing the NBT block state of _farmland_, in _/src/chunk/palette.rs_:
```rust
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

As you can see from the code excerpt above, block states are to be put in an `nbt::Map` inside of an `nbt::Value::Compound`. The typical flow of generating the block state NBT structure, is therefore to:

1. Grab the required fields from the `Block`, e.g.:
    ```rust
    PaletteItem::Block(Block::Farmland { wetness }) => {
    ```
2. Create an `nbt::Map`, i.e.:
    ```rust
        let mut properties = nbt::Map::new();
    ```
3. Fill the map with the required block states, e.g.:
    ```rust
        properties.insert("moisture".into(), nbt::Value::String(wetness.to_string()));
    ```
4. Return the filled structure, packed in the correct NBT structure, i.e.:
    ```rust
        Some(nbt::Value::Compound(properties))
    ```

In step 3. above, the _key_ of "moisture" comes from the official block state name for that block state, for the _farmland_ block. The _value_ of a block state is always an `nbt::Value::String`. In this case the string contains the number as a string using decimal notation.

Finally, the conversion from block ID and NBT block state, to the `Block` variant. The NBT structure containing the block state is available at the needed location, within the `from_section()` function, as a variable named `properties`. From within that structure, block state _values_ are fetched by lookup on the _name_, then converted to the corerct data type for inclusion in the returned `Block` variant instance.

Mapping _farmland_ from block ID to `Block`, in _/src/chunk/palette.rs_:
```rust
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:farmland" => block(Block::Farmland { wetness: prop(&properties, "moisture") }),
            (...)
```

For a more complex block, we could have made a dedicated function for creating the block from the `properties` variable. For `Block::Farmland` there is only one field, `wetness`, and so the full implementation fits in one line. We fetch the value of the `moisture` block state from `properties` with the generic utility function `prop()`. When given the `properties` NBT structure and the name of the block state, it returns the value with the required type -- in this case an `Int0Through7`.

There are some instances where the `prop()` function doesn't work. In those instances there might be more manual work for extracting the block state and converting it to the correct type. Luckily for us, _farmland_ is not one of those instances.

For another example, here is the implementation of _End rod_.

`Block` variant implementation for the _End rod_ block, in _/src/block.rs_:
```rust
pub enum Block {
    (...)
    EndRod {
        facing: Surface6,
    },
    (...)
```

Mapping _End rod_ from `Block` to block ID, in _/src/chunk/palette.rs_:
```rust
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::EndRod { .. }) => "minecraft:end_rod",
            (...)
```

Generating the NBT block state of _End rod_, in _/src/chunk/palette.rs_:
```rust
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
```rust
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:end_rod" => block(Block::EndRod { facing: facing_surface6(&properties)}),
            (...)
```

There are currently a whole mess of convenience functions for extracting values from the NBT structure. In the case of _end rod_, the `facing` value, with the block state name "facing", needs conversion to the type `Surface6`, and for that there is a dedicated function `facing_surface6()` for extracting the value from the NBT structure. If `Surface6` gets implementations of the required traits, then this can also be changed to use `prop()`, as was the case for _farmland_. To be specific, `prop()` works for data types with both the `FromStr`, `Default` and `Display` traits.

Just as for trivial blocks and blocks with different variants, this is the point where the main implementation of the block is complete.

## Adding blocks with many parameters

TODO: Block with more parameters. Example: Coral? CoralFan?

TODO: Blocks with struct. Example: Door?

## Adding blocks with block entity data

Blocks with block entity data are typically too large for holding the state directly as a `Block` enum variant. The variant definition in _/src/block.rs_ therefore uses a `Box`:

```rust
pub enum Block {
    (...)
    Banner(Box<Banner>),
    (...)
```

Also, due to the complexity of the block, the structure definition and implementation is typically found in a file dedicated for the particular block. In the case of _banner_, in _/src/block/banner.rs_:

```rust
/// Banner "block".
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Banner {
    /// Base colour of the banner.
    pub colour: Colour,
    /// If used: The name is used as a marker on maps.
    pub custom_name: Option<String>,
    pub placement: WallOrRotatedOnFloor,
    /// List of (normally) up to 6 coloured patterns,
    /// that are featured on top of each other on the banner.
    pub patterns: Vec<ColouredPattern>,
}
```

The _banner_ is somewhat complicated:

* There are several _block IDs_; one for each of the 16 base colours. Here they are grouped into one struct for all banner types, with the field `colour` to distinguish between them.

* _Block states_ are used for storing the position of the banner. Either:
    * `rotation`, a number in the range 0 through 15, if the banner stands on the floor, or:
    * `facing`, one of four cardinal directions (`east`, `north`, `south` or `west`), if the banner hangs on a wall.

    For the `Banner` struct, this positioning data is stored in one field, named `placement`, of the type `WallOrRotatedOnFloor`.

* _Block entity data_:
    * Optionally holds a custom name for the banner (`custom_name`),
    * as well as the `pattern`: A list of up to 6 coloured patterns.

Most of the data types themselves are straight-forward: We already have `Colour`, which is suited both for the base colour and for the colours of the patterns. `WallOrRotatedOnFloor` has previously been used for _signs_. The custom name uses a `String`. That leaves the list of coloured patterns, which needs a new type:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ColouredPattern {
    pub colour: Colour,
    pub pattern: BannerPattern,
}
```

The `pattern` naturally encodes as an `enum`:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BannerPattern {
    BaseColor,               // b (base)
    Base,                    // bs (bottom stripe)
    Chief,                   // ts (top stripe)
    PaleDexter,              // ls (left stripe)
    PaleSinister,            // rs (right sripe)
    (...)
}
```

Creating a `BannerPattern` from a string would significantly simplify conversion from block entity data...

```rust
impl From<&str> for BannerPattern {
    fn from(pattern_string: &str) -> Self {
        match pattern_string {
            "b" => Self::BaseColor,                // b (base)
            "bs" => Self::Base,                    // bs (bottom stripe)
            "ts" => Self::Chief,                   // ts (top stripe)
            "ls" => Self::PaleDexter,              // ls (left stripe)
            "rs" => Self::PaleSinister,            // rs (right sripe)
            (...)
        }
    }
}
```

... and the ability to convert from `BannerPattern` to string would significantly simplify the reverse operation:

```rust
// TODO!
```

TODO: Mapping from `BannerPattern` to _block ID_, _block state_ and _block entity data_.

TODO: Mapping from _block ID_, _block state_ and _block entity data_ to `BannerPattern`.

TODO: The "block entity" stuff.

TODO: Finish the walk-through of _banner_.

## Adding remaining book-keeping, congregate functions, etc.

TODO: Lighting information (transparence, etc.) Direction, material, waterlogged, etc. congregate functions (polling for traits and values). Long term goal: Automate most of these. For now: Manual work.

## Some rambling about what is found in various files

TODO: See what to do with the below rambling. Probably worth some kind of general overview of different data types and the philosophy behind them.

### /src/block.rs

The `Block` enum contains all block types and variants, either directly or indirectly. Any new block implementation therefore starts by adding the new block to the `Block` enum, or through extending one of the existing variants.

For the implementation, some data types (defined elsewhere) are worth mentioning:

* Bounded ints (/src/bounded\_ints.rs): Data types with restricted ranges for integer values. Naming is on the form Int[min]Through[max], where [min] is the lower (inclusive) value, and [max] is the upper (inclusive) value that the integer data type may hold. E.g. an `Int0Through3` can hold the values 0, 1, 2 and 3, but no other integer values.
* Colour (/src/colour.rs): Minecraft has 16 default colours, represented by the `Colour` enum. Blocks which have these colour variants should have a field of type `Colour`, or alternatively `Option<Colour>` if there is also an uncoloured version of the block.
* Materials (/src/material.rs): Some materials are common for a large array of blocks. Typically each type of block, such as e.g. "Slab", "Stair", etc., has 

### /src/chunk/palette.rs




## Checklist

- [ ] In _/src/block.rs_: Add new block to `Block` enum, or extend on one of the existing variants
    - [ ] This includes any additons or changes to _/src/bounded\_ints.rs_, _/src/material.rs_, _/src/colour.rs_, etc., as appropriate
    - [ ] This may include creating new files / structs, for the new or existing `Block` enum variant.
- [ ] In _/src/chunk/palette.rs_: Map from `PaletteItem` to _block ID_, in `PaletteItem::name()`.
- [ ] In _/src/chunk/palette.rs_: Map from `PaletteItem` to _block states_ in NBT format, in `PaletteItem::properties()`.
- [ ] In _/src/chunk/palette.rs_: Map from _block ID_, and if applicable from _block states_ (`properties`), to `PaletteItem`, in `from_section()`.


TODO: Split this document into separate chapters.

TODO: Check the checklist
