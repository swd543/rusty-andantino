use crate::hex::Hex;
use crate::havannah::Havannah;
use std::borrow::Borrow;

impl Hex{
    pub fn move_repeated(&mut self, times:i8, moves:&Vec<Havannah>, mut from:Havannah){
        for _i in 0..times{
            for m in moves{
                from=from.add(*m);
                self.move_game(from);
            }
        }
    }
}