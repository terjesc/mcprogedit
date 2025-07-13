# Adding blocks with different variants

Sometimes it makes sense to represent multiple similar or related blocks as one `Block` enum variant, and use variant data to distinguish the related blocks. A prominent example is the set of clear and stained _glass_ blocks. In addition to the uncoloured (clear) variant, there are 16 coloured (stained) variants corresponding to the 16 main colours.

## Block representation

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

## API functionality

TODO

## Block export

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
                (...) // Skipping 12 entries, for brevity.
                Some(Colour::Red) => "minecraft:red_stained_glass",
                Some(Colour::Black) => "minecraft:black_stained_glass",
            }
            (...)
```

## Block import

Mapping _glass_ from block IDs to `Block`, in _/src/chunk/palette.rs_:

```rust
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    (...)
        let palette_item = match name.as_str() {
            (...)
            "minecraft:glass" => block(Block::Glass { colour: None }),
            "minecraft:white_stained_glass" => block(Block::Glass {
                colour: Some(Colour::White),
            }),
            "minecraft:orange_stained_glass" => block(Block::Glass {
                colour: Some(Colour::Orange),
            }),
            (...) // Skipping 12 entries, for brevity.
            "minecraft:red_stained_glass" => block(Block::Glass {
                colour: Some(Colour::Red),
            }),
            "minecraft:black_stained_glass" => block(Block::Glass {
                colour: Some(Colour::Black),
            }),
            (...)
```

## Bookkeeping

If there are no block states or other data fields, then the main implementation is complete at this point, just as for implementing trivial blocks.

Still there is some bookkeeping. In the case of _glass_ blocks, they have _colour_, and so should be added to the colour related functions of the `Block` enum itself.

TODO: Show colour related bookkeeping for `Block::Glass { .. }`.

Other blocks, with other properties, should be added to the `Block` API functions for those properties. Also, there is the relevant general bookkeeping needed for all new blocks.

TODO: Link to page on bookkeeping.
