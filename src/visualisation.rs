extern crate termcolor;
use crate::game_state_type::GameStateType;
use crate::hex::Hex;
use std::io::Write;
use termcolor::{StandardStream, ColorChoice, ColorSpec, Color, WriteColor};
use crate::havannah::Havannah;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use crate::game_state_type::GameState::{INVALID, BLACK, WHITE};

pub fn visualize(a:Vec<Vec<GameStateType>>){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    println!("Object at {:p}",&a);
    for i in a{
        for j in i{
            let char=j as i16;
            let color=(char%256-128) as u8;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(color,color,color))));
            write!(&mut stdout, "\u{2588}");
        }
        writeln!(&mut stdout);
    }
    stdout.reset();
}

pub fn visualize_with_values(a:&Vec<Vec<GameStateType>>){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    println!("Object at {:p} looks like this",a);
    for i in a{
        for j in i{
            let char=*j as i16;
            let color=(char%256-128) as u8;
            stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(color,color,color))).set_intense(true).set_fg(Some(Color::Green)));
            write!(&mut stdout, "{:04} ",j);
        }
        stdout.reset();
        writeln!(&mut stdout);
    }
    stdout.reset();
}

pub fn calibrate(){
    let white=vec![vec![127;5];2];
    let black=vec![vec![-128;5];2];
    let center=vec![vec![0;5];2];
    visualize(white);
    visualize(black);
    visualize(center)
}

impl Hex{
    pub fn display(&self){
        let mut hasher=DefaultHasher::new();
        self.hash(&mut hasher);
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        println!("Game at {:p} ==> hash:{:x} size:{} side:{}, winner:{:?}, is_game:{}, player:{:?}, board:",self,hasher.finish(), self.size, self.side, self.winner, self.is_game, self.player,);
        visualize_with_values(self.board.as_ref());
    }

    pub fn hexify(&self){
        let mut hasher=DefaultHasher::new();
        self.hash(&mut hasher);
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        println!("Game at {:p} ==> hash:{:x} size:{} side:{}, winner:{:?}, is_game:{}, player:{:?}, count:{}, board:",self,hasher.finish(), self.size, self.side, self.winner, self.is_game, self.player,self.count);
        for i in 0..self.board.len(){
            let mut vec=vec![];
            for j in 0..self.board.len(){
                if self.board[i][j]==INVALID as GameStateType{
                    continue;
                }
                vec.push(Havannah{x:i as isize,y:j as isize})
            }
            for _i in 0..self.size- vec.len(){
                write!(&mut stdout, "  ");
            }
            for v in vec {
                if *self.get(v)== BLACK as i8 {
                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(0,0,0))).set_intense(false).set_fg(Some(Color::Red)));
                } else if *self.get(v)==WHITE as i8{
                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(255,255,255))).set_intense(false).set_fg(Some(Color::Red)));
                } else{
                    let mut color=((*self.get(v) as i16 %8)*16-130) as u8;
                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(color,color,color))).set_intense(false).set_fg(Some(Color::Red)));
                }
                write!(&mut stdout, " {}{:02}", std::char::from_u32(('A' as isize + v.x) as u32).unwrap(), v.y);
            }
            stdout.reset();
            writeln!(&mut stdout);
        }

//        for i in self.board.clone(){
//            let mut vec=vec![];
//            for j in i{
//                if j==INVALID as GameStateType{
//                    continue;
//                }
//                vec.push(j as i16);
//            }
//            for _i in 0..self.size- vec.len(){
//                write!(&mut stdout, "   ");
//            }
//            for v in vec {
//                if v==BLACK as i16{
//                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(0,0,0))).set_intense(false).set_fg(Some(Color::Red)));
//                } else if v==WHITE as i16{
//                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(255,255,255))).set_intense(false).set_fg(Some(Color::Red)));
//                } else{
//                    let mut color=((v%8)*16-130) as u8;
//                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(color,color,color))).set_intense(false).set_fg(Some(Color::Red)));
//                }
//                write!(&mut stdout, " {:02} {:02} {:02} ",v, );
//            }
//            stdout.reset();
//            writeln!(&mut stdout);
//        }
//        stdout.reset();
    }
}