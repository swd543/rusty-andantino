use super::*;

#[test]
fn test_init() {
    let x=Hex::new(1);
    let mut y=vec![vec![NONE as GameStateType;3];3];
    y[0][2]=INVALID as GameStateType;
    y[2][0]=INVALID as GameStateType;
    assert_eq!(x.board,y);
    assert_eq!(x.count,0);
    assert_eq!(x.size,3);
    assert_eq!(x.side,1);
}

#[test]
fn test_copy() {
    let x=Hex::new(1);
    let mut y=x.clone();
    assert_eq!(x.board,y.board);
    y.move_game(Havannah{x:0,y:0});
    assert_ne!(x.board,y.board);
}

//#[test]
//fn test_hexify() {
//    let x=Hex::new(1);
//    x.hexify();
//}

#[test]
fn test_direction_sanity() {
    let zero = Havannah{x:0,y:0};
    assert_eq!(NORTHWEST.add(SOUTHEAST), zero);
    assert_eq!(EAST.add(WEST), zero);
    assert_eq!(SOUTHWEST.add(NORTHEAST), zero);
    for i in NEIGHBOUR_OPP.iter(){
        assert_eq!((*i)[0].add(*i[1]), zero);
    }
}

#[test]
fn test_havannah_add() {
    let x=Havannah{x:2,y:2};
    assert_eq!(x.add(WEST),Havannah{x:2,y:1});
    assert_eq!(x.add(EAST),Havannah{x:2,y:3});
}

#[test]
fn test_havannah_fused_multiply_add() {
    assert_eq!(EAST.fused_multiply_add(WEST,10), Havannah{x:0,y:-9});
}

#[test]
fn test_check_line() {
    let mut game=Hex::new(10);
    for i in 0..5{
        game.board[i][i]=BLACK as GameStateType;
    }
    assert!(game.check_line(Havannah{x:0,y:0}));
    assert!(game.check_line(Havannah{x:2,y:2}));
    assert!(game.check_line(Havannah{x:4,y:4}));

    game.board[4][4]=NONE as GameStateType;
    assert!(!game.check_line(Havannah{x:0,y:0}));
    assert!(!game.check_line(Havannah{x:2,y:2}));
    assert!(!game.check_line(Havannah{x:3,y:3}));

    for i in 0..5{
        game.board[game.side-1][i]=WHITE as GameStateType;
    }
    assert!(game.check_line(Havannah{x:9,y:0}));
    game.board[9][4]=BLACK as GameStateType;
    assert!(!game.check_line(Havannah{x:9,y:0}));
//    game.hexify();
}

#[test]
fn test_move() {
    let mut game=Hex::new(10);
    let start=Havannah{x:game.side as isize, y:game.side as isize};
    game.move_game(start);
    assert_eq!(*game.get(start), BLACK as GameStateType);
    game.neighbours_do(start, |x|{
        assert_eq!(x, 1);
        x
    });
    game.move_game(start.add(EAST));
    assert_eq!(*game.get(start.add(NORTHEAST)),2);
    assert_eq!(*game.get(start.add(SOUTHEAST)),2);
}

#[test]
fn test_can_exit() {
    let mut game=Hex::new(10);
    let start=Havannah{x:game.side as isize, y:game.side as isize};
    game.move_game(start);
    assert!(game.can_exit(start));
    let player=game.player as GameStateType;
    game.neighbours_do(start,|_x|{
        player
    });
    assert!(!game.can_exit(start));
    let t=start.add(SOUTHEAST);
    game.board[t.x as usize][t.y as usize]=NONE as GameStateType;
    assert!(game.can_exit(start));
}

#[test]
fn test_can_exit2() {
    let mut game=Hex::new(10);
    let start=Havannah{x:game.side as isize, y:game.side as isize};
    for i in 0..4{
        let y=start.fused_multiply_add(NORTHEAST, i);
        game.player=BLACK;
        game.move_game(y);
    }
    for i in 0..4{
        let y=start.fused_multiply_add(NORTHEAST, i);
        game.neighbours_do(y, |x|{
            WHITE as GameStateType
        });
    }
    assert!(!game.can_exit(start));
}

#[test]
#[ignore]
fn test_recursive_overflow() {
    let mut game=Hex::new(3000);
    let move1=Havannah{x:game.side as isize, y:game.side as isize};
    game.move_game(move1);
    game.neighbours_do(move1,|_x|WHITE as GameStateType);
    game.check_win(move1);
}

#[test]
#[ignore]
fn test_minimax() {
    let mut game=Hex::new(10);
    let mut move1=Havannah{x:game.side as isize, y:game.side as isize};
    let move_sequence =[WEST,NORTHEAST];
    game.move_game(move1);
    for i in 0..2{
        for j in 0..move_sequence.len(){
            move1=move1.add(move_sequence[j]);
            game.move_game(move1);
        }
    }
    game.hexify();
    game.minimax(3,true);
}

#[test]
fn test_thread() {
    let mut children = vec![];

    for i in 0..10 {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
            return i;
        }));
    }
    let mut results=vec![];
    for child in children {
        results.push(child.join());
    }
    println!("{:?}",results);
}