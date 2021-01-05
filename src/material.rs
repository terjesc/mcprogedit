//! For describing material variants of blocks and items.

use std::convert::TryFrom;

/// Materials for Armour.
//TODO add descriptions for when materials were first introduced
#[derive(Clone, Debug, PartialEq)]
pub enum ArmourMaterial {
    Chainmail,
    Diamond,
    Gold,
    Iron,
    Leather,
    Netherite,
}

impl TryFrom<Material> for ArmourMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Chainmail => Ok(Self::Chainmail),
            Material::Diamond => Ok(Self::Diamond),
            Material::Gold => Ok(Self::Gold),
            Material::Iron => Ok(Self::Iron),
            Material::Leather => Ok(Self::Leather),
            Material::Netherite => Ok(Self::Netherite),
            _ => Err(()),
        }
    }
}

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

impl TryFrom<Material> for ButtonMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Birch => Ok(Self::Birch),
            Material::Crimson => Ok(Self::Crimson),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::Jungle => Ok(Self::Jungle),
            Material::Oak => Ok(Self::Oak),
            Material::PolishedBlackstone => Ok(Self::PolishedBlackstone),
            Material::Spruce => Ok(Self::Spruce),
            Material::Stone => Ok(Self::Stone),
            Material::Warped => Ok(Self::Warped),
            _ => Err(()),
        }
    }
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

impl TryFrom<Material> for CoralMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Bubble => Ok(Self::Bubble),
            Material::Brain => Ok(Self::Brain),
            Material::Fire => Ok(Self::Fire),
            Material::Horn => Ok(Self::Horn),
            Material::Tube => Ok(Self::Tube),
            _ => Err(()),
        }
    }
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

impl TryFrom<Material> for DoorMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Birch => Ok(Self::Birch),
            Material::Crimson => Ok(Self::Crimson),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::Iron => Ok(Self::Iron),
            Material::Jungle => Ok(Self::Jungle),
            Material::Oak => Ok(Self::Oak),
            Material::Spruce => Ok(Self::Spruce),
            Material::Warped => Ok(Self::Warped),
            _ => Err(()),
        }
    }
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

impl TryFrom<Material> for FenceMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Birch => Ok(Self::Birch),
            Material::Crimson => Ok(Self::Crimson),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::Jungle => Ok(Self::Jungle),
            Material::NetherBrick => Ok(Self::NetherBrick),
            Material::Oak => Ok(Self::Oak),
            Material::Spruce => Ok(Self::Spruce),
            Material::Warped => Ok(Self::Warped),
            _ => Err(()),
        }
    }
}

/// Materials for Horse Armor.
//TODO add descriptions for when materials were first introduced
#[derive(Clone, Debug, PartialEq)]
pub enum HorseArmorMaterial {
    Gold,
    Iron,
    Diamond,
}

impl TryFrom<Material> for HorseArmorMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Gold => Ok(Self::Gold),
            Material::Iron => Ok(Self::Iron),
            Material::Diamond => Ok(Self::Diamond),
            _ => Err(()),
        }
    }
}

/// Materials for Ingots.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::IngotMaterial::Gold;
/// mcprogedit::material::IngotMaterial::Iron;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::IngotMaterial::Netherite;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum IngotMaterial {
    Gold,
    Iron,
    Netherite,
}

impl TryFrom<Material> for IngotMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Gold => Ok(Self::Gold),
            Material::Iron => Ok(Self::Iron),
            Material::Netherite => Ok(Self::Netherite),
            _ => Err(()),
        }
    }
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

impl TryFrom<Material> for LeavesMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Birch => Ok(Self::Birch),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::Jungle => Ok(Self::Jungle),
            Material::Oak => Ok(Self::Oak),
            Material::Spruce => Ok(Self::Spruce),
            _ => Err(()),
        }
    }
}

/// All materials.
///
/// Convertible to and from all other material types.
pub enum Material {
    Acacia,
    Andesite,
    Bamboo,
    Beetroot,
    Birch,
    Blackstone,
    Brain,
    Brick,
    Bubble,
    Chainmail,
    Cobblestone,
    Crimson,
    CutRedSandstone,
    CutSandstone,
    DarkOak,
    DarkPrismarine,
    Diamond,
    Diorite,
    EndStoneBrick,
    Fire,
    Gold,
    Granite,
    Horn,
    Iron,
    Jungle,
    Leather,
    Melon,
    MossyCobblestone,
    MossyStoneBrick,
    NetherBrick,
    Netherite,
    Oak,
    PetrifiedOak,
    PolishedAndesite,
    PolishedBlackstone,
    PolishedBlackstoneBrick,
    PolishedDiorite,
    PolishedGranite,
    Prismarine,
    PrismarineBrick,
    Pumpkin,
    Purpur,
    Quartz,
    RedNetherBrick,
    RedSandstone,
    Sandstone,
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStone,
    Spruce,
    Stone,
    StoneBrick,
    Tube,
    Warped,
    Wheat,
    Wood,
}

impl From<ArmourMaterial> for Material {
    fn from(item: ArmourMaterial) -> Self {
        match item {
            ArmourMaterial::Chainmail => Self::Chainmail,
            ArmourMaterial::Diamond => Self::Diamond,
            ArmourMaterial::Gold => Self::Gold,
            ArmourMaterial::Iron => Self::Iron,
            ArmourMaterial::Leather => Self::Leather,
            ArmourMaterial::Netherite => Self::Netherite,
        }
    }
}

impl From<ButtonMaterial> for Material {
    fn from(item: ButtonMaterial) -> Self {
        match item {
            ButtonMaterial::Acacia => Self::Acacia,
            ButtonMaterial::Birch => Self::Birch,
            ButtonMaterial::Crimson => Self::Crimson,
            ButtonMaterial::DarkOak => Self::DarkOak,
            ButtonMaterial::Jungle => Self::Jungle,
            ButtonMaterial::Oak => Self::Oak,
            ButtonMaterial::PolishedBlackstone => Self::PolishedBlackstone,
            ButtonMaterial::Spruce => Self::Spruce,
            ButtonMaterial::Stone => Self::Stone,
            ButtonMaterial::Warped => Self::Warped,
        }
    }
}

impl From<CoralMaterial> for Material {
    fn from(item: CoralMaterial) -> Self {
        match item {
            CoralMaterial::Bubble => Self::Bubble,
            CoralMaterial::Brain => Self::Brain,
            CoralMaterial::Fire => Self::Fire,
            CoralMaterial::Horn => Self::Horn,
            CoralMaterial::Tube => Self::Tube,
        }
    }
}

impl From<DoorMaterial> for Material {
    fn from(item: DoorMaterial) -> Self {
        match item {
            DoorMaterial::Acacia => Self::Acacia,
            DoorMaterial::Birch => Self::Birch,
            DoorMaterial::Crimson => Self::Crimson,
            DoorMaterial::DarkOak => Self::DarkOak,
            DoorMaterial::Iron => Self::Iron,
            DoorMaterial::Jungle => Self::Jungle,
            DoorMaterial::Oak => Self::Oak,
            DoorMaterial::Spruce => Self::Spruce,
            DoorMaterial::Warped => Self::Warped,
        }
    }
}

impl From<FenceMaterial> for Material {
    fn from(item: FenceMaterial) -> Self {
        match item {
            FenceMaterial::Acacia => Self::Acacia,
            FenceMaterial::Birch => Self::Birch,
            FenceMaterial::Crimson => Self::Crimson,
            FenceMaterial::DarkOak => Self::DarkOak,
            FenceMaterial::Jungle => Self::Jungle,
            FenceMaterial::NetherBrick => Self::NetherBrick,
            FenceMaterial::Oak => Self::Oak,
            FenceMaterial::Spruce => Self::Spruce,
            FenceMaterial::Warped => Self::Warped,
        }
    }
}

impl From<HorseArmorMaterial> for Material {
    fn from(item: HorseArmorMaterial) -> Self {
        match item {
            HorseArmorMaterial::Gold => Self::Gold,
            HorseArmorMaterial::Iron => Self::Iron,
            HorseArmorMaterial::Diamond => Self::Diamond,
        }
    }
}

impl From<IngotMaterial> for Material {
    fn from(item: IngotMaterial) -> Self {
        match item {
            IngotMaterial::Gold => Self::Gold,
            IngotMaterial::Iron => Self::Iron,
            IngotMaterial::Netherite => Self::Netherite,
        }
    }
}

impl From<LeavesMaterial> for Material {
    fn from(item: LeavesMaterial) -> Self {
        match item {
            LeavesMaterial::Acacia => Self::Acacia,
            LeavesMaterial::Birch => Self::Birch,
            LeavesMaterial::DarkOak => Self::DarkOak,
            LeavesMaterial::Jungle => Self::Jungle,
            LeavesMaterial::Oak => Self::Oak,
            LeavesMaterial::Spruce => Self::Spruce,
        }
    }
}

impl From<NuggetMaterial> for Material {
    fn from(item: NuggetMaterial) -> Self {
        match item {
            NuggetMaterial::Gold => Self::Gold,
            NuggetMaterial::Iron => Self::Iron,
        }
    }
}

impl From<PressurePlateMaterial> for Material {
    fn from(item: PressurePlateMaterial) -> Self {
        match item {
            PressurePlateMaterial::Acacia => Self::Acacia,
            PressurePlateMaterial::Birch => Self::Birch,
            PressurePlateMaterial::Crimson => Self::Crimson,
            PressurePlateMaterial::DarkOak => Self::DarkOak,
            PressurePlateMaterial::Gold => Self::Gold,
            PressurePlateMaterial::Iron => Self::Iron,
            PressurePlateMaterial::Jungle => Self::Jungle,
            PressurePlateMaterial::Oak => Self::Oak,
            PressurePlateMaterial::PolishedBlackstone => Self::PolishedBlackstone,
            PressurePlateMaterial::Spruce => Self::Spruce,
            PressurePlateMaterial::Stone => Self::Stone,
            PressurePlateMaterial::Warped => Self::Warped,
        }
    }
}

impl From<SaplingMaterial> for Material {
    fn from(item: SaplingMaterial) -> Self {
        match item {
            SaplingMaterial::Acacia => Self::Acacia,
            SaplingMaterial::Bamboo => Self::Bamboo,
            SaplingMaterial::Birch => Self::Birch,
            SaplingMaterial::DarkOak => Self::DarkOak,
            SaplingMaterial::Jungle => Self::Jungle,
            SaplingMaterial::Oak => Self::Oak,
            SaplingMaterial::Spruce => Self::Spruce,
        }
    }
}

impl From<SeedMaterial> for Material {
    fn from(item: SeedMaterial) -> Self {
        match item {
            SeedMaterial::Beetroot => Self::Beetroot,
            SeedMaterial::Melon => Self::Melon,
            SeedMaterial::Pumpkin => Self::Pumpkin,
            SeedMaterial::Wheat => Self::Wheat,
        }
    }
}

impl From<SlabMaterial> for Material {
    fn from(item: SlabMaterial) -> Self {
        match item {
            SlabMaterial::Acacia => Self::Acacia,
            SlabMaterial::Andesite => Self::Andesite,
            SlabMaterial::Birch => Self::Birch,
            SlabMaterial::Blackstone => Self::Blackstone,
            SlabMaterial::Brick => Self::Brick,
            SlabMaterial::Cobblestone => Self::Cobblestone,
            SlabMaterial::Crimson => Self::Crimson,
            SlabMaterial::CutRedSandstone => Self::CutRedSandstone,
            SlabMaterial::CutSandstone => Self::CutSandstone,
            SlabMaterial::DarkOak => Self::DarkOak,
            SlabMaterial::DarkPrismarine => Self::DarkPrismarine,
            SlabMaterial::Diorite => Self::Diorite,
            SlabMaterial::EndStoneBrick => Self::EndStoneBrick,
            SlabMaterial::Granite => Self::Granite,
            SlabMaterial::Jungle => Self::Jungle,
            SlabMaterial::MossyCobblestone => Self::MossyCobblestone,
            SlabMaterial::MossyStoneBrick => Self::MossyStoneBrick,
            SlabMaterial::NetherBrick => Self::NetherBrick,
            SlabMaterial::Oak => Self::Oak,
            SlabMaterial::PetrifiedOak => Self::PetrifiedOak,
            SlabMaterial::PolishedAndesite => Self::PolishedAndesite,
            SlabMaterial::PolishedBlackstone => Self::PolishedBlackstone,
            SlabMaterial::PolishedBlackstoneBrick => Self::PolishedBlackstoneBrick,
            SlabMaterial::PolishedDiorite => Self::PolishedDiorite,
            SlabMaterial::PolishedGranite => Self::PolishedGranite,
            SlabMaterial::Prismarine => Self::Prismarine,
            SlabMaterial::PrismarineBrick => Self::PrismarineBrick,
            SlabMaterial::Purpur => Self::Purpur,
            SlabMaterial::Quartz => Self::Quartz,
            SlabMaterial::RedNetherBrick => Self::RedNetherBrick,
            SlabMaterial::RedSandstone => Self::RedSandstone,
            SlabMaterial::Sandstone => Self::Sandstone,
            SlabMaterial::SmoothQuartz => Self::SmoothQuartz,
            SlabMaterial::SmoothRedSandstone => Self::SmoothRedSandstone,
            SlabMaterial::SmoothSandstone => Self::SmoothSandstone,
            SlabMaterial::SmoothStone => Self::SmoothStone,
            SlabMaterial::Spruce => Self::Spruce,
            SlabMaterial::Stone => Self::Stone,
            SlabMaterial::StoneBrick => Self::StoneBrick,
            SlabMaterial::Warped => Self::Warped,
        }
    }
}

impl From<StairMaterial> for Material {
    fn from(item: StairMaterial) -> Self {
        match item {
            StairMaterial::Acacia => Self::Acacia,
            StairMaterial::Andesite => Self::Andesite,
            StairMaterial::Birch => Self::Birch,
            StairMaterial::Blackstone => Self::Blackstone,
            StairMaterial::Brick => Self::Brick,
            StairMaterial::Cobblestone => Self::Cobblestone,
            StairMaterial::Crimson => Self::Crimson,
            StairMaterial::DarkOak => Self::DarkOak,
            StairMaterial::DarkPrismarine => Self::DarkPrismarine,
            StairMaterial::Diorite => Self::Diorite,
            StairMaterial::EndStoneBrick => Self::EndStoneBrick,
            StairMaterial::Granite => Self::Granite,
            StairMaterial::Jungle => Self::Jungle,
            StairMaterial::MossyCobblestone => Self::MossyCobblestone,
            StairMaterial::MossyStoneBrick => Self::MossyStoneBrick,
            StairMaterial::NetherBrick => Self::NetherBrick,
            StairMaterial::Oak => Self::Oak,
            StairMaterial::PolishedAndesite => Self::PolishedAndesite,
            StairMaterial::PolishedBlackstone => Self::PolishedBlackstone,
            StairMaterial::PolishedBlackstoneBrick => Self::PolishedBlackstoneBrick,
            StairMaterial::PolishedDiorite => Self::PolishedDiorite,
            StairMaterial::PolishedGranite => Self::PolishedGranite,
            StairMaterial::Prismarine => Self::Prismarine,
            StairMaterial::PrismarineBrick => Self::PrismarineBrick,
            StairMaterial::Purpur => Self::Purpur,
            StairMaterial::Quartz => Self::Quartz,
            StairMaterial::RedNetherBrick => Self::RedNetherBrick,
            StairMaterial::RedSandstone => Self::RedSandstone,
            StairMaterial::Sandstone => Self::Sandstone,
            StairMaterial::SmoothQuartz => Self::SmoothQuartz,
            StairMaterial::SmoothRedSandstone => Self::SmoothRedSandstone,
            StairMaterial::SmoothSandstone => Self::SmoothSandstone,
            StairMaterial::Spruce => Self::Spruce,
            StairMaterial::Stone => Self::Stone,
            StairMaterial::StoneBrick => Self::StoneBrick,
            StairMaterial::Warped => Self::Warped,
        }
    }
}

impl From<ToolMaterial> for Material {
    fn from(item: ToolMaterial) -> Self {
        match item {
            ToolMaterial::Diamond => Self::Diamond,
            ToolMaterial::Gold => Self::Gold,
            ToolMaterial::Iron => Self::Iron,
            ToolMaterial::Netherite => Self::Netherite,
            ToolMaterial::Stone => Self::Stone,
            ToolMaterial::Wood => Self::Wood,
        }
    }
}

impl From<WallMaterial> for Material {
    fn from(item: WallMaterial) -> Self {
        match item {
            WallMaterial::Andesite => Self::Andesite,
            WallMaterial::Blackstone => Self::Blackstone,
            WallMaterial::Brick => Self::Brick,
            WallMaterial::Cobblestone => Self::Cobblestone,
            WallMaterial::Diorite => Self::Diorite,
            WallMaterial::EndStoneBrick => Self::EndStoneBrick,
            WallMaterial::Granite => Self::Granite,
            WallMaterial::MossyCobblestone => Self::MossyCobblestone,
            WallMaterial::MossyStoneBrick => Self::MossyStoneBrick,
            WallMaterial::NetherBrick => Self::NetherBrick,
            WallMaterial::PolishedBlackstone => Self::PolishedBlackstone,
            WallMaterial::PolishedBlackstoneBrick => Self::PolishedBlackstoneBrick,
            WallMaterial::Prismarine => Self::Prismarine,
            WallMaterial::RedNetherBrick => Self::RedNetherBrick,
            WallMaterial::RedSandstone => Self::RedSandstone,
            WallMaterial::Sandstone => Self::Sandstone,
            WallMaterial::StoneBrick => Self::StoneBrick,
        }
    }
}

impl From<WoodMaterial> for Material {
    fn from(item: WoodMaterial) -> Self {
        match item {
            WoodMaterial::Acacia => Self::Acacia,
            WoodMaterial::Birch => Self::Birch,
            WoodMaterial::Crimson => Self::Crimson,
            WoodMaterial::DarkOak => Self::DarkOak,
            WoodMaterial::Jungle => Self::Jungle,
            WoodMaterial::Oak => Self::Oak,
            WoodMaterial::Spruce => Self::Spruce,
            WoodMaterial::Warped => Self::Warped,
        }
    }
}

/// Materials for the "Nugget" family of items.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since Beta 1.9 Prerelease
/// mcprogedit::material::NuggetMaterial::Gold;
/// ```
/// ## Introduced in Minecraft 1.11.1
/// ```
/// // Since 16w50a
/// mcprogedit::material::NuggetMaterial::Iron;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NuggetMaterial {
    Gold,
    Iron,
}

impl TryFrom<Material> for NuggetMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Gold => Ok(Self::Gold),
            Material::Iron => Ok(Self::Iron),
            _ => Err(()),
        }
    }
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

impl TryFrom<Material> for PressurePlateMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Birch => Ok(Self::Birch),
            Material::Crimson => Ok(Self::Crimson),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::Gold => Ok(Self::Gold),
            Material::Iron => Ok(Self::Iron),
            Material::Jungle => Ok(Self::Jungle),
            Material::Oak => Ok(Self::Oak),
            Material::PolishedBlackstone => Ok(Self::PolishedBlackstone),
            Material::Spruce => Ok(Self::Spruce),
            Material::Stone => Ok(Self::Stone),
            Material::Warped => Ok(Self::Warped),
            _ => Err(()),
        }
    }
}

/// Materials for Saplings.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::SaplingMaterial::Oak;
///
/// // From Minecraft Beta 1.5
/// mcprogedit::material::SaplingMaterial::Birch;
/// mcprogedit::material::SaplingMaterial::Spruce;
/// ```
/// ## Introduced in Minecraft 1.2.1
/// ```
/// // From 12w04a
/// mcprogedit::material::SaplingMaterial::Jungle;
/// ```
/// ## Introduced in Minecraft 1.7.2
/// ```
/// // From 13w43a
/// mcprogedit::material::SaplingMaterial::Acacia;
/// mcprogedit::material::SaplingMaterial::DarkOak;
/// ```
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

impl TryFrom<Material> for SaplingMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Bamboo => Ok(Self::Bamboo),
            Material::Birch => Ok(Self::Birch),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::Jungle => Ok(Self::Jungle),
            Material::Oak => Ok(Self::Oak),
            Material::Spruce => Ok(Self::Spruce),
            _ => Err(()),
        }
    }
}

/// Materials for Seeds.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::SeedMaterial::Wheat;
///
/// // From Minecraft Beta 1.8
/// mcprogedit::material::SeedMaterial::Melon;
/// mcprogedit::material::SeedMaterial::Pumpkin;
/// ```
/// ## Introduced in Minecraft 1.9
/// ```
/// // From 15w31a
/// mcprogedit::material::SeedMaterial::Beetroot;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SeedMaterial {
    Beetroot,
    Melon,
    Pumpkin,
    Wheat,
}

impl TryFrom<Material> for SeedMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Beetroot => Ok(Self::Beetroot),
            Material::Melon => Ok(Self::Melon),
            Material::Pumpkin => Ok(Self::Pumpkin),
            Material::Wheat => Ok(Self::Wheat),
            _ => Err(()),
        }
    }
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

impl TryFrom<Material> for SlabMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Andesite => Ok(Self::Andesite),
            Material::Birch => Ok(Self::Birch),
            Material::Blackstone => Ok(Self::Blackstone),
            Material::Brick => Ok(Self::Brick),
            Material::Cobblestone => Ok(Self::Cobblestone),
            Material::Crimson => Ok(Self::Crimson),
            Material::CutRedSandstone => Ok(Self::CutRedSandstone),
            Material::CutSandstone => Ok(Self::CutSandstone),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::DarkPrismarine => Ok(Self::DarkPrismarine),
            Material::Diorite => Ok(Self::Diorite),
            Material::EndStoneBrick => Ok(Self::EndStoneBrick),
            Material::Granite => Ok(Self::Granite),
            Material::Jungle => Ok(Self::Jungle),
            Material::MossyCobblestone => Ok(Self::MossyCobblestone),
            Material::MossyStoneBrick => Ok(Self::MossyStoneBrick),
            Material::NetherBrick => Ok(Self::NetherBrick),
            Material::Oak => Ok(Self::Oak),
            Material::PetrifiedOak => Ok(Self::PetrifiedOak),
            Material::PolishedAndesite => Ok(Self::PolishedAndesite),
            Material::PolishedBlackstone => Ok(Self::PolishedBlackstone),
            Material::PolishedBlackstoneBrick => Ok(Self::PolishedBlackstoneBrick),
            Material::PolishedDiorite => Ok(Self::PolishedDiorite),
            Material::PolishedGranite => Ok(Self::PolishedGranite),
            Material::Prismarine => Ok(Self::Prismarine),
            Material::PrismarineBrick => Ok(Self::PrismarineBrick),
            Material::Purpur => Ok(Self::Purpur),
            Material::Quartz => Ok(Self::Quartz),
            Material::RedNetherBrick => Ok(Self::RedNetherBrick),
            Material::RedSandstone => Ok(Self::RedSandstone),
            Material::Sandstone => Ok(Self::Sandstone),
            Material::SmoothQuartz => Ok(Self::SmoothQuartz),
            Material::SmoothRedSandstone => Ok(Self::SmoothRedSandstone),
            Material::SmoothSandstone => Ok(Self::SmoothSandstone),
            Material::SmoothStone => Ok(Self::SmoothStone),
            Material::Spruce => Ok(Self::Spruce),
            Material::Stone => Ok(Self::Stone),
            Material::StoneBrick => Ok(Self::StoneBrick),
            Material::Warped => Ok(Self::Warped),
            _ => Err(()),
        }
    }
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

impl TryFrom<Material> for StairMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Andesite => Ok(Self::Andesite),
            Material::Birch => Ok(Self::Birch),
            Material::Blackstone => Ok(Self::Blackstone),
            Material::Brick => Ok(Self::Brick),
            Material::Cobblestone => Ok(Self::Cobblestone),
            Material::Crimson => Ok(Self::Crimson),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::DarkPrismarine => Ok(Self::DarkPrismarine),
            Material::Diorite => Ok(Self::Diorite),
            Material::EndStoneBrick => Ok(Self::EndStoneBrick),
            Material::Granite => Ok(Self::Granite),
            Material::Jungle => Ok(Self::Jungle),
            Material::MossyCobblestone => Ok(Self::MossyCobblestone),
            Material::MossyStoneBrick => Ok(Self::MossyStoneBrick),
            Material::NetherBrick => Ok(Self::NetherBrick),
            Material::Oak => Ok(Self::Oak),
            Material::PolishedAndesite => Ok(Self::PolishedAndesite),
            Material::PolishedBlackstone => Ok(Self::PolishedBlackstone),
            Material::PolishedBlackstoneBrick => Ok(Self::PolishedBlackstoneBrick),
            Material::PolishedDiorite => Ok(Self::PolishedDiorite),
            Material::PolishedGranite => Ok(Self::PolishedGranite),
            Material::Prismarine => Ok(Self::Prismarine),
            Material::PrismarineBrick => Ok(Self::PrismarineBrick),
            Material::Purpur => Ok(Self::Purpur),
            Material::Quartz => Ok(Self::Quartz),
            Material::RedNetherBrick => Ok(Self::RedNetherBrick),
            Material::RedSandstone => Ok(Self::RedSandstone),
            Material::Sandstone => Ok(Self::Sandstone),
            Material::SmoothQuartz => Ok(Self::SmoothQuartz),
            Material::SmoothRedSandstone => Ok(Self::SmoothRedSandstone),
            Material::SmoothSandstone => Ok(Self::SmoothSandstone),
            Material::Spruce => Ok(Self::Spruce),
            Material::Stone => Ok(Self::Stone),
            Material::StoneBrick => Ok(Self::StoneBrick),
            Material::Warped => Ok(Self::Warped),
            _ => Err(()),
        }
    }
}

/// Materials for tools and swords.
///
/// # Variant availability
/// ## Introduced in Minecraft Beta
/// ```
/// // Since time immemorial
/// mcprogedit::material::ToolMaterial::Diamond;
/// mcprogedit::material::ToolMaterial::Gold;
/// mcprogedit::material::ToolMaterial::Iron;
/// mcprogedit::material::ToolMaterial::Stone;
/// mcprogedit::material::ToolMaterial::Wood;
/// ```
/// ## Introduced in Minecraft 1.16
/// ```
/// // From 20w06a
/// mcprogedit::material::ToolMaterial::Netherite;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ToolMaterial {
    Diamond,
    Gold,
    Iron,
    Netherite,
    Stone,
    Wood,
}

impl TryFrom<Material> for ToolMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Diamond => Ok(Self::Diamond),
            Material::Gold => Ok(Self::Gold),
            Material::Iron => Ok(Self::Iron),
            Material::Netherite => Ok(Self::Netherite),
            Material::Stone => Ok(Self::Stone),
            Material::Wood => Ok(Self::Wood),
            _ => Err(()),
        }
    }
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
/// mcprogedit::material::WallMaterial::EndStoneBrick;
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
    EndStoneBrick,
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

impl TryFrom<Material> for WallMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Andesite => Ok(Self::Andesite),
            Material::Blackstone => Ok(Self::Blackstone),
            Material::Brick => Ok(Self::Brick),
            Material::Cobblestone => Ok(Self::Cobblestone),
            Material::Diorite => Ok(Self::Diorite),
            Material::EndStoneBrick => Ok(Self::EndStoneBrick),
            Material::Granite => Ok(Self::Granite),
            Material::MossyCobblestone => Ok(Self::MossyCobblestone),
            Material::MossyStoneBrick => Ok(Self::MossyStoneBrick),
            Material::NetherBrick => Ok(Self::NetherBrick),
            Material::PolishedBlackstone => Ok(Self::PolishedBlackstone),
            Material::PolishedBlackstoneBrick => Ok(Self::PolishedBlackstoneBrick),
            Material::Prismarine => Ok(Self::Prismarine),
            Material::RedNetherBrick => Ok(Self::RedNetherBrick),
            Material::RedSandstone => Ok(Self::RedSandstone),
            Material::Sandstone => Ok(Self::Sandstone),
            Material::StoneBrick => Ok(Self::StoneBrick),
            _ => Err(()),
        }
    }
}

/// Materials for the "FenceGate", "Log", "Planks" and "Sign" families of blocks, and Boats.
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
    Acacia,
    Birch,
    Crimson,
    DarkOak,
    Jungle,
    Oak,
    Spruce,
    Warped,
}

impl TryFrom<Material> for WoodMaterial {
    type Error = ();

    fn try_from(item: Material) -> Result<Self, Self::Error> {
        match item {
            Material::Acacia => Ok(Self::Acacia),
            Material::Birch => Ok(Self::Birch),
            Material::Crimson => Ok(Self::Crimson),
            Material::DarkOak => Ok(Self::DarkOak),
            Material::Jungle => Ok(Self::Jungle),
            Material::Oak => Ok(Self::Oak),
            Material::Spruce => Ok(Self::Spruce),
            Material::Warped => Ok(Self::Warped),
            _ => Err(()),
        }
    }
}
