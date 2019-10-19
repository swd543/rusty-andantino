use crate::hex::Hex;
use crate::game_state_type::GameState::{BLACK, WHITE};

impl Hex{
    pub fn eval(&self)->i32{
        if self.winner.is_none() {return 0}
        else if self.winner.unwrap()==BLACK {return -1}
        else if self.winner.unwrap()==WHITE {return 1}
        0
    }
}