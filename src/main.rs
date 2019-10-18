use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::{io, thread};
use std::process::exit;
use std::sync::Mutex;

use crate::GameState::{BLACK, INVALID, NONE, WHITE};

type GameStateType = i8;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
enum GameState {
    WHITE=8,
    BLACK=7,
    NONE=0,
    INVALID=9
}
impl GameState {
    fn from_game(value: GameStateType) -> GameState {
        match value {
            8 => WHITE,
            7 => BLACK,
            _ => {panic!("Invalid game state provided => {}!",value)}
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Hex{
    size:usize,
    side:usize,
    board:Vec<Vec<GameStateType>>,
    count:u64,
    player:GameState,
    is_game:bool,
    winner:Option<GameState>
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Havannah{
    x:isize,
    y:isize
}

impl Havannah{
    fn add(&self, b:Havannah )->Havannah{
        Havannah{x:self.x+b.x, y:self.y+b.y}
    }

    fn fused_multiply_add(&self, direction:Havannah, steps:isize) ->Havannah{
        Havannah{x:self.x+ direction.x*steps, y:self.y+ direction.y*steps}
    }

    fn is_bounded(&self, between:isize)->bool{
        self.x<between && self.y<between && self.y>=0 && self.x>=0
    }
}

const NORTHEAST:Havannah    =Havannah{x:-1,y:0};
const NORTHWEST:Havannah    =Havannah{x:-1,y:-1};
const SOUTHEAST:Havannah    =Havannah{x:1,y:1};
const SOUTHWEST:Havannah    =Havannah{x:1,y:0};
const EAST:Havannah         =Havannah{x:0,y:1};
const WEST:Havannah         =Havannah{x:0,y:-1};
const NEIGHBOURS:[&Havannah;6] =[&NORTHEAST, &NORTHWEST, &SOUTHEAST, &SOUTHWEST, &EAST, &WEST];
const NEIGHBOUR_OPP:[[&Havannah;2];3]=[[&EAST, &WEST], [&SOUTHWEST, &NORTHEAST], [&SOUTHEAST, &NORTHWEST]];

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
    fn display(&self){
        let mut hasher=DefaultHasher::new();
        self.hash(&mut hasher);
        println!("Game at {:p} ==> hash:{:x} size:{} side:{}, winner:{:?}, is_game:{}, player:{:?}, board:",self,hasher.finish(), self.size, self.side, self.winner, self.is_game, self.player,);
        for i in 0..self.board.len(){
            println!("{}\t{:?}",i, self.board[i])
        }
    }

    #[allow(dead_code)]
    fn hexify(&self){
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

    fn get(&self, location:Havannah) ->&GameStateType{
        &self.board[location.x as usize][location.y as usize]
    }

    fn get_do<F>(&mut self, coordinates:Havannah, f:F) where F:FnOnce(GameStateType)->GameStateType{
        let r=self.board[coordinates.x as usize][coordinates.y as usize].borrow_mut();
        *r=f(*r);
    }

    fn neighbours_do<F>(&mut self, coordinates:Havannah, f:F) where F:Fn(GameStateType)->GameStateType{
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

    fn move_game(&mut self, coordinates:Havannah){
        let p=self.player;
        self.get_do(coordinates, |_x| p as GameStateType);
        self.neighbours_do(coordinates,|x| x+1);
        self.flip_player();
    }

    fn flip_player(&mut self){
        if self.player==WHITE{
            self.player=BLACK;
        }else{
            self.player=WHITE;
        }
    }

    fn check_line(&self, coordinate:Havannah) ->bool{
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
            if start+end>5{
                return true;
            }
        }
        false
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
            if v==player as GameStateType && !visited.contains(location.borrow()) || !(v==BLACK as GameStateType || v==WHITE as GameStateType){
                return self.can_exit_helper(m, player, visited);
            }
        }
        false
    }

    fn check_encloses(&self, coordinate:Havannah)->bool{
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

    fn can_exit(&self, from:Havannah)->bool{
        let s=HashSet::new();
        let player=GameState::from_game(*self.get(from));
        return self.can_exit_helper(from,player,s);
    }

    fn check_win(&mut self, from:Havannah)->bool{
        let player=*self.get(from);
        // TODO Parallelize
        if self.check_line(from) || self.check_encloses(from){
            self.winner=Some(GameState::from_game(player));
            self.is_game=true;
        }
        self.is_game
    }

    fn all_positions_do<Filter,F>(&self, filter:Filter, mut function:F) where Filter:Fn(usize, usize, GameStateType)->bool, F:FnMut(usize, usize, GameStateType){
        for i in 0..self.size{
            for j in 0..self.size{
                if filter(i,j,self.board[i][j]){
                    function(i,j,self.board[i][j]);
                }
            }
        }
    }
    // ----------------------------- AI stuff below -----------------------

    fn eval(&self)->i32{
        if self.winner.is_none() {return 0}
        else if self.winner.unwrap()==BLACK {return -1}
        else if self.winner.unwrap()==WHITE {return 1}
        0
    }

    fn minimax(&self, depth:i8, maximising:bool)->i32{
        if self.is_game || depth==0{
            return self.eval();
        }
        println!("Here {}",depth);
        let mut children=Box::new(Vec::new());
        {
            let moves=self.get_possible_moves();
            println!("Possible moves {:?}",moves);
            for m in 0..moves.len(){
                children.push(self.clone());
                children[m].move_game(moves[m]);
                children[m].check_win(moves[m]);
            }
        }
        if maximising{
            let mut score=std::i32::MIN;
            for c in 0..children.len(){
                let value=children[c].minimax(depth-1,false);
                if value>score {score=value}
            }
            return score;
        }
        else{
            let mut score=std::i32::MAX;
            for c in 0..children.len(){
                let value=children[c].minimax(depth-1,true);
                if value<score {score=value}
            }
            return score;
        }
    }

    fn get_possible_moves(&self)->Vec<Havannah>{
        let mut moves =Vec::new();
        self.all_positions_do(|_i,_j,v|{v>1 && v<BLACK as GameStateType}, |i,j,_k|{
            moves.push(Havannah{x:i as isize, y:j as isize});
        });
        moves.to_vec()
    }
}

fn is_occupied(location:&GameStateType) ->bool{
    *location==WHITE as GameStateType || *location==BLACK as GameStateType || *location==INVALID as GameStateType
}

fn run(){
    let mut game =Box::new(Hex::new(10));
    let start=Havannah{x:game.side as isize,y:game.side as isize};
    game.move_game(start);
    game.move_game(start.add(WEST));
    loop{
        game.display();
        {
            println!("{}",game.minimax(4, game.player==WHITE));
        }
        let moves=game.get_possible_moves();
        println!("{:?}",moves);
        {
            loop{
                println!("Enter yo option, mate : ");
                let mut x = String::with_capacity(3);
                io::stdin().read_line(&mut x).expect("Error reading input");
                let x:usize = x.trim().parse().expect("Error parsing number");
                if x>=moves.len(){
                    println!("Try again.");
                    continue;
                }
                let k=moves[x];
                println!("You entered {} ---> {:?}",x,k);
                game.move_game(k);
                if game.check_win(k){
                    game.hexify();
                    println!("{:?} won!", game.winner);
                    exit(0);
                } else {
                    break;
                }
            }
        }
    }
}

fn main() {
    const STACK_SIZE:usize=1024*1024*2;
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
    run();
}
// Test cases hooked here
mod hex_test;