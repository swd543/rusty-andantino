use crate::game_state_type::{GameStateType, GameState};
use crate::havannah::Havannah;

pub fn filter<F>(input:&mut Vec<Vec<GameStateType>>, f:F) where F:Fn(GameStateType)->bool{
    for i in input{
        for mut j in i{
            if f(*j){
                *j=GameStateType::default();
            }
        }
    }
}

pub fn conv(input:&Vec<Vec<GameStateType>>, kernel:&Vec<Vec<GameStateType>>)->Vec<Vec<GameStateType>>{
    let k_center_x=kernel.len()/2;
    let k_center_y=kernel[0].len()/2;
    let mut out=input.clone();

    for i in 0..input.len(){
        for j in 0..input.len(){
            for m in 1..kernel.len(){
                let mm=kernel.len()-1-m;
                for n in 1..kernel[0].len(){
                    let nn=kernel.len()-1-n;
                    let ii = i + (k_center_x - mm);
                    let jj = j + (k_center_y - nn);
                    if ii >= 0 && ii < input.len() && jj < input[0].len() {
                        out[i][j] += input[ii][jj] * kernel[mm][nn];
                    }
                }
            }
        }
    }
    out
}

pub fn conv_arr(input:&Vec<Vec<GameStateType>>, kernel:&[[GameStateType;5];5])->Vec<Vec<GameStateType>>{
    let k_center_x=kernel.len()+1/2;
    let k_center_y=kernel[0].len()+1/2;
    let mut out=input.clone();

    for i in 0..input.len(){
        for j in 0..input.len(){
            for m in 1..kernel.len(){
                let mm=kernel.len()-1-m;
                for n in 1..kernel[0].len(){
                    let nn=kernel.len()-1-n;
                    let ii = i + (k_center_x - mm);
                    let jj = j + (k_center_y - nn);
                    if ii >= 0 && ii < input.len() && jj < input[0].len() {
                        out[i][j] += input[ii][jj] * kernel[mm][nn];
                    }
                }
            }
        }
    }
    out
}

pub fn sum(input:&Vec<Vec<GameStateType>>)->isize{
    let mut out= 0;
    for i in input{
        for j in i{
            out+=*j as isize;
        }
    }
    out
}
//
//pub fn diagonal_kernel(size:i8)->Vec<Vec<GameStateType>>{
//    let mut kernel=vec![];
//    for i in 0..size{
//        kernel.push(vec![]);
//        for j in 0..size{
//            kernel[i].push(-1);
//        }
//    }
//    let mut start=Havannah{x:0,y:0};
//    kernel
//}