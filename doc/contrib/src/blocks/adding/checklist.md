# Checklist

1. Block representation
    - In _/src/block.rs_, Add the new block to the `Block` enum, or extend an existing variant.
    - The implementation might need new data structures, or changes to old ones.
2. Block export
    - In _/src/chunk/palette.rs_, map from `PaletteItem` to _block ID_ (formatted as String), in function `PaletteItem::name()`.
    - In _/src/chunk/palette.rs_, map from `PaletteItem` to _block states_ (formatted as NBT), in function `PaletteItem::properties()`.
    - TODO: Block instance data export.
3. Block import
    - In _/src/chunk/palette.rs_, map from _block ID_, and if applicable from _block states_ (`properties`), to `PaletteItem`, in function `from_section()`.
    - TODO: Block instance data import.
4. Bookkeeping
    - TODO: Lighting related stuff.
    - TODO: `Block` related functions for direction, facing, material, etc.
    - TODO: Any other bookkeeping needed?
