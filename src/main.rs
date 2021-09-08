#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
mod board;
mod move_generator;
mod tests;
mod engine;

use crate::board::{Board, ColoredPiece, Color, Piece, Move};
use engine::MoveEvaluator;
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
        dst: [7,2],
    };
    let b1 = Board::from_fen("r1bqkb1r/ppp2ppp/2np1n2/4p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 0 1");
    let b2 = Board::from_fen("rn1qkb1r/pppb1p1p/3p1np1/4p1B1/4P3/2NP4/PPPQ1PPP/R3KBNR w KQkq - 0 1");
    b2.print();
    let p = b1.get([7,7]).expect("rook not found");
    for m in MoveGenerator::get_all_moves(&b2) {
        let mut b_copy = b1.clone();
        //m.print();
        if Move::equal(&m, &m1) {
            b_copy.play_move(m);
            b_copy.print();
        }
    }
    uci_main();
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
                Board::print_new_game();
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
        b = Board::from_fen(fen);
    }
    else {
        println!("Inalid pos");
        return;
    }
    for m in moves {
        let e = MoveEvaluator {
            m: Move::chess_notation_to_move(*m),
            b: &b,
        };
        if e.is_legal_move() {
            b.play_move(Move::chess_notation_to_move(m));
        }    
        else {
            b.print();
            println!("Illegal move");
            return;
        }
    }
    b.print();
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