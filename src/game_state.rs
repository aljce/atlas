// Game state module for Magic: The Gathering Amulet Titan simulation

use crate::cards::{Card, Land, Spell, Permanent, CardType, card_type};
use rand::rngs::StdRng;
use rand::SeedableRng;
use enum_map::EnumMap;

// ============================================================================
// MAIN GAME STATE
// ============================================================================

#[derive(Debug, Clone)]
pub struct GameState {
    pub life_total: i32,
    pub hand: Hand,
    pub battlefield: Battlefield,
    pub graveyard: Graveyard,
    pub mana_pool: ManaPool,
    pub library: Library,
    pub stack: Stack,
}

// ============================================================================
// GRAVEYARD
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Graveyard {
    pub spells: Vec<Spell>,
    pub lands: Vec<Land>,
}


// ============================================================================
// HAND
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Hand {
    pub lands: Vec<Land>,
    pub spells: Vec<Spell>,
}

// ============================================================================
// BATTLEFIELD
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Battlefield {
    pub lands: Vec<Tapped<Land>>,
    pub non_lands: Vec<Permanent>,
    pub land_plays: usize,
}

// ============================================================================
// TAPPED WRAPPER
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Tapped<A> {
    pub permanent: A,
    pub is_tapped: bool,
}

// ============================================================================
// MANA POOL
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ManaPool {
    pub white: usize,
    pub blue: usize,
    pub black: usize,
    pub red: usize,
    pub green: usize,
    pub colorless: usize,
}



// ============================================================================
// STACK
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    pub spells: Vec<Spell>,
}

// ============================================================================
// LIBRARY
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct SortedLibrary {
    pub cards: EnumMap<Card, u8>,
}

#[derive(Debug, Clone)]
pub struct Library {
    pub cards: EnumMap<Card, u8>,
    pub size: usize,
    pub rng: StdRng,
}

impl Library {
    /// Creates a new library with the given cards and RNG seed
    pub fn new(cards: Vec<Card>, seed: u64) -> Self {
        let size = cards.len();
        let mut card_counts = EnumMap::default();
        for card in cards {
            card_counts[card] += 1;
        }
        let rng = StdRng::seed_from_u64(seed);
        Library {
            cards: card_counts,
            size,
            rng,
        }
    }

    /// Returns the number of cards in the library
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns true if the library is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns a reference to the sorted library (cards are already sorted in EnumMap)
    pub fn as_sorted(&self) -> SortedLibrary {
        SortedLibrary {
            cards: self.cards.clone(),
        }
    }

    /// Draws a random card from the library, returns None if library is empty
    pub fn draw_random_card(&mut self) -> Option<Card> {
        use rand::Rng;

        if self.size == 0 {
            return None;
        }

        // Create a vector of available cards (cards with count > 0)
        let mut available_cards = Vec::new();
        for (card, &count) in &self.cards {
            if count > 0 {
                // Add each card type 'count' times to represent the probability
                for _ in 0..count {
                    available_cards.push(card);
                }
            }
        }

        // Pick a random card from the available cards
        let random_index = self.rng.gen_range(0..available_cards.len());
        let drawn_card = available_cards[random_index];

        // Decrease the count for this card type
        self.cards[drawn_card] -= 1;
        self.size -= 1;

        Some(drawn_card)
    }
}

// ============================================================================
// GRAVEYARD IMPLEMENTATION
// ============================================================================

impl Graveyard {
    /// Returns an iterator over the cards in the graveyard
    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        let land_cards = self.lands.iter().map(|&land| Card::Land(land));
        let spell_cards = self.spells.iter().map(|&spell| Card::Spell(spell));

        land_cards.chain(spell_cards)
    }

    /// Calculates if delirium is active (4 or more different card types in graveyard)
    pub fn has_delirium(&self) -> bool {
        self.iter()
            .map(card_type)
            .fold(CardType::empty(), std::ops::BitOr::bitor)
            .bits()
            .count_ones()
            >= 4
    }
}