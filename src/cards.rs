// Magic: The Gathering card enums for Amulet Titan deck simulation

use bitflags::bitflags;
use enum_map::Enum;

// ============================================================================
// CARD TYPE BITFLAGS
// ============================================================================

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CardType: u8 {
        const LAND = 1 << 0;
        const ARTIFACT = 1 << 1;
        const ENCHANTMENT = 1 << 2;
        const CREATURE = 1 << 3;
        const SORCERY = 1 << 4;
        const INSTANT = 1 << 5;
    }
}

// ============================================================================
// MANA VALUE STRUCT
// ============================================================================

/// Mana value representation with individual mana costs
#[derive(Debug, Clone, PartialEq)]
pub struct ManaValue {
    pub white: u8,
    pub blue: u8,
    pub black: u8,
    pub red: u8,
    pub green: u8,
    pub colorless: u8,
    pub generic: u8,
    pub x: u8,
}

// ============================================================================
// MAIN CARD ENUM
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum Card {
    Land(Land),
    Spell(Spell),
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum Spell {
    Permanent(Permanent),
    NonPermanent(NonPermanent),
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum Permanent {
    // Artifacts
    AmuletOfVigor,

    // Enchantments
    Spelunking,

    // Creatures
    AftermathAnalyst,
    ArborealGrazer,
    CultivatorColossus,
    PrimevalTitan,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum NonPermanent {
    Sorcery(Sorcery),
    Instant(Instant),
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum Sorcery {
    Explore,
    GreenSunsZenith,
    Scapeshift,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum Instant {
    SummonersPact,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum Land {
    BoseijuWhoEndures,
    CrumblingVestige,
    EchoingDeeps,
    Forest,
    GruulTurf,
    HanweirBattlements,
    LotusField,
    Mirrorpool,
    OtawaraSoaringCity,
    ShiftingWoodland,
    SimicGrowthChamber,
    TheMycosynthGardens,
    TolariaWest,
    UrzasCave,
    UrzasSaga,
    Vesuva,
}

// ============================================================================
// MANA VALUE TRAIT
// ============================================================================

pub trait HasManaValue {
    fn mana_value(&self) -> ManaValue;
}

// ============================================================================
// CARD TYPE FUNCTION
// ============================================================================

pub const fn card_type(card: Card) -> CardType {
    match card {
        Card::Land(land) => {
            match land {
                // Urza's Saga is both Land and Enchantment
                Land::UrzasSaga => CardType::LAND.union(CardType::ENCHANTMENT),
                // All other lands are just Land
                _ => CardType::LAND,
            }
        }
        Card::Spell(spell) => {
            match spell {
                Spell::Permanent(permanent) => {
                    match permanent {
                        // Artifacts
                        Permanent::AmuletOfVigor => CardType::ARTIFACT,

                        // Enchantments
                        Permanent::Spelunking => CardType::ENCHANTMENT,

                        // Creatures
                        Permanent::AftermathAnalyst | Permanent::ArborealGrazer |
                        Permanent::CultivatorColossus | Permanent::PrimevalTitan => CardType::CREATURE,
                    }
                }
                Spell::NonPermanent(non_permanent) => {
                    match non_permanent {
                        NonPermanent::Sorcery(sorcery) => {
                            match sorcery {
                                Sorcery::Explore | Sorcery::GreenSunsZenith | Sorcery::Scapeshift => CardType::SORCERY,
                            }
                        }
                        NonPermanent::Instant(instant) => {
                            match instant {
                                Instant::SummonersPact => CardType::INSTANT,
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// HAS MANA VALUE TRAIT IMPLEMENTATIONS
// ============================================================================

impl HasManaValue for Spell {
    fn mana_value(&self) -> ManaValue {
        match self {
            Spell::Permanent(permanent) => permanent.mana_value(),
            Spell::NonPermanent(non_permanent) => non_permanent.mana_value(),
        }
    }
}

impl HasManaValue for Permanent {
    fn mana_value(&self) -> ManaValue {
        match self {
            // Artifacts
            Permanent::AmuletOfVigor => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 0,
                colorless: 0, generic: 1, x: 0
            },

            // Enchantments
            Permanent::Spelunking => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 1,
                colorless: 0, generic: 2, x: 0
            },

            // Creatures
            Permanent::AftermathAnalyst => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 1,
                colorless: 0, generic: 0, x: 0
            },
            Permanent::ArborealGrazer => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 1,
                colorless: 0, generic: 0, x: 0
            },
            Permanent::CultivatorColossus => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 3,
                colorless: 0, generic: 1, x: 0
            },
            Permanent::PrimevalTitan => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 2,
                colorless: 0, generic: 4, x: 0
            },
        }
    }
}

impl HasManaValue for NonPermanent {
    fn mana_value(&self) -> ManaValue {
        match self {
            NonPermanent::Sorcery(sorcery) => sorcery.mana_value(),
            NonPermanent::Instant(instant) => instant.mana_value(),
        }
    }
}

impl HasManaValue for Sorcery {
    fn mana_value(&self) -> ManaValue {
        match self {
            Sorcery::Explore => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 1,
                colorless: 0, generic: 1, x: 0
            },
            Sorcery::GreenSunsZenith => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 1,
                colorless: 0, generic: 0, x: 1
            },
            Sorcery::Scapeshift => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 2,
                colorless: 0, generic: 2, x: 0
            },
        }
    }
}

impl HasManaValue for Instant {
    fn mana_value(&self) -> ManaValue {
        match self {
            Instant::SummonersPact => ManaValue {
                white: 0, blue: 0, black: 0, red: 0, green: 0,
                colorless: 0, generic: 0, x: 0
            },
        }
    }
}

