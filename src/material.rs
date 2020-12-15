//! For describing material variants of blocks.

/// Materials for the "Button" family of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::ButtonMaterial::Stone;
/// ```
/// ## Introduced in Minecraft 1.4.2
/// ```
/// // From 12w34a
/// mcprogedit::material::ButtonMaterial::Oak;
/// ```
/// ## Introduced in Minecraft 1.13
/// ```
/// // From 17w47a
/// mcprogedit::material::ButtonMaterial::Acacia;
/// mcprogedit::material::ButtonMaterial::Birch;
/// mcprogedit::material::ButtonMaterial::DarkOak;
/// mcprogedit::material::ButtonMaterial::Jungle;
/// mcprogedit::material::ButtonMaterial::Spruce;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::ButtonMaterial::Crimson;
/// mcprogedit::material::ButtonMaterial::Warped;
///
/// // From 20w15a
/// mcprogedit::material::ButtonMaterial::PolishedBlackstone;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonMaterial {
    Acacia,
    Birch,
    Crimson,
    DarkOak,
    Jungle,
    Oak,
    PolishedBlackstone,
    Spruce,
    Stone,
    Warped,
}

/// Materials for the "Coral", "CoralBlock" and "CoralFan" families of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft 1.13
/// ```
/// // From 18w09a (CoralBlock), 18w10a (Coral), 18w11a (CoralFan)
/// mcprogedit::material::CoralMaterial::Bubble;
/// mcprogedit::material::CoralMaterial::Brain;
/// mcprogedit::material::CoralMaterial::Fire;
/// mcprogedit::material::CoralMaterial::Horn;
/// mcprogedit::material::CoralMaterial::Tube;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum CoralMaterial {
    /// Purple coral variant
    Bubble,
    /// Pink coral variant
    Brain,
    /// Red coral variant
    Fire,
    /// Yellow coral variant
    Horn,
    /// Blue coral variant
    Tube,
}

/// Materials for the "Door" and "Trapdoor" families of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::DoorMaterial::Iron; // Door
/// mcprogedit::material::DoorMaterial::Oak;  // Door
///
/// // From Minecraft Beta 1.6 Test Build 3
/// mcprogedit::material::DoorMaterial::Oak;  // Trapdoor
/// ```
/// ## Introduced in Minecraft 1.8
/// ```
/// // From 14w07a
/// mcprogedit::material::DoorMaterial::Iron;  // Trapdoor
///
/// // From 14w32d
/// mcprogedit::material::DoorMaterial::Acacia;  // Door
/// mcprogedit::material::DoorMaterial::Birch;   // Door
/// mcprogedit::material::DoorMaterial::DarkOak; // Door
/// mcprogedit::material::DoorMaterial::Jungle;  // Door
/// mcprogedit::material::DoorMaterial::Spruce;  // Door
/// ```
/// ## Introduced in Minecraft 1.13
/// ```
/// // From 17w47a
/// mcprogedit::material::DoorMaterial::Acacia;  // Trapdoor
/// mcprogedit::material::DoorMaterial::Birch;   // Trapdoor
/// mcprogedit::material::DoorMaterial::DarkOak; // Trapdoor
/// mcprogedit::material::DoorMaterial::Jungle;  // Trapdoor
/// mcprogedit::material::DoorMaterial::Spruce;  // Trapdoor
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::DoorMaterial::Crimson; // Door, Trapdoor
/// mcprogedit::material::DoorMaterial::Warped;  // Door, Trapdoor
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum DoorMaterial {
    Acacia,
    Birch,
    Crimson,
    DarkOak,
    Iron,
    Jungle,
    Oak,
    Spruce,
    Warped,
}

/// Materials for the "Fence" family of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::FenceMaterial::Oak;
/// ```
/// ## Introduced in Minecraft 1.0.0
/// ```
/// // From Beta 1.9 Prerelease
/// mcprogedit::material::FenceMaterial::NetherBrick;
/// ```
/// ## Introduced in Minecraft 1.8
/// ```
/// // From 14w32b
/// mcprogedit::material::FenceMaterial::Acacia;
/// mcprogedit::material::FenceMaterial::Birch;
/// mcprogedit::material::FenceMaterial::DarkOak;
/// mcprogedit::material::FenceMaterial::Jungle;
/// mcprogedit::material::FenceMaterial::Spruce;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::FenceMaterial::Crimson;
/// mcprogedit::material::FenceMaterial::Warped;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum FenceMaterial {
    Acacia,
    Birch,
    Crimson,
    DarkOak,
    Jungle,
    NetherBrick,
    Oak,
    Spruce,
    Warped,
}

/// Materials for the "Leaves" family of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::LeavesMaterial::Oak;
///
/// // From Minecraft Beta 1.2
/// mcprogedit::material::LeavesMaterial::Birch;
/// mcprogedit::material::LeavesMaterial::Spruce;
/// ```
/// ## Introduced in Minecraft 1.2.1
/// ```
/// // From 12w03a
/// mcprogedit::material::LeavesMaterial::Jungle;
/// ```
/// ## Introduced in Minecraft 1.7.2
/// ```
/// // From 13w43a
/// mcprogedit::material::LeavesMaterial::Acacia;
/// mcprogedit::material::LeavesMaterial::DarkOak;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum LeavesMaterial {
    Acacia,
    Birch,
    DarkOak,
    Jungle,
    Oak,
    Spruce,
}

/// Materials for the "PressurePlate" family of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::PressurePlateMaterial::Oak;
/// mcprogedit::material::PressurePlateMaterial::Stone;
/// ```
/// ## Introduced in Minecraft 1.5
/// ```
/// // From 13w01a
/// mcprogedit::material::PressurePlateMaterial::Gold;
/// mcprogedit::material::PressurePlateMaterial::Iron;
/// ```
/// ## Introduced in Minecraft 1.13
/// ```
/// // From 17w47a
/// mcprogedit::material::PressurePlateMaterial::Acacia;
/// mcprogedit::material::PressurePlateMaterial::Birch;
/// mcprogedit::material::PressurePlateMaterial::DarkOak;
/// mcprogedit::material::PressurePlateMaterial::Jungle;
/// mcprogedit::material::PressurePlateMaterial::Spruce;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::PressurePlateMaterial::Crimson;
/// mcprogedit::material::PressurePlateMaterial::Warped;
///
/// // From 20w15a
/// mcprogedit::material::PressurePlateMaterial::PolishedBlackstone;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum PressurePlateMaterial {
    Acacia,
    Birch,
    Crimson,
    DarkOak,
    Gold,
    Iron,
    Jungle,
    Oak,
    PolishedBlackstone,
    Spruce,
    Stone,
    Warped,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SaplingMaterial {
    Acacia,
    Bamboo,
    Birch,
    DarkOak,
    Jungle,
    Oak,
    Spruce,
}

/// Materials for the "Slab" family of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::SlabMaterial::SmoothStone;
///
/// // From Minecraft Beta 1.3
/// mcprogedit::material::SlabMaterial::Cobblestone;
/// mcprogedit::material::SlabMaterial::Oak;
/// mcprogedit::material::SlabMaterial::Sandstone;
///
/// // From Minecraft Beta 1.8 / Pre-release
/// mcprogedit::material::SlabMaterial::Brick;
/// mcprogedit::material::SlabMaterial::StoneBrick;
/// ```
/// ## Introduced in Minecraft 1.3.1
/// ```
/// // From 12w17a
/// mcprogedit::material::SlabMaterial::Birch;
/// mcprogedit::material::SlabMaterial::Jungle;
/// mcprogedit::material::SlabMaterial::Spruce;
/// ```
/// ## Introduced in Minecraft 1.4.6
/// ```
/// // From 12w49a
/// mcprogedit::material::SlabMaterial::NetherBrick;
/// ```
/// ## Introduced in Minecraft 1.5
/// ```
/// // From 13w02a
/// mcprogedit::material::SlabMaterial::Quartz;
/// ```
/// ## Introduced in Minecraft 1.7.2
/// ```
/// // From 1.7
/// mcprogedit::material::SlabMaterial::Acacia;
/// mcprogedit::material::SlabMaterial::DarkOak;
/// ```
/// ## Introduced in Minecraft 1.8
/// ```
/// // From 14w32a
/// mcprogedit::material::SlabMaterial::RedSandstone;
/// ```
/// ## Introduced in Minecraft 1.9
/// ```
/// // From 15w31a
/// mcprogedit::material::SlabMaterial::Purpur;
/// ```
/// ## Introduced in Minecraft 1.13
/// ```
/// // From 18w07a
/// mcprogedit::material::SlabMaterial::DarkPrismarine;
/// mcprogedit::material::SlabMaterial::Prismarine;
/// mcprogedit::material::SlabMaterial::PrismarineBrick;
/// ```
/// ## Introduced in Minecraft 1.14
/// ```
/// // From 18w43a
/// mcprogedit::material::SlabMaterial::Andesite;
/// mcprogedit::material::SlabMaterial::Diorite;
/// mcprogedit::material::SlabMaterial::Granite;
/// mcprogedit::material::SlabMaterial::EndStoneBrick;
/// mcprogedit::material::SlabMaterial::MossyCobblestone;;
/// mcprogedit::material::SlabMaterial::MossyStoneBrick;
/// mcprogedit::material::SlabMaterial::PolishedAndesite;
/// mcprogedit::material::SlabMaterial::PolishedDiorite;
/// mcprogedit::material::SlabMaterial::PolishedGranite;
/// mcprogedit::material::SlabMaterial::RedNetherBrick;
/// mcprogedit::material::SlabMaterial::SmoothQuartz;
/// mcprogedit::material::SlabMaterial::SmoothRedSandstone;
/// mcprogedit::material::SlabMaterial::SmoothSandstone;
/// mcprogedit::material::SlabMaterial::Stone;
///
/// // From 19w08a
/// mcprogedit::material::SlabMaterial::CutRedSandstone;
/// mcprogedit::material::SlabMaterial::CutSandstone;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::SlabMaterial::Crimson;
/// mcprogedit::material::SlabMaterial::Warped;
///
/// // From 20w15a
/// mcprogedit::material::SlabMaterial::Blackstone;
/// mcprogedit::material::SlabMaterial::PolishedBlackstone;
/// mcprogedit::material::SlabMaterial::PolishedBlackstoneBrick;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SlabMaterial {
    Acacia,
    Andesite,
    Birch,
    Blackstone,
    Brick,
    Cobblestone,
    Crimson,
    CutRedSandstone,
    CutSandstone,
    DarkOak,
    DarkPrismarine,
    Diorite,
    EndStoneBrick,
    Granite,
    Jungle,
    MossyCobblestone,
    MossyStoneBrick,
    NetherBrick,
    Oak,
    /// Only available for slabs and in creative. Slabs of this material are
    /// mined using a pickaxe. Probably exists for compatibility with old
    /// versions, where all slabs were mined using pickaxe.
    PetrifiedOak,
    PolishedAndesite,
    PolishedBlackstone,
    PolishedBlackstoneBrick,
    PolishedDiorite,
    PolishedGranite,
    Prismarine,
    PrismarineBrick,
    Purpur,
    Quartz,
    RedNetherBrick,
    RedSandstone,
    Sandstone,
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    /// The old "Stone Slab" was renamed to "Smooth Stone Slab" in 1.14 (18w43a),
    /// when a new "Stone Slab" was introduced.
    SmoothStone,
    Spruce,
    Stone,
    StoneBrick,
    Warped,
}

/// Materials for the "Stair" family of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Alpha
/// ```
/// // Since time immemorial
/// mcprogedit::material::StairMaterial::Cobblestone;
/// mcprogedit::material::StairMaterial::Oak;
/// ```
/// ## Introduced in Minecraft Beta 1.8
/// ```
/// // From Prerelease
/// mcprogedit::material::StairMaterial::Brick;
/// mcprogedit::material::StairMaterial::StoneBrick;
/// ```
/// ## Introduced in Minecraft 1.0.0
/// ```
/// // From Minecraft Beta 1.9 Prerelease
/// mcprogedit::material::StairMaterial::NetherBrick;
/// ```
/// ## Introduced in Minecraft 1.3.1
/// ```
/// // From 12w21a
/// mcprogedit::material::StairMaterial::Sandstone;
///
/// // From 12w25a
/// mcprogedit::material::StairMaterial::Birch;
/// mcprogedit::material::StairMaterial::Spruce;
/// mcprogedit::material::StairMaterial::Jungle;
/// ```
/// ## Introduced in Minecraft 1.5
/// ```
/// // From 13w02a
/// mcprogedit::material::StairMaterial::Quartz;
/// ```
/// ## Introduced in Minecraft 1.7.2
/// ```
/// // From the 1.7 prerelease
/// mcprogedit::material::StairMaterial::Acacia;
/// mcprogedit::material::StairMaterial::DarkOak;
/// ```
/// ## Introduced in Minecraft 1.8
/// ```
/// // From snapshot 14w32a
/// mcprogedit::material::StairMaterial::RedSandstone;
/// ```
/// ## Introduced in Minecraft 1.9 (15w31a)
/// ```
/// // From snapshot 15w31a
/// mcprogedit::material::StairMaterial::Purpur;
/// ```
/// ## Introduced in Minecraft 1.13
/// ```
/// // From snapshot 18w07a
/// mcprogedit::material::StairMaterial::DarkPrismarine;
/// mcprogedit::material::StairMaterial::Prismarine;
/// mcprogedit::material::StairMaterial::PrismarineBrick;
/// ```
/// ## Introduced in Minecraft 1.14
/// ```
/// // From snapshot 18w43a
/// mcprogedit::material::StairMaterial::Andesite;
/// mcprogedit::material::StairMaterial::Diorite;
/// mcprogedit::material::StairMaterial::EndStoneBrick;
/// mcprogedit::material::StairMaterial::Granite;
/// mcprogedit::material::StairMaterial::MossyCobblestone;
/// mcprogedit::material::StairMaterial::MossyStoneBrick;
/// mcprogedit::material::StairMaterial::PolishedAndesite;
/// mcprogedit::material::StairMaterial::PolishedDiorite;
/// mcprogedit::material::StairMaterial::PolishedGranite;
/// mcprogedit::material::StairMaterial::RedNetherBrick;
/// mcprogedit::material::StairMaterial::SmoothQuartz;
/// mcprogedit::material::StairMaterial::SmoothRedSandstone;
/// mcprogedit::material::StairMaterial::SmoothSandstone;
/// mcprogedit::material::StairMaterial::Stone;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From snapshot 20w06a
/// mcprogedit::material::StairMaterial::Crimson;
/// mcprogedit::material::StairMaterial::Warped;
///
/// // From snapshot 20w15a
/// mcprogedit::material::StairMaterial::Blackstone;
/// mcprogedit::material::StairMaterial::PolishedBlackstone;
/// mcprogedit::material::StairMaterial::PolishedBlackstoneBrick;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum StairMaterial {
    Acacia,
    Andesite,
    Birch,
    Blackstone,
    Brick,
    Cobblestone,
    Crimson,
    DarkOak,
    DarkPrismarine,
    Diorite,
    EndStoneBrick,
    Granite,
    Jungle,
    MossyCobblestone,
    MossyStoneBrick,
    NetherBrick,
    Oak,
    PolishedAndesite,
    PolishedBlackstone,
    PolishedBlackstoneBrick,
    PolishedDiorite,
    PolishedGranite,
    Prismarine,
    PrismarineBrick,
    Purpur,
    Quartz,
    RedNetherBrick,
    RedSandstone,
    Sandstone,
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    Spruce,
    Stone,
    StoneBrick,
    Warped,
}

/// Materials for the "Wall" family of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft 1.4.2
/// ```
/// // From 12w32a
/// mcprogedit::material::WallMaterial::Cobblestone;
/// mcprogedit::material::WallMaterial::MossyCobblestone;
/// ```
/// ## Introduced in Minecraft 1.14
/// ```
/// // From 18w43a
/// mcprogedit::material::WallMaterial::Andesite;
/// mcprogedit::material::WallMaterial::Brick;
/// mcprogedit::material::WallMaterial::Diorite;
/// mcprogedit::material::WallMaterial::EndStone;
/// mcprogedit::material::WallMaterial::Granite;
/// mcprogedit::material::WallMaterial::MossyStoneBrick;
/// mcprogedit::material::WallMaterial::NetherBrick;
/// mcprogedit::material::WallMaterial::Prismarine;
/// mcprogedit::material::WallMaterial::RedNetherBrick;
/// mcprogedit::material::WallMaterial::RedSandstone;
/// mcprogedit::material::WallMaterial::Sandstone;
/// mcprogedit::material::WallMaterial::StoneBrick;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w15a
/// mcprogedit::material::WallMaterial::Blackstone;
/// mcprogedit::material::WallMaterial::PolishedBlackstone;
/// mcprogedit::material::WallMaterial::PolishedBlackstoneBrick;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum WallMaterial {
    Andesite,
    Blackstone,
    Brick,
    Cobblestone,
    Diorite,
    EndStone,
    Granite,
    MossyCobblestone,
    MossyStoneBrick,
    NetherBrick,
    PolishedBlackstone,
    PolishedBlackstoneBrick,
    Prismarine,
    RedNetherBrick,
    RedSandstone,
    Sandstone,
    StoneBrick,
}

/// Materials for the "FenceGate", "Log", "Planks" and "Sign" families of blocks.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::WoodMaterial::Oak; // Log, Planks, Sign
///
/// // From Minecraft Beta 1.3
/// mcprogedit::material::WoodMaterial::Birch;  // Log
/// mcprogedit::material::WoodMaterial::Spruce; // Log
///
/// // From Minecraft Beta 1.8 / Pre-release
/// mcprogedit::material::WoodMaterial::Oak; // FenceGate
/// ```
/// ## Introduced in Minecraft 1.2.1
/// ```
/// // From 12w03a
/// mcprogedit::material::WoodMaterial::Jungle; // Log
/// ```
/// ## Introduced in Minecraft 1.2.4
/// ```
/// // From Minecraft 1.2.4 release
/// mcprogedit::material::WoodMaterial::Birch;  // Planks
/// mcprogedit::material::WoodMaterial::Jungle; // Planks
/// mcprogedit::material::WoodMaterial::Spruce; // Planks
/// ```
/// ## Introduced in Minecraft 1.7.2
/// ```
/// // From 13w43a
/// mcprogedit::material::WoodMaterial::Acacia;  // Log
/// mcprogedit::material::WoodMaterial::DarkOak; // Log
///
/// // From 1.7.1
/// mcprogedit::material::WoodMaterial::Acacia;  // Planks
/// mcprogedit::material::WoodMaterial::DarkOak; // Planks
/// ```
/// ## Introduced in Minecraft 1.8
/// ```
/// // From 14w32b
/// mcprogedit::material::WoodMaterial::Acacia;  // FenceGate
/// mcprogedit::material::WoodMaterial::Birch;   // FenceGate
/// mcprogedit::material::WoodMaterial::DarkOak; // FenceGate
/// mcprogedit::material::WoodMaterial::Jungle;  // FenceGate
/// mcprogedit::material::WoodMaterial::Spruce;  // FenceGate
/// ```
/// ## Introduced in Minecraft 1.13
/// ```
/// // From 18w07a
/// // "Stripped" Log variants, for all already introduced WoodMaterials.
/// ```
/// ## Introduced in Minecraft 1.14
/// ```
/// // From 18w43a
/// mcprogedit::material::WoodMaterial::Acacia;  // Sign
/// mcprogedit::material::WoodMaterial::Birch;   // Sign
/// mcprogedit::material::WoodMaterial::DarkOak; // Sign
/// mcprogedit::material::WoodMaterial::Jungle;  // Sign
/// mcprogedit::material::WoodMaterial::Spruce;  // Sign
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::WoodMaterial::Crimson; // FenceGate, Log, Planks, Sign
/// mcprogedit::material::WoodMaterial::Warped;  // FenceGate, Log, Planks, Sign
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum WoodMaterial {
    Oak,
    Spruce,
    Birch,
    Jungle,
    Acacia,
    DarkOak,
    Crimson,
    Warped,
}
