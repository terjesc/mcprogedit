# Adding blocks to mcprogedit

First of all, the new block needs a _representation_ in mcprogedit. It also needs some _API functionality_, both for creating and modifying the block, and for probing its contents.

As was seen in the previous chapter, Minecraft and mcprogedit stores blocks in different ways. While block information in Minecraft are spread across three different sources, blocks in mcprogedit are tighter structures with the data for a block centralized in one location. This has implications for the required _translation_, both ways, between the two formats.

There are also some parts and systems in mcprogedit which needs to know about the new block and its properties, leading to the need for some additional _bookkeeping_.

Taking into account the above, adding a block therefore consists of five main steps:

1. Block representation
    - What data structures best hold the required information about a given block?
2. API functionality
    - For creating, modifying and probing the block and its data.
3. Block export
    - Translation from the mcprogedit format, to the block ID, block state and block entity data required by the Minecraft save format.
4. Block import
    - Translation from the block ID, block state and block entity data from the Minecraft save format, to the mcprogedit format.
5. Bookkeeping
    - Registering the block and its properties in other parts of mcprogedit.

The remainder of this chapter provides a detailed view on how to perform the above steps. First, in general terms what is needed, and where. Second, walk-throughs of adding blocks of varying complexity. Third, a convenient checklist for the steps and sub-steps required.
