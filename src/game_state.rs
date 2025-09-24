// Game state module for Magic: The Gathering Amulet Titan simulation

use crate::cards::{Card, Land, Spell, Permanent, CardType, card_type, ManaValue};
use std::collections::HashMap;

// ============================================================================
// MAIN GAME STATE
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub life_total: i32,
    pub hand: Hand,
    pub battlefield: Battlefield,
    pub graveyard: Graveyard,
    pub mana_pool: ManaPool,
    pub library: Library,
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
// LIBRARY
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum LibraryState {
    Sorted(HashMap<Card, u8>),
    Shuffled(Vec<Card>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Library {
    pub state: LibraryState,
    pub size: usize,
}

impl Library {
    /// Creates a new shuffled library with the given cards
    pub fn new(cards: Vec<Card>) -> Self {
        let size = cards.len();
        Library {
            state: LibraryState::Shuffled(cards),
            size,
        }
    }

    /// Draws the top card from the library
    pub fn draw(&mut self) -> Option<Card> {
        let card = match &mut self.state {
            LibraryState::Shuffled(cards) => cards.pop(),
            LibraryState::Sorted(card_map) => {
                // For sorted, we need to pick a card and decrement its count
                if let Some((&card, &count)) = card_map.iter().next() {
                    let card = card.clone();
                    if count > 1 {
                        card_map.insert(card.clone(), count - 1);
                    } else {
                        card_map.remove(&card);
                    }
                    Some(card)
                } else {
                    None
                }
            }
        };

        if card.is_some() {
            self.size = self.size.saturating_sub(1);
        }

        card
    }

    /// Returns the number of cards in the library
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns true if the library is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

// ============================================================================
// GRAVEYARD IMPLEMENTATION
// ============================================================================

impl Graveyard {
    /// Returns an iterator over the cards in the graveyard
    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        let land_cards = self.lands.iter().map(|land| Card::Land(land.clone()));
        let spell_cards = self.spells.iter().map(|spell| Card::Spell(spell.clone()));

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