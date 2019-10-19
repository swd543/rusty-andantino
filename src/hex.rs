use std::clone::Clone;
use std::hash::{Hash, Hasher};
use crate::havannah::{Havannah, NEIGHBOURS, NEIGHBOUR_OPP};
use std::collections::hash_map::DefaultHasher;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use std::thread;
use crate::game_state_type::{GameStateType, GameState};
use crate::game_state_type::GameState::{NONE, INVALID, WHITE, BLACK};

pub const STARTER:GameState=BLACK;

#[derive(Debug, Clone, Hash)]
pub struct Hex{
    pub size:usize,
    pub side:usize,
    pub board:Vec<Vec<GameStateType>>,
    pub count:u64,
    pub player:GameState,
    pub is_game:bool,
    pub winner:Option<GameState>
}

impl Hex{
    pub fn new(side:usize)->Hex{
        println!("Initializing new game...");
        let mut x=Hex{
            side,
            size:2*side+1,
            board:[].to_vec(),
            count:0,
            player:BLACK,
            is_game:false,
            winner:None
        };
        for _ in 0..x.size{
            let mut v=Vec::new();
            for _ in 0..x.size{
                v.push(INVALID as GameStateType);
            }
            x.board.push(v.to_vec());
        }
        for i in 0 .. x.side+1{
            for j in 0 .. x.side+i+1 {
                x.board[i][j]=NONE as GameStateType;
                x.board[x.size-i-1][x.size-j-1]=NONE as GameStateType;
            }
        }
        println!("Initialized new game!");
        x
    }

    #[allow(dead_code)]
    pub fn display(&self){
        let mut hasher=DefaultHasher::new();
        self.hash(&mut hasher);
        println!("Game at {:p} ==> hash:{:x} size:{} side:{}, winner:{:?}, is_game:{}, player:{:?}, board:",self,hasher.finish(), self.size, self.side, self.winner, self.is_game, self.player,);
        for i in 0..self.board.len(){
            println!("{}\t{:?}",i, self.board[i])
        }
    }


    #[allow(dead_code)]
    pub fn hexify(&self){
        let mut hasher=DefaultHasher::new();
        self.hash(&mut hasher);
        println!("Game at {:p} ==> hash:{:x} size:{} side:{}, winner:{:?}, is_game:{}, player:{:?}, board:",self,hasher.finish(), self.size, self.side, self.winner, self.is_game, self.player,);
        for i in 0..=self.side{
            for _j in 0..self.side-i{
                print!(" ");
            }
            let slice=self.board[i][0..self.side+i+1].borrow();
            println!("{:?}",slice);
        }
        for i in (self.side+1..=self.size-1).rev(){
            for _j in 0..self.size-i{
                print!("  ");
            }
            let slice=self.board[self.size-i+self.side][self.size-i .. self.size].borrow();
            println!("{:?}",slice);
        }
    }

    pub fn get(&self, location:Havannah) ->&GameStateType{
        &self.board[location.x as usize][location.y as usize]
    }

    pub fn get_do<F>(&mut self, coordinates:Havannah, f:F) where F:FnOnce(GameStateType)->GameStateType{
        let r=self.board[coordinates.x as usize][coordinates.y as usize].borrow_mut();
        *r=f(*r);
    }

    pub fn neighbours_do<F>(&mut self, coordinates:Havannah, f:F) where F:Fn(GameStateType)->GameStateType{
        for n in NEIGHBOURS.iter(){
            let neighbour=coordinates.add(**n);
            if neighbour.is_bounded(self.size as isize){
                let r=self.board[neighbour.x as usize][neighbour.y as usize].borrow_mut();
                if !is_occupied(r){
                    *r=f(*r);
                }
            }
        }
    }

    pub fn move_game(&mut self, coordinates:Havannah){
        let p=self.player;
        self.get_do(coordinates, |_x| p as GameStateType);
        self.neighbours_do(coordinates,|x| x+1);
        self.flip_player();
    }

    pub fn flip_player(&mut self){
        if self.player==WHITE{
            self.player=BLACK;
        }else{
            self.player=WHITE;
        }
    }

    pub fn check_line(&self, coordinate:Havannah) ->(bool, GameState, isize){
        let player=self.get(coordinate);
        for pair in NEIGHBOUR_OPP.iter(){
            let mut start=0;
            let mut end =0;

            let k=coordinate.clone();
            // TODO while can be optimized
            while k.fused_multiply_add(*pair[0],start).is_bounded(self.size as isize) &&
                self.get(k.fused_multiply_add(*pair[0],start))==player{
                start+=1;
            }
            let k=coordinate.clone();
            while k.fused_multiply_add(*pair[1],end).is_bounded(self.size as isize) &&
                self.get( k.fused_multiply_add(*pair[1],end))==player{
                end+=1;
            }
            println!("{} -> {}",player,start+end);
            let run_length=start+end;
            if run_length>5{
                return (true,GameState::from_game(*player),run_length);
            }
        }
        (false,NONE,0)
    }

    fn can_exit_helper(&self, location:Havannah, player: GameState, mut visited: HashSet<Havannah>) ->bool{
        if !location.is_bounded(self.size as isize){
            return true;
        }
        visited.insert(location);
        for n in NEIGHBOURS.iter(){
            let m=location.add(**n);
            if !m.is_bounded(self.size as isize){
                return true;
            }
            let v=*self.get(m);
            if v==INVALID as GameStateType{
                return true;
            }
            if (v==player as GameStateType || !(v==BLACK as GameStateType || v==WHITE as GameStateType)) && !visited.contains(m.borrow()) {
                return self.can_exit_helper(m, player, visited);
            }
        }
        false
    }

    pub fn check_encloses(&self, coordinate:Havannah)->bool{
        // TODO optimize this function
        let player=*self.get(coordinate);
        for i in 0..self.size{
            for j in 0..self.size{
                let t=self.board[i][j];
                if t==player{
                    continue;
                }
                if t==BLACK as GameStateType||t==WHITE as GameStateType{
                    return !self.can_exit(Havannah{x:i as isize,y:j as isize});
                }
            }
        }
        false
    }

    pub fn can_exit(&self, from:Havannah)->bool{
        let s=HashSet::new();
        let player=GameState::from_game(*self.get(from));
        return self.can_exit_helper(from,player,s);
    }

    pub fn check_win(&mut self, from:Havannah)->bool{
        let player=*self.get(from);
        // TODO Parallelize
        if self.check_line(from).0 || self.check_encloses(from){
            self.winner=Some(GameState::from_game(player));
            self.is_game=true;
        }
        self.is_game
    }

    pub fn all_positions_do<Filter,F>(&self, filter:Filter, mut function:F) where Filter:Fn(usize, usize, GameStateType)->bool, F:FnMut(usize, usize, GameStateType) {
        for i in 0..self.size {
            for j in 0..self.size {
                if filter(i, j, self.board[i][j]) {
                    function(i, j, self.board[i][j]);
                }
            }
        }
    }

    pub fn get_possible_moves(&self)->Vec<Havannah>{
        let mut moves =vec![];
        self.all_positions_do(|_i,_j,v|{v>1 && v<BLACK as GameStateType || v==1 && self.count==1}, |i,j,_k|{
            moves.push(Havannah{x:i as isize, y:j as isize});
        });
        moves.to_vec()
    }
}

fn is_occupied(location:&GameStateType) ->bool{
    *location==WHITE as GameStateType || *location==BLACK as GameStateType || *location==INVALID as GameStateType
}