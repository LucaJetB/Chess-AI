#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
mod board;
mod move_generator;

use crate::board::{Board, ColoredPiece, Color, Piece};
use move_generator::*;
use Color::*; 
use Piece::*;
use ::std::*;
use std::io::{self, Read};
fn main() {
    let board1= 
    "r n b q k b n r\n\
     p p p p p p p p\n\
     . . . . . . . .\n\
     . . . . . . . .\n\
     . . . . . . . .\n\
     . . . . . . . .\n\
     P P P P P P P P\n\
     R N B Q K B N R\n\
     ";
     let white_king = ColoredPiece {
        color: White,
        piece: King,
    };
    let b1 = Board::from_str(board1);
    let mut buff = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buff).expect("error");
    println!("{}",buff); 
    let c = '8';
    let num = c as i32 - 0x30;
    println!("{}", num);
    //play(b1); 
}
// TODO use UCI/any protocal to interact with engine and get an UI
// write a to_FEN func for boards use a few tests, write a from_FEN func as well
#[test] 
fn test_basic1() {
    let board1 = 
    ". . . . . . k .\n\
     . . . . . p p p\n\
     . . . . . . b .\n\
     . . . . n . . .\n\
     . . Q . . . . .\n\
     . . . . . . . .\n\
     P . . B . P . .\n\
     . . . . . . K .\n\
     ";
     let board2 = 
     "r n b q k b n r\n\
      p p p p p p p p\n\
      . . . . . . . .\n\
      . . . . . . . .\n\
      . . . . . . . .\n\
      . . . . . . . .\n\
      P P P P P P P P\n\
      R N B Q K B N R\n\
      ";
      let board3 = 
      ". . . . . . . .\n\
       . . . . . . . .\n\
       . . . . . . . .\n\
       . . . . . . . .\n\
       . . . q . b . .\n\
       . n . . P . . .\n\
       P . . . . . . .\n\
       . . . . . . . .\n\
       ";
    let b1 = Board::from_str(board1);
    let b2 = Board::from_str(board2);
    let b3 = Board::from_str(board3);
    dbg!(&b1);
    assert_eq!(b1.arr[0][6], Some(ColoredPiece{color: Black, piece: King}));
    println!("Board 1: ");
    b1.get_advantage_print(White);
    println!("");
    println!("Board 2: ");
    b2.get_advantage_print(White);
    println!("");
    b1.print();
    let best_board = get_next_move(&b1, Black);
    best_board.print();
    println!("\n");
    let fen = b2.to_fen();
    println!("{}", fen);
   // let best_board = get_best_board(&b1, White, 2);
   // best_board.print();
    let from_fen = Board::from_fen(fen);
    from_fen.print();
}

fn get_next_move(board: &Board, c: Color) -> Board {
    let mut best_score = board.get_advantage(c);
    let mut best_board = board.clone();
    for m in MoveGenerator::get_moves(c, &board) {
        let mut b = board.clone();
        let p = b.get(m.src);
        b.set(m.dst, p);
        b.set(m.src, None);
        println!("{:?}", p);
        println!("{:?}", m.src);
        println!("{:?}", m.dst);
        b.print();
        println!("");

        if b.get_advantage(c) > best_score {
            best_score = b.get_advantage(c);
            best_board = b;
        }
    }
    best_board
}

fn get_best_board(board: &Board, c: Color, depth: i8) -> Board {
    let mut best_board = board.clone();
    if depth == 0 {
        return best_board
    }
    for m in MoveGenerator::get_moves(c, board) {
        let mut b = board.clone();
        let p = b.get(m.src);
        b.set(m.dst, p);
        b.set(m.src, None);
        let b_copy = b.clone();
        b = get_next_move(&b, c.opposite_color());
        let test_board = get_best_board(&b, c, depth-1);
        if test_board.get_advantage(c) > best_board.get_advantage(c) {
            best_board = b_copy;
        }
    }
    best_board
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading stdin");
    if input.ends_with('\n') {
        input.pop();
    }
    input
}
/* 
fn play(mut board: Board) {
    let white_king = ColoredPiece {
        color: White,
        piece: King,
    };
    let black_king = ColoredPiece {
        color: Black,
        piece: King,
    };
    let mut input = String::new();
    let mut c: Color;
    board.print();
    println!("Go first? y/n");
    input = get_user_input();
    if input == "y" {
        while board.find_piece(white_king) && board.find_piece(black_king) {
            println!("Move: ");
            input = get_user_input();

            //e4
            let src = get_move(&white_src);
            println!("Move to: ");
            io::stdin().read_line(&mut input).expect("Error");
            let dst = get_move(&white_dst);
            let player_move = Move {
                src,
                dst,
            };
            if !is_valid_move(&player_move, &board, White) {
                println!("Error invalid move");
                continue;
            }
            else {
                let p = board.get(player_move.src);
                board.set(player_move.dst, p);
                board.set(player_move.src, None);
                board.print();
            }
            board = get_best_board_depth_3(board, Black);
            board.print();
        }
    }
    else {
        board = get_best_board_depth_3(board, Black);
        while board.find_piece(white_king) && board.find_piece(black_king) {
            println!("Piece to move: ");
            io::stdin().read_line(&mut input).expect("Error");
            let src = get_move(&input);
            println!("Move to: ");
            io::stdin().read_line(&mut input).expect("Error");
            let dst = get_move(&input);
            let player_move = Move {
                src,
                dst,
            };
            if !is_valid_move(&player_move, &board, Black) {
                println!("Error invalid move");
                continue;
            }
            else {
                let p = board.get(player_move.src);
                board.set(player_move.dst, p);
                board.set(player_move.src, None);
                board.print();
            }
            board = get_best_board_depth_3(board, Black);
            board.print();
        }
    }       
}

fn is_valid_move(m_to_check: &Move, b: &Board, c: Color) -> bool {
    for m in MoveGenerator::get_moves(c, &b) {
        if m.src == m_to_check.src {
            if m.dst == m_to_check.dst {
                true;
            }
        }
    }
    false
}
*/
