#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
mod board;
mod move_generator;

use crate::board::{Board, ColoredPiece, Color, Piece, Move};
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
    let m1 = Move {
        src: [7,4],
        dst: [7,6],
    };
    let b1 = Board::from_fen(String::from("r1bqkb1r/ppp2ppp/2np1n2/4p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 0 1"));
    b1.print();
    let p = b1.get([7,7]).expect("rook not found");
    dbg!(p);
    for m in MoveGenerator::get_all_moves(&b1) {
        let mut b2 = b1.clone();
        //m.print();
        if Move::equal(&m, &m1) {
            b2 = b2.play_move(m);
            b2.print();
        }
    }
    //uci_main();
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
       let board4 = 
       "♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖\n\
        ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟\n\
        ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜\n\
        ";
    let b1 = Board::from_str(board1, White);
    let b2 = Board::from_str(board2, White);
    let b3 = Board::from_str(board3, White);
    //let b4 = Board::from_str_piece(board4, White);
    let b4 = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    b4.print();
    dbg!(&b1);
    assert_eq!(b1.arr[0][6], Some(ColoredPiece{color: Black, piece: King}));
    println!("Board 1: ");
    b1.get_advantage_print(White);
    println!("");
    println!("Board 2: ");
    b2.get_advantage_print(White);
    println!("");
    b1.print();
    let best_move = get_best_move(&b1, Black, 4);
    Move::print_option(best_move);
    println!("\n");
    let fen = b2.to_fen();
    println!("{}", fen);
   // let best_board = get_best_board(&b1, White, 2);
   // best_board.print();
    let from_fen = Board::from_fen(fen);
    from_fen.print();
}


pub fn uci_main() {
    loop {
        let s = get_user_input().expect("EOF");
        let cmd = s.split_whitespace().collect::<Vec<_>>();
        match *cmd {
            ["uci"] => {
                println!("uciok");
            }
            ["isready"] => {
                println!("readyok");
            }
            ["ucinewgame"] => {
                print_new_game();
            }
            ["position", boardsetup, "moves", ref moves @ ..] =>  {
                cmd_position(boardsetup, moves);
                println!("position command: boardsetup={}, moves={:?}", boardsetup, moves);
            }
            ["quit"] => {
                return;
            }
            _ => {
                println!("Command not recognized: {:?}", cmd);
            }
        }
    }
}

pub fn cmd_position(boardsetup: &str, moves: &[&str]) {
    let mut b = Board::new();
    if boardsetup == "startpos" {
        b = b;
    }
    else if &boardsetup[0..3] == "fen" {
        let fen = &boardsetup[5..boardsetup.len()-1];
        b = Board::from_fen(String::from(fen));
    }
    else {
        println!("Inalid pos");
        return;
    }
    for m in moves {
        if Move::is_legal_move(&Move::chess_notation_to_move(*m),&b) {
            b = b.play_move(Move::chess_notation_to_move(m));
        }    
        else {
            b.print();
            println!("Illegal move");
            return;
        }
    }
    b.print();
}

struct UCI {
}

impl UCI {
    fn cmd_position(boardsetup: &str, moves: &[&str]) {
    }
}


pub fn print_new_game() {
    Board::new().print();
}

//breath first search. loop through depth 0 and find some promising moves first 
fn get_best_move(board: &Board, c: Color, depth: i8) -> Option<Move> {
    if depth == 0 {
        return None;
    }
    let mut best_score = 0;
    let mut best_move = None;
    for m in MoveGenerator::get_moves(c, &board) {
        let mut b = board.clone();
        b = b.play_move(m);
        let opponents_reponse = get_best_move(&b, c.opposite_color(), depth-1);
        if let Some(opponents_reponse) = opponents_reponse{
            b = b.play_move(opponents_reponse);
        }
        let new_score = b.get_score(c);
        if new_score > best_score {
            best_move = Some(m);
            best_score = new_score;
        }
    }
    best_move
}

fn get_user_input() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading stdin");
    if input.len() == 0 {
        None
    } else {
        if input.ends_with('\n') {
            input.pop();
        }
        Some(input)
    }
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
