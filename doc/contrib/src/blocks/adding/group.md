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

## Block import

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

## Bookkeeping

If there are no block states or other data fields, then the main implementation is complete at this point, just as for implementing trivial blocks.

TODO: Link to page on helper functions, lighting categories (transparence), etc.

