# The Minecraft block storage format

## Three pieces of block information

Each block of a Minecraft world can be made up of up to three different pieces of data: _Block ID_, _block states_, and _block entity data_. Those pieces are described next.

### Block ID

All Minecraft blocks are identified through a textual _block ID_, such as `"minecraft:Air"` or `"minecraft:Dirt"`. The block ID is always present, regardless of the existence of block states or block entity data.

### Block states

Many blocks also come with _block states_, describing properties of a particular instance of the block in the Minecraft world. Such block states can describe fixed properties, such as the rotational orientation of a chest or block of stairs. There are also block states describing changing properties, such as the power level of a redstone wire, the on/off status of a lever, or the growth stage of a beetroot.

Each block state has a _name_ used for lookup, a set of _allowed values_, and a _default value_. As an example, _wheat crops_ has a block state whose name is _age_, holding a value in the range 0 through 7, with a default value of 0. This state encodes the growth stage of the wheat crops, from newly sown (0) to ripe (7). Other block states have textual data (e.g. "north", "top").

### Block entity data

A small number of blocks have additional data in the form of a _block entity_. A typical example is a _chest_, which can hold a large inventory of items.

The block entity is used for storing complex data, and is formatted using an _NBT structure_. NBT objects have a tree structure, where data is stored as _nodes_ in the tree. Each node has a _tag_ and a value, where the value can be another node, or hold a list or map of several other nodes.

All block entities have a standard set of nodes, as well as nodes which are specific to the block entity of one particular block ID. Basic information is still stored as block states, even for blocks associated with a block entity.

### Documentation for block ID, block states and block entity data

Formats and values for block ID, block states and block entity data are documented under the _Data values_ section of the Minecraft wiki entry for each block. Block ID is listed as _ID_ or _identifier_. For blocks with block states, those are listed under _Block states_, and for blocks with a block entity the full NBT structure is documented under _Block data_.

## The location of blocks within a Minecraft world save

In the current save format, blocks are stored within _chunks_, which are smaller sections of the Minecraft world. Each chunk holds 16 x 16 blocks in the horizontal plane, and extend to the full height and depth of the world along the y axis. The chunk is further subdivided into _sections_, each 16 blocks tall, leaving a cube of 16 x 16 x 16 blocks, 4096 blocks in total, to each section.

Among other data, each section holds an array of 4096 blocks, with each position in that array mapping to a location within the 16^3 cube. Instead of storing the block's ID and states directly, the block array stores _indexes_ into a different array; the _palette_. The palette in turn holds _each distinct block ID + block states combination_ from within the section. Any block entities come in addition to the palette, with each block entity mapping to a single block within the section.

The use of palettes saves storage space, since sections typically hold a lot of repeating blocks. It does however come at the expense of added complexity. Nevertheless, in practical terms this has implications only for _where_ the different parts of a block implementation go. Luckliy the main _complexity_ of chunks, sections and palettes can be abstracted away in the respective modules. To a large extent the same goes for block entities.

## Final warnings

The Minecraft storage format _has_ changed in the past, and it _might_ do so again in the future. For instance, prior to Minecraft 1.13, block ID was an 8 bit integer, further bits were used for block states, and there was no palette. The switch from numeric IDs, along with other accompanying changes, were so impactful that the Minecraft community coined a name for it: [_The Flattening_](https://minecraft.wiki/w/Java_Edition_Flattening).

Perhaps more relevant are _small_ changes, in particular with newly introduced blocks from recent snapshot versions of Minecraft. For those blocks, some behaviour, or the block ID, block states or block entity data, might change from version to version, until they are stabilized. Sometimes even long lasting blocks might change. This means some blocks may eventually need a more complicated implementation in mcprogedit, in order to be compliant with multiple Minecraft versions. Maybe more code related to import and export must become version aware.

