# Checklist

- [ ] In _/src/block.rs_: Add new block to `Block` enum, or extend on one of the existing variants
    - [ ] This includes any additons or changes to _/src/bounded\_ints.rs_, _/src/material.rs_, _/src/colour.rs_, etc., as appropriate
    - [ ] This may include creating new files / structs, for the new or existing `Block` enum variant.
- [ ] In _/src/chunk/palette.rs_: Map from `PaletteItem` to _block ID_, in `PaletteItem::name()`.
- [ ] In _/src/chunk/palette.rs_: Map from `PaletteItem` to _block states_ in NBT format, in `PaletteItem::properties()`.
- [ ] In _/src/chunk/palette.rs_: Map from _block ID_, and if applicable from _block states_ (`properties`), to `PaletteItem`, in `from_section()`.

TODO: Check the checklist

