use havannah::Havannah;

use crate::game_state_type::GameState::*;
use crate::havannah::WEST;
use crate::hex::Hex;
use std::process::exit;
use std::io;

mod havannah;
mod hex;
mod game_state_type;
mod hex_test;
mod evaluation;
mod minimax;

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
    run();
}
