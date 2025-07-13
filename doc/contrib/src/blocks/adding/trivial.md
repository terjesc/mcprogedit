# Adding trivial blocks

For blocks without variants or block states, the `Block` enum variant doesn't need any data, and the rest of the implementation is pretty straight-forward.

## Block representation

For the block implementation, we must add the block variant to the `Block` definition, in _/src/block.rs_. E.g. for adding a _bedrock_ block:

```rust
pub enum Block {
    (...)
    Bedrock,
    (...)
```

## API functionality

In the case of trivial blocks, they are simply created and probed using the `Block` variant directly, e.g. `Block::Bedrock`. No further block API is needed.

## Block export

For export, we must add the mapping from `Block` variant to block ID, in _/src/chunk/palette.rs_. Note that for this mapping we are working on a `PaletteItem`, which is simply an enum with a variant named `Block`, which in turn holds the `Block` enum. The mapping is added to the `match self` in the function `name()` of the `PaletteItem` implementation, and it maps from `PaletteItem::Block(Block::Bedrock)` to the textual block ID string:

```rust
impl PaletteItem {
    (...)
    fn name(&self) -> &str {
        match self {
            (...)
            PaletteItem::Block(Block::Bedrock) => "minecraft:bedrock",
            (...)
```

Since the trivial block contains neither block state nor block entity data, no further actions are needed for the export.

## Block import

For import, we must add the mapping back from the block ID to the `Block` variant, also in _/src/chunk/palette.rs_. For this, the convenience function `block()` conveniently creates a `PaletteItem` from a `Block`, leaving us with a short and easy addition inside the `let palette_item = match name.as_str()` inside of the `from_section` function:

```rust
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:bedrock" => block(Block::Bedrock),
            (...)
```

Since the trivial block contains neither block state nor block entity data, no further actions are needed for the import.

## Bookkeeping

Due to the simplicity of trivial blocks, the main part of the implementation is complete at this point, having added only three lines of code. However there is still some minimal bookkeeping, as with all blocks.

TODO: Link to page on bookkeeping.
