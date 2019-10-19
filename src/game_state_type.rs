use crate::game_state_type::GameState::{BLACK, WHITE};

pub type GameStateType = i8;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum GameState {
    WHITE=-7,
    BLACK=7,
    NONE=0,
    INVALID=9
}

impl GameState {
    pub fn from_game(value: GameStateType) -> GameState {
        match value {
            -7 => WHITE,
            7 => BLACK,
            _ => {panic!("Invalid game state provided => {}!",value)}
        }
    }
}
