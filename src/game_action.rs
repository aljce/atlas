// Game action trait for Magic: The Gathering Amulet Titan simulation

use crate::game_state::*;
use crate::cards::*;

/// Trait for game actions that can be applied and reverted

pub enum PrimitiveGameAction {
    DrawCards(usize),
    MillCards(usize),
    PlayLand(Land, TapState),
    IncreaseLandPlays(usize),
    SearchLibraryToHand(Vec<Card>),
    SearchLibraryToBattlefield(Vec<GameObject<Card>>),
    Trigger(Trigger)
}

pub enum GameAction {
    PassPriority,
    Primitive(PrimitiveGameAction),
    CastSpell(Spell),
    ActivateAbility {
        source: GameObjectId,
        target: Option<Target>,
    },
    Sequence(Vec<PrimitiveGameAction>),
}

pub enum PrimitiveGameActionResult {
    DrawCards(Vec<Card>),
    MillCards(Vec<Card>),
    PlayLand(GameObjectId),
    IncreaseLandPlays(usize),
    SearchLibraryToHand(Vec<Card>),
    SearchLibraryToBattlefield(Vec<GameObjectId>),
    Trigger,
}

pub enum GameActionResult {
    PassPriority,
    Primitive(PrimitiveGameActionResult),
    CastSpell,
    ActivateAbility(GameObjectId),
    Sequence(Vec<PrimitiveGameActionResult>),
}

impl GameAction {
    pub fn apply(&self, game_state: &mut GameState) -> GameActionResult {
        match self {
            GameAction::PassPriority => {
                // Switch priority to the other player
                game_state.priority = match game_state.priority {
                    PlayerId::Active => PlayerId::NonActive,
                    PlayerId::NonActive => PlayerId::Active,
                };
                GameActionResult::PassPriority
            }
            GameAction::Primitive(primitive_action) => {
                let result = primitive_action.apply(game_state);
                GameActionResult::Primitive(result)
            }
            GameAction::CastSpell(spell) => {
                // Add spell to stack
                game_state.stack.objects.push(StackObject::Spell(*spell));
                GameActionResult::CastSpell
            }
            GameAction::ActivateAbility { source, target } => {
                // Add activated ability to stack
                let ability = StackObject::ActivatedAbility {
                    source: *source,
                    target: target.clone(),
                };
                game_state.stack.objects.push(ability);
                GameActionResult::ActivateAbility(*source)
            }
            GameAction::Sequence(actions) => {
                let mut results = Vec::new();
                for action in actions {
                    let result = action.apply(game_state);
                    results.push(result);
                }
                GameActionResult::Sequence(results)
            }
        }
    }
}

impl PrimitiveGameAction {
    pub fn apply(&self, game_state: &mut GameState) -> PrimitiveGameActionResult {
        match self {
            PrimitiveGameAction::DrawCards(count) => {
                let mut drawn_cards = Vec::new();
                for _ in 0..*count {
                    if let Some(card) = game_state.active_player.library.draw_random_card() {
                        match card {
                            Card::Land(land) => game_state.active_player.hand.lands.push(land),
                            Card::Spell(spell) => game_state.active_player.hand.spells.push(spell),
                        }
                        drawn_cards.push(card);
                    }
                }
                PrimitiveGameActionResult::DrawCards(drawn_cards)
            }
            PrimitiveGameAction::MillCards(count) => {
                let mut milled_cards = Vec::new();
                for _ in 0..*count {
                    if let Some(card) = game_state.active_player.library.draw_random_card() {
                        match card {
                            Card::Land(land) => game_state.active_player.graveyard.lands.push(land),
                            Card::Spell(spell) => game_state.active_player.graveyard.spells.push(spell),
                        }
                        milled_cards.push(card);
                    }
                }
                PrimitiveGameActionResult::MillCards(milled_cards)
            }
            PrimitiveGameAction::PlayLand(land, tap_state) => {
                let id = game_state.next_game_object_id();
                let game_object = GameObject {
                    permanent: *land,
                    tap_state: *tap_state,
                };
                game_state.active_player.battlefield.lands.insert(id, game_object);
                PrimitiveGameActionResult::PlayLand(id)
            }
            PrimitiveGameAction::IncreaseLandPlays(amount) => {
                game_state.active_player.battlefield.land_plays += amount;
                PrimitiveGameActionResult::IncreaseLandPlays(*amount)
            }
            PrimitiveGameAction::SearchLibraryToHand(cards) => {
                for card in cards {
                    // Remove card from library
                    game_state.active_player.library.cards[*card] =
                        game_state.active_player.library.cards[*card].saturating_sub(1);
                    game_state.active_player.library.size =
                        game_state.active_player.library.size.saturating_sub(1);

                    // Add to hand
                    match card {
                        Card::Land(land) => game_state.active_player.hand.lands.push(*land),
                        Card::Spell(spell) => game_state.active_player.hand.spells.push(*spell),
                    }
                }
                PrimitiveGameActionResult::SearchLibraryToHand(cards.clone())
            }
            PrimitiveGameAction::SearchLibraryToBattlefield(game_objects) => {
                let mut object_ids = Vec::new();
                for game_object in game_objects {
                    // Remove card from library
                    game_state.active_player.library.cards[game_object.permanent] =
                        game_state.active_player.library.cards[game_object.permanent].saturating_sub(1);
                    game_state.active_player.library.size =
                        game_state.active_player.library.size.saturating_sub(1);

                    // Add to battlefield
                    match game_object.permanent {
                        Card::Land(land) => {
                            let id = game_state.next_game_object_id();
                            let battlefield_object = GameObject {
                                permanent: land,
                                tap_state: game_object.tap_state,
                            };
                            game_state.active_player.battlefield.lands.insert(id, battlefield_object);
                            object_ids.push(id);
                        }
                        Card::Spell(Spell::Permanent(permanent)) => {
                            let id = game_state.next_game_object_id();
                            let battlefield_object = GameObject {
                                permanent: permanent,
                                tap_state: game_object.tap_state,
                            };
                            game_state.active_player.battlefield.non_lands.insert(id, battlefield_object);
                            object_ids.push(id);
                        }
                        Card::Spell(Spell::NonPermanent(_)) => {
                            // Non-permanent spells can't enter the battlefield
                            // This should probably be an error, but for now we'll skip it
                        }
                    }
                }
                PrimitiveGameActionResult::SearchLibraryToBattlefield(object_ids)
            }
            PrimitiveGameAction::Trigger(trigger) => {
                game_state.stack.objects.push(StackObject::Trigger(trigger.clone()));
                PrimitiveGameActionResult::Trigger
            }
        }
    }
}

impl PrimitiveGameActionResult {
    pub fn revert(&self, game_state: &mut GameState) {
        match self {
            PrimitiveGameActionResult::DrawCards(cards) => {
                // Count how many lands and spells were drawn
                let mut lands_drawn = 0;
                let mut spells_drawn = 0;
                for card in cards {
                    match card {
                        Card::Land(_) => lands_drawn += 1,
                        Card::Spell(_) => spells_drawn += 1,
                    }
                    game_state.active_player.library.add_card(*card);
                }

                // Truncate the hand vectors to remove the drawn cards
                let lands_len = game_state.active_player.hand.lands.len();
                let spells_len = game_state.active_player.hand.spells.len();

                game_state.active_player.hand.lands.truncate(lands_len.saturating_sub(lands_drawn));
                game_state.active_player.hand.spells.truncate(spells_len.saturating_sub(spells_drawn));

            }
            PrimitiveGameActionResult::MillCards(cards) => {
                // Remove cards from graveyard and add them back to library
                for card in cards.iter().rev() {
                    match card {
                        Card::Land(land) => {
                            if let Some(pos) = game_state.active_player.graveyard.lands.iter().position(|&l| l == *land) {
                                game_state.active_player.graveyard.lands.remove(pos);
                            }
                        }
                        Card::Spell(spell) => {
                            if let Some(pos) = game_state.active_player.graveyard.spells.iter().position(|&s| s == *spell) {
                                game_state.active_player.graveyard.spells.remove(pos);
                            }
                        }
                    }
                    game_state.active_player.library.add_card(*card);
                }
            }
            PrimitiveGameActionResult::PlayLand(id) => {
                // Remove land from battlefield
                game_state.active_player.battlefield.lands.remove(id);
            }
            PrimitiveGameActionResult::IncreaseLandPlays(amount) => {
                // Decrease land plays
                game_state.active_player.battlefield.land_plays =
                    game_state.active_player.battlefield.land_plays.saturating_sub(*amount);
            }
            PrimitiveGameActionResult::SearchLibraryToHand(cards) => {
                // Remove cards from hand and add them back to library
                for card in cards.iter().rev() {
                    match card {
                        Card::Land(land) => {
                            if let Some(pos) = game_state.active_player.hand.lands.iter().position(|&l| l == *land) {
                                game_state.active_player.hand.lands.remove(pos);
                            }
                        }
                        Card::Spell(spell) => {
                            if let Some(pos) = game_state.active_player.hand.spells.iter().position(|&s| s == *spell) {
                                game_state.active_player.hand.spells.remove(pos);
                            }
                        }
                    }
                    game_state.active_player.library.add_card(*card);
                }
            }
            PrimitiveGameActionResult::SearchLibraryToBattlefield(object_ids) => {
                // Remove objects from battlefield and add corresponding cards back to library
                for id in object_ids.iter().rev() {
                    if let Some(land_object) = game_state.active_player.battlefield.lands.remove(id) {
                        game_state.active_player.library.add_card(Card::Land(land_object.permanent));
                    } else if let Some(permanent_object) = game_state.active_player.battlefield.non_lands.remove(id) {
                        game_state.active_player.library.add_card(Card::Spell(Spell::Permanent(permanent_object.permanent)));
                    }
                }
            }
            PrimitiveGameActionResult::Trigger => {
                // Remove the last trigger from the stack
                game_state.stack.objects.pop();
            }
        }
    }
}

impl GameActionResult {
    pub fn revert(&self, game_state: &mut GameState) {
        match self {
            GameActionResult::PassPriority => {
                // Switch priority back to the previous player
                game_state.priority = match game_state.priority {
                    PlayerId::Active => PlayerId::NonActive,
                    PlayerId::NonActive => PlayerId::Active,
                };
            }
            GameActionResult::Primitive(primitive_result) => {
                primitive_result.revert(game_state);
            }
            GameActionResult::CastSpell => {
                // Remove the last spell from the stack
                if let Some(StackObject::Spell(_)) = game_state.stack.objects.last() {
                    game_state.stack.objects.pop();
                }
            }
            GameActionResult::ActivateAbility(_source) => {
                // Remove the last activated ability from the stack
                if let Some(StackObject::ActivatedAbility { .. }) = game_state.stack.objects.last() {
                    game_state.stack.objects.pop();
                }
            }
            GameActionResult::Sequence(results) => {
                // Revert actions in reverse order
                for result in results.iter().rev() {
                    result.revert(game_state);
                }
            }
        }
    }
}