use crate::hex::Hex;
use crate::game_state_type::GameState::{BLACK, WHITE, INVALID};
use crate::convolution::{conv, filter, sum, conv_arr};
use crate::game_state_type::GameStateType;
use std::collections::HashMap;

//pub const KERNEL:[[GameStateType;5];5]=[
//    [2,-1,2,-1,-1],
//    [-1,1,1,-1,-1],
//    [2,1,1,1,2],
//    [-1,-1,1,1,-1],
//    [-1,-1,2,-1,2]
//];

//pub const KERNEL:[[GameStateType;5];5]=[
//    [ 1, 1, 1,-1,-1],
//    [ 1, 2, 2, 1,-1],
//    [ 1, 2,-3, 2, 1],
//    [-1, 1, 2, 2, 1],
//    [-1,-1, 1, 1, 1]
//];
//
//pub const KERNEL:[[GameStateType;5];5]=[
//    [ 2, 1, 2,-1,-1],
//    [ 1, 3, 3, 1,-1],
//    [ 2, 3, 0, 3, 2],
//    [-1, 1, 3, 3, 1],
//    [-1,-1, 2, 1, 2]
//];

pub const KERNEL:[[GameStateType;5];5]=[
    [ 1,-1, 1,-1,-1],
    [-1, 2, 2,-1,-1],
    [ 1, 2, 0, 2, 1],
    [-1,-1, 2, 2,-1],
    [-1,-1, 1,-1, 1]
];

//pub const KERNEL:[[GameStateType;5];5]=[
//    [ 0, 0, 1, 0, 0],
//    [ 0, 0, 1, 0, 0],
//    [ 1, 1, 1, 1, 1],
//    [ 0, 0, 1, 0, 0],
//    [ 0, 0, 1, 0, 0],
//];


impl Hex{
    pub fn eval(&self)->i32{
        if self.winner.is_none() {
            let mut arr=self.board.clone();
            filter(arr.as_mut(),|x|x==INVALID as GameStateType || x!=WHITE as GameStateType && x!=BLACK as GameStateType);
            return sum(&conv_arr(&arr, &KERNEL)) as i32;
        }
        else if self.winner.unwrap()==BLACK {return std::i32::MAX-2}
        else if self.winner.unwrap()==WHITE {return std::i32::MIN+2}
        0
    }

    fn dumb_eval(&self)->i32{
        if self.winner.is_none() {return 0}
        else if self.winner.unwrap()==BLACK {return -1}
        else if self.winner.unwrap()==WHITE {return 1}
        0
    }
}