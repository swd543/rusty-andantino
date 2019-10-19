use crate::hex::Hex;
use crate::game_state_type::GameState::{BLACK, WHITE, INVALID};
use crate::convolution::{conv, filter, sum, conv_arr};
use crate::game_state_type::GameStateType;

const KERNEL:[[GameStateType;5];5]=[
    [1,-1,1,-1,-1],
    [-1,1,1,-1,-1],
    [1,1,1,1,1],
    [-1,-1,1,1,-1],
    [-1,-1,1,-1,1]
];

impl Hex{
    pub fn eval(&self)->i32{
        if self.winner.is_none() {
            let mut arr=self.board.clone();
            filter(arr.as_mut(),|x|x==INVALID as GameStateType || x!=WHITE as GameStateType && x!=BLACK as GameStateType);
            return sum(&conv_arr(&arr, &KERNEL)) as i32;
        }
        else if self.winner.unwrap()==BLACK {return std::i32::MIN}
        else if self.winner.unwrap()==WHITE {return std::i32::MAX}
        0
    }

    fn dumb_eval(&self)->i32{
        if self.winner.is_none() {return 0}
        else if self.winner.unwrap()==BLACK {return -1}
        else if self.winner.unwrap()==WHITE {return 1}
        0
    }
}