use crate::hex::Hex;
use crate::{convolution, visualisation};
use std::borrow::Borrow;
use crate::game_state_type::GameState::{INVALID, WHITE, BLACK};
use crate::game_state_type::GameStateType;
use crate::havannah::{EAST, SOUTHEAST, Havannah, SOUTHWEST};

#[ignore]
#[test]
fn test_conv_visualize(){
    let arr=Hex::new(10).board;
    let kernel=
        [
            [1,-1,-1].to_vec(),
            [-1,1,-1].to_vec(),
            [-1,-1,1].to_vec()
        ].to_vec();
    let out=convolution::conv(&arr,&kernel);
    visualisation::calibrate();
    visualisation::visualize(out.clone());
    visualisation::visualize(arr.clone());
    visualisation::visualize_with_values(out.borrow());
    visualisation::visualize_with_values(arr.borrow());
    println!("{}",convolution::sum(&out));
}

#[test]
#[ignore]
fn test_filter(){
    let mut arr=Hex::new(10).board;
    convolution::filter(arr.as_mut(),|x|x==INVALID as GameStateType);
    visualisation::visualize_with_values(&arr);
    let kernel=
        [
            [1,-1,-1].to_vec(),
            [-1,1,-1].to_vec(),
            [-1,-1,1].to_vec()
        ].to_vec();
    let k=convolution::conv(&arr,&kernel);
    println!("{}",convolution::sum(&k));
}

#[test]
#[ignore]
fn test_conv2(){
    let mut game=Hex::new(10);
    game.move_repeated(1, [EAST,SOUTHWEST].to_vec().borrow(), Havannah{x:3,y:3});
    game.display();
    let mut arr=game.board.clone();
    convolution::filter(arr.as_mut(),|x|x==INVALID as GameStateType || x!=WHITE as GameStateType && x!=BLACK as GameStateType);
    visualisation::visualize_with_values(&arr);
    let kernel=
        [
            [1,-1,1].to_vec(),
            [-1,1,-1].to_vec(),
            [1,-1,1].to_vec()
        ].to_vec();
    let k=convolution::conv(&arr,&kernel);
    println!("{}",convolution::sum(&k));
    game.hexify();
}