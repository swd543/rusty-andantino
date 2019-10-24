use havannah::Havannah;

use crate::game_state_type::GameState::*;
use crate::havannah::WEST;
use crate::hex::{Hex, is_occupied};
use std::process::exit;
use std::io;
use crate::convolution::{filter, conv_arr, sum};
use crate::game_state_type::GameStateType;
use crate::evaluation::KERNEL;
use crate::visualisation::visualize_with_values;

mod havannah;
mod hex;
mod game_state_type;
mod evaluation;
mod minimax;
mod visualisation;
mod convolution;
mod negamax;

fn play_ai(game:&mut Hex){
//    let ai_moves=game.minimax_parallel(4, game.player==WHITE,2).1.move_chain;
    let ai=game.negamax_parallel(4, 3);
//    let ai=game.minimax_parallel(4, game.player==BLACK,1);
    println!("Score possible : {}",ai.0);
    let ai_moves=ai.1.move_chain;
    println!("{:?}",ai_moves);
    for m in ai_moves{
        if is_occupied(game.get(m)){continue}
        game.move_game(m);
        println!("AI plays {:?}",m);
        if game.check_win(m){
            game.hexify();
            println!("{:?} won!", game.winner);
            exit(0);
        } else {
            break;
        }
        break;
    }
}

fn play_human(game:&mut Hex){
    let moves=game.get_possible_moves();
    for i in 0..moves.len(){
        print!("{}:{:?} ",i,moves[i]);
    }
    println!();
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
        println!("Exisitng move chain {:?}",game.move_chain);
        if game.check_win(k){
            game.hexify();
            println!("{:?} won!", game.winner);
            exit(0);
        } else {
            break;
        }
    }
}

fn hvp(){
    let mut game =Hex::new(10);
    let start=Havannah{x:game.side as isize,y:game.side as isize};
    game.move_game(start);
    loop{
        game.hexify();
        play_ai(&mut game);
        game.hexify();
        println!("Score is {}",game.eval());
        play_human(&mut game);
    }
}

fn pvp(){
    let mut game =Hex::new(10);
    let start=Havannah{x:game.side as isize,y:game.side as isize};
    game.move_game(start);
    loop{
        game.hexify();
        play_ai(&mut game);
        game.hexify();
        println!("Score is {}",game.eval());
        play_ai(&mut game);
    }
}

fn main() {
    pvp();
}

mod hex_test;
mod convolution_test;
mod hex_util;