# mcprogedit
Programmatically edit Minecraft savefiles and schematics

## What mcprogedit is
mcprogedit is currently in the prototyping stage, and very much a work in progress.

## What mcprogedit aims to be
* A library for importing, modifying, and exporting parts of a Minecraft world.
* Providing an easy-to-use and well documented API for modification of Minecraft worlds.
* Capable of importing and exporting Minecraft saves.
* Capable of importing and exporting schematic formats, such as the one used by e.g. MCEdit.
* Supporting Minecraft version 1.12.2 (for compatibility with older tools).
* Supporting the latest version of Minecraft, as well as keeping support for already supported Minecraft versions.

The main intended workflow for a program using mcprogedit is "import -> modify -> export", with most resources spent in the "modify" stage.

## What mcprogedit is not, and will probably never be
mcprogedit is not suited if you want to create a Minecraft server or client, as such usage scenarios are not taken into account for any design choices.
For instance, some block states may be outsourced to the game to figure out when it loads the exported world, or they are automatically added during export.
Such states may have no internal representation and/or no public API.
This may or may not include redstone signal strengths, stair shapes, fence post connections, water flow, and others.
