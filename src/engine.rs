use crate::board::{Board, ColoredPiece, Color, Piece, Move};
use crate::move_generator::{DesitinationState, MoveGenerator};
use Color::*; 
use Piece::*;
use std::fs;
use std::fs::File;
use std::io::Read;



//breath first search
//teach checkmate, always go to eat king
pub fn get_best_move(board: &Board, c: Color, depth: i8) -> Option<(Move, i32)> {
    if depth == 0 {
        return None;
    }
    let mut moves = Vec::new();
    for m in MoveGenerator::get_moves(c, &board) {
        let e = MoveEvaluator {m, b: &board};
        let (real_score, anticipated_score) = e.evaluate();
        moves.push((m, real_score, anticipated_score));
    }
    moves.sort_by_key(|(_,_,a)| -a);
    moves.truncate(5);

    for (m,real_score,_) in moves.iter_mut() {
        let mut b = board.clone();
        b.play_move(*m);
        let opponents_reponse = get_best_move(&b, c.opposite_color(), depth-1);
        if let Some((opponents_reponse, opponents_score)) = opponents_reponse {
            *real_score -= opponents_score;
        }

    }

    moves.sort_by_key(|(_,r,_)| -r);
    moves.first().map(|(m,r,_)| (*m,*r))
}


pub struct MoveEvaluator<'a> {
    pub m: Move,
    pub b: &'a Board,
}

impl<'a> MoveEvaluator<'a> {
    pub fn evaluate(&self) -> (i32, i32) {
        let mut real_score = 0;
        if let Some(p) = self.get_immediate_capture() {
            real_score += p.get_value();
        }
        //Pinning pieces?

        let mut anticipated_score = real_score;
        for p in self.get_attacked_pieces() {
            anticipated_score += p.get_value() / 3;
        }

        let smaller_center = Board::get_smaller_center();
        let larger_center = Board::get_larger_center();
        if smaller_center.contains(&self.m.dst) {
            anticipated_score += 100;
        } else if larger_center.contains(&self.m.dst) {
            anticipated_score += 50;
        }

        if self.m.is_castle(&self.b) {
            anticipated_score += 1000;
        }

        (real_score, anticipated_score)
    }

    pub fn get_immediate_capture(&self) -> Option<Piece> {
        let p = self.b.get(self.m.src).unwrap();
        if let Some(capture) = self.b.get(self.m.dst) {
            assert!(capture.color != p.color);
            return Some(capture.piece);
        }
        None
    }

    pub fn get_attacked_pieces(&self) -> Vec<Piece> {
        let mut attacked_pieces = Vec::new();
        let p = self.b.get(self.m.src).unwrap();
        let mut b1 = self.b.clone();
        b1.play_move(self.m);
        for m in MoveGenerator::movegen_get_piece_moves(&b1, p, self.m.dst) {
            let e = MoveEvaluator {
                m, 
                b: &b1,
            };
            if let Some(capture) = e.get_immediate_capture() {
                attacked_pieces.push(capture);
            }
        }
        attacked_pieces
    }

    pub fn is_check(&self) -> bool {
        self.get_attacked_pieces().contains(&King)
    }

    pub fn is_legal_move(&self) -> bool {
        self.m.print();
        for m in MoveGenerator::get_all_moves(self.b) {
            m.print();
            if Move::equal(&self.m, &m) {
                return true;
            }
        }
        false
    }
}

pub fn chess_notation_to_move(m: &str) -> Move {
    let first_half = &m[0..2];
    let second_half = &m[2..4];
    let s = Move::parse_move_str(first_half);
    let d = Move::parse_move_str(second_half);
    return Move { src: s, dst: d };
}

pub fn play(board: &Board, mut c: Color) {
    let mut move_number = 0;
    let mut move_history = String::new();
    let mut b = board.clone();
    let book = fs::read_to_string("src/book.txt").expect("bad read");
    let lines = book.lines().collect::<Vec<&str>>();
    //shuffle lines to get random opening
    let m = book_moves(&move_history, &lines, move_number);
    let mut use_book = true;


    for i in 0..100 {
        let m = if use_book {
            book_moves(&move_history, &lines, move_number)
        } else {
            get_best_move(&b, c, 4).map(|(m,_)| m)
        };
        if m.is_none() {
            if use_book {
                use_book = false;
                continue;
            } else {
                break;
            }

        }
        let m = m.unwrap();
        b.play_move(m);
        move_history.push_str(&m.to_move_string());
        move_history.push(' ');
        move_number += 1;
        c = c.opposite_color();
        b.print();
    }
}

pub fn book_moves(move_order: &str, lines: &Vec<&str>, move_number: usize) -> Option<Move> {
    let mut best_length = "";
    dbg!(move_number, move_order);
    if move_number == 0 { 
        return Some(chess_notation_to_move("e2e4"));
    }
    for line in lines {
        //dbg!(line);
        if line.starts_with(move_order) { 
            if line.len() > best_length.len() {
                best_length = line;
            }
        }
    }
    if best_length != "" {
        let moves = best_length.split(' ').collect::<Vec<&str>>();
        return moves.get(move_number).map(|m| chess_notation_to_move(m));
    }

    None
}
