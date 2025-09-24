// Game action trait for Magic: The Gathering Amulet Titan simulation

use crate::game_state::GameState;

/// Trait for game actions that can be applied and reverted
pub trait GameAction {
    fn make_move(&mut self, _game_state: &mut GameState) {}
    fn unmake_move(&mut self, _game_state: &mut GameState) {}
}
