# Adding blocks with some (limited) _block state_

Some blocks come with additional state. Such state can be either fixed (e.g. rotational orientation), or the state can change during gameplay (e.g. hydration level of a block of farmland). For those blocks, in addition to ID conversion, it is also necessary to convert the _block state_ information. Block state information is held as data in the `Block` variant.

## Block representation

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

## Block export

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

## Block import

Finally, the conversion from block ID and NBT block state, to the `Block` variant. The NBT structure containing the block state is available at the needed location, within the `from_section()` function, as a variable named `properties`. From within that structure, block state _values_ are fetched by lookup on the _name_, then converted to the corerct data type for inclusion in the returned `Block` variant instance.

Mapping _farmland_ from block ID to `Block`, in _/src/chunk/palette.rs_:
```rust
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:farmland" => block(Block::Farmland {
                wetness: prop(&properties, "moisture"),
            }),
            (...)
```

For a more complex block, we could have made a dedicated function for creating the block from the `properties` variable. For `Block::Farmland` there is only one field, `wetness`, and so the full implementation is short. We fetch the value of the `moisture` block state from `properties` with the generic utility function `prop()`. When given the `properties` NBT structure and the name of the block state, it returns the value with the required type -- in this case an `Int0Through7`.

There are some instances where the `prop()` function doesn't work. In those instances there might be more manual work for extracting the block state and converting it to the correct type. Luckily for us, _farmland_ is not one of those instances.

## A second example implementation

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
            "minecraft:end_rod" => block(Block::EndRod {
                facing: facing_surface6(&properties),
            }),
            (...)
```

There are currently a whole mess of convenience functions for extracting values from the NBT structure. In the case of _end rod_, the `facing` value, with the block state name "facing", needs conversion to the type `Surface6`, and for that there is a dedicated function `facing_surface6()` for extracting the value from the NBT structure. If `Surface6` gets implementations of the required traits, then this can also be changed to use `prop()`, as was the case for _farmland_. To be specific, `prop()` works for data types with both the `FromStr`, `Default` and `Display` traits.

## Bookkeeping

Just as for trivial blocks and blocks with different variants, this is the point where the main implementation of the block is complete.

TODO: Link to page on helper functions, lighting categories (transparence), etc.

