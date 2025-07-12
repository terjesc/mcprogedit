# Adding blocks with block entity data

## Block representation

Blocks with block entity data are typically too large for holding the state directly as a `Block` enum variant. In those cases the variant definition in _/src/block.rs_ uses a `Box`:

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
    (...) Remaining entries left out for brevity.
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
            (...) Remaining entries left out for brevity.
        }
    }
}
```

... and the ability to convert from `BannerPattern` to string would significantly simplify the reverse operation:

```rust
// TODO!
```

## Preparations for import and export

In order to handle the block entity for _banner_, we need a Rust implementation for that block entity. Similar to how `Block` is an enum with variants for various blocks, `BlockEntity` is an enum with variants for various block entities. The definition and implementation of `BlockEntity` is found in _src/block_entity.rs_, and here is the definition of `BlockEntity::Banner`:

```rust
pub enum BlockEntity {
    (...)
    Banner {
        common: CommonTags,
        colour: Colour,
        custom_name: Option<String>,
        patterns: Vec<ColouredPattern>,
    },
    (...)
```

`BlockEntity::Banner` has the set of standard tags (`CommonTags`), as well as the custom name and patterns. Notice how the 16 different coloured variants of the banner block are also encoded here, in the block entity, even though the colour is not really part ofo the block entity data. The reason will become evident when we get to block export and import.

## Block export

TODO: Mapping from `BannerPattern` to _block ID_, _block state_ and _block entity data_.

## Block import

TODO: Mapping from _block ID_, _block state_ and _block entity data_ to `BannerPattern`.

## Bookkeeping

TODO: Link to page on helper functions, lighting categories (transparence), etc.

