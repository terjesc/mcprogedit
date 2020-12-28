#[derive(Clone, Debug, PartialEq)]
pub enum StatusEffect {
    Speed = 1,
    Slowness = 2,
    Haste = 3,
    MiningFatigue = 4,
    Strength = 5,
    InstantHealth = 6,
    InstantDamage = 7,
    JumpBoost = 8,
    Nausea = 9,
    Regeneration = 10,
    Resistance = 11,
    FireResistance = 12,
    WaterBreathing = 13,
    Invisibility = 14,
    Blindness = 15,
    NightVision = 16,
    Hunger = 17,
    Weakness = 18,
    Poison = 19,
    Wither = 20,
    HealthBoost = 21,
    Absorption = 22,
    Saturation = 23,
    Glowing = 24,
    Levitation = 25,
    Luck = 26,
    BadLuck = 27,
    SlowFalling = 28,
    ConduitPower = 29,
    DolphinsGrace = 30,
    BadOmen = 31,
    HeroOfTheVillage = 32,
}

impl From<i32> for StatusEffect {
    fn from(effect_number: i32) -> Self {
        match effect_number {
            1 => StatusEffect::Speed,
            2 => StatusEffect::Slowness,
            3 => StatusEffect::Haste,
            4 => StatusEffect::MiningFatigue,
            5 => StatusEffect::Strength,
            6 => StatusEffect::InstantHealth,
            7 => StatusEffect::InstantDamage,
            8 => StatusEffect::JumpBoost,
            9 => StatusEffect::Nausea,
            10 => StatusEffect::Regeneration,
            11 => StatusEffect::Resistance,
            12 => StatusEffect::FireResistance,
            13 => StatusEffect::WaterBreathing,
            14 => StatusEffect::Invisibility,
            15 => StatusEffect::Blindness,
            16 => StatusEffect::NightVision,
            17 => StatusEffect::Hunger,
            18 => StatusEffect::Weakness,
            19 => StatusEffect::Poison,
            20 => StatusEffect::Wither,
            21 => StatusEffect::HealthBoost,
            22 => StatusEffect::Absorption,
            23 => StatusEffect::Saturation,
            24 => StatusEffect::Glowing,
            25 => StatusEffect::Levitation,
            26 => StatusEffect::Luck,
            27 => StatusEffect::BadLuck,
            28 => StatusEffect::SlowFalling,
            29 => StatusEffect::ConduitPower,
            30 => StatusEffect::DolphinsGrace,
            31 => StatusEffect::BadOmen,
            32 => StatusEffect::HeroOfTheVillage,
            _ => panic!("Invalid status effect number: {}", effect_number),
        }
    }
}

impl From<&str> for StatusEffect {
    fn from(effect_name: &str) -> Self {
        match effect_name {
            "speed" => StatusEffect::Speed,
            "slowness" => StatusEffect::Slowness,
            "haste" => StatusEffect::Haste,
            "mining_fatigue" => StatusEffect::MiningFatigue,
            "strength" => StatusEffect::Strength,
            "instant_health" => StatusEffect::InstantHealth,
            "instant_damage" => StatusEffect::InstantDamage,
            "jump_boost" => StatusEffect::JumpBoost,
            "nausea" => StatusEffect::Nausea,
            "regeneration" => StatusEffect::Regeneration,
            "resistance" => StatusEffect::Resistance,
            "fire_resistance" => StatusEffect::FireResistance,
            "water_breathing" => StatusEffect::WaterBreathing,
            "invisibility" => StatusEffect::Invisibility,
            "blindness" => StatusEffect::Blindness,
            "night_vision" => StatusEffect::NightVision,
            "hunger" => StatusEffect::Hunger,
            "weakness" => StatusEffect::Weakness,
            "poison" => StatusEffect::Poison,
            "wither" => StatusEffect::Wither,
            "health_boost" => StatusEffect::HealthBoost,
            "absorption" => StatusEffect::Absorption,
            "saturation" => StatusEffect::Saturation,
            "glowing" => StatusEffect::Glowing,
            "levitation" => StatusEffect::Levitation,
            "luck" => StatusEffect::Luck,
            "unluck" => StatusEffect::BadLuck,
            "slow_falling" => StatusEffect::SlowFalling,
            "conduit_power" => StatusEffect::ConduitPower,
            "dolphins_grace" => StatusEffect::DolphinsGrace,
            "bad_omen" => StatusEffect::BadOmen,
            "hero_of_the_village" => StatusEffect::HeroOfTheVillage,
            _ => panic!("Invalid status effect name: {}", effect_name),
        }
    }
}

impl StatusEffect {
    pub fn is_neutral(&self) -> bool {
        match self {
            StatusEffect::Glowing => true,
            _ => false,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            StatusEffect::Slowness |
                StatusEffect::MiningFatigue |
                StatusEffect::Weakness |
                StatusEffect::InstantDamage |
                StatusEffect::Nausea |
                StatusEffect::Blindness |
                StatusEffect::Hunger |
                StatusEffect::Poison |
                StatusEffect::Wither |
                StatusEffect::Levitation |
                StatusEffect::BadLuck |
                StatusEffect::BadOmen => true,
            _ => false,
        }
    }

    pub fn is_positive(&self) -> bool {
        !self.is_neutral() && !self.is_negative()
    }
}
