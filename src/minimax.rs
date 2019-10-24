use crate::hex::Hex;
use std::thread;
use crate::havannah::Havannah;
use std::borrow::Borrow;

impl Hex{
    pub fn minimax(&self, depth:i8, maximising:bool)->(i32,Hex){
        if self.is_game || depth==0{
            return (self.eval(),self.clone());
        }
        let mut children=vec![];
        let moves=self.get_possible_moves();
        for m in moves{
            let mut cloned=self.clone();
            cloned.move_game(m);
            cloned.check_win(m);
            children.push(cloned);
        }
        let mut score=0;
        if maximising{
            score=std::i32::MIN;
        } else {
            score = std::i32::MAX;
        }
        let mut optimal=self.clone();
        for c in children{
            let (value,pointer)=c.minimax(depth-1,!maximising);
            if maximising{
                if value>score  {score=value; optimal=pointer}
            } else {
                if value<score  {score=value; optimal=pointer}
            }
        }
        return (score, optimal);
    }

    pub fn minimax_parallel(&self, depth:i8, maximising:bool, branch_factor:i8)->(i32,Hex){
        if depth%branch_factor==0{
            return self.minimax(depth,maximising);
        }
        if self.is_game || depth==0{
            return (self.eval(),self.clone());
        }
        let mut children=vec![];
        let moves=self.get_possible_moves();
        for m in moves{
            let mut cloned=self.clone();
            cloned.move_game(m);
            cloned.check_win(m);
            children.push(cloned);
        }
        let mut score=0;
        if maximising{
            score=std::i32::MIN;
        } else {
            score = std::i32::MAX;
        }
        let mut threads=vec![];
        for c in children{
            threads.push(thread::Builder::new()
                .name(format!("min-{}",depth))
                .spawn(move||c.minimax_parallel(depth-1,!maximising, branch_factor))
                .unwrap());
        }
        let mut results=vec![];
        for t in threads{
            results.push(t.join());
        }
        let mut optimal=self.clone();
        for r in results{
            let (value,pointer)=r.unwrap_or((0, Hex::new(0)));
            if maximising{
                if value>score  {score=value; optimal=pointer}
            } else {
                if value<score  {score=value; optimal=pointer}
            }
        }
        return (score, optimal);
    }
}