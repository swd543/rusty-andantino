use crate::hex::Hex;
use std::thread;

impl Hex{

    pub fn minimax(&self, depth:i8, maximising:bool)->i32{
        if self.is_game || depth==0{
            println!("winner:{}",self.eval());
            return self.eval();
        }
        println!("Searching at depth {}",depth);
        let mut children=vec![];
        let moves=self.get_possible_moves();
        for m in moves{
            let mut cloned=self.clone();
            cloned.move_game(m);
            cloned.check_win(m);
            children.push(cloned);
        }
        if maximising{
            let mut score=std::i32::MIN;
            for c in children{
                let value=c.minimax(depth-1,false);
                if value>score {score=value}
            }
            return score;
        }
        else{
            let mut score=std::i32::MAX;
            for c in children{
                let value=c.minimax(depth-1,true);
                if value<score {score=value}
            }
            return score;
        }
    }

    pub fn minimax_parallel(&self, depth:i8, maximising:bool)->i32{
        if self.is_game || depth==0{
            return self.eval();
        }
        let mut children=Vec::new();
        let moves=self.get_possible_moves();
        for m in 0..moves.len(){
            children.push(self.clone());
            children[m].move_game(moves[m]);
            children[m].check_win(moves[m]);
        }
        if maximising{
            let mut score=std::i32::MIN;
            let mut threads=vec![];
            for c in children{
                threads.push(thread::Builder::new()
                    .name(format!("min-{}",depth))
                    .spawn(move||c.minimax_parallel(depth-1,false))
                    .unwrap());
            }
            let mut results=vec![];
            for t in threads{
                results.push(t.join());
            }
            for r in results{
                let value=r.unwrap_or_default();
                if value>score {score=value}
            }
            return score;
        } else{
            let mut score=std::i32::MAX;
            let mut threads=vec![];
            for c in children{
                threads.push(thread::Builder::new()
                    .name(format!("max-{}",depth))
                    .spawn(move||c.minimax_parallel(depth-1,true))
                    .unwrap());
            }
            let mut results=vec![];
            for t in threads{
                results.push(t.join());
            }
            for r in results{
                let value=r.unwrap_or_default();
                if value<score {score=value}
            }
            return score;
        }
    }
}