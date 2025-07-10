# Implementing new blocks

Blocks are the basic building blocks of a Minecraft world. Some are cube shaped voxels, such as _dirt_ and _stone_. Others have various shapes, such as _cobweb_ and _dead bush_. Some have additional properties describing various states such as the block's orientation, variant or contents (e.g. _torch_, _lever_). Further, some of that state can be stored in convoluted ways by the game itself (e.g. _chest_, _furnace_). This guide aims to show how to add support to mcprogedit for all kinds of new blocks, regardless of complexity.

TODO: Overview of subsections.

## Adding blocks with many parameters

TODO: Block with more parameters. Example: Coral? CoralFan?

TODO: Blocks with struct. Example: Door?

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

TODO: Split this document into separate chapters.
