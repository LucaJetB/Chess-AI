use crate::board::{Board, Color, Piece, Move};
use crate::move_generator::{MoveGenerator, DestinationState};
use Piece::*;
use Color::*;
//use core::num::dec2flt::rawfp::RawFloat;
use std::fs;
use std::u64::MAX;
use std::cmp;




//breath first search
//teach checkmate, always go to eat king
//if piece gets captured, all points should go away
/// Sometimes c is != board.color, in this case it's used to find opponents best move.
pub fn get_best_move(board: &Board, c: Color, depth: i8) -> Option<(Move, i32)> {
    if depth == 0 {
        return None;
    }
    let mut moves = Vec::new();
    for m in MoveGenerator::get_moves(&board, c) {
        let e = MoveEvaluator {m, b: &board};
        let real_score = e.evaluate();
        moves.push((m, real_score));
    }
    moves.sort_by_key(|(_,r)| -r);
    //evaluate defensive moves
    for (m,real_score) in moves.iter_mut() {
        let mut b = board.clone();
        b.play_move(*m);
        let opponents_reponse = get_best_move(&b, c.opposite_color(), depth-1);
        if let Some((opponents_move, opponents_score)) = opponents_reponse {
            if depth == 4 {
                println!("{:?}, {}, {}, {:?}", m, real_score, opponents_score, opponents_move);
            }
            *real_score -= opponents_score;
        }

    }

    moves.sort_by_key(|(_,r)| -r);
    
    if depth == 4 {
        println!("---");
        for (m,r) in &moves {
            println!("{:?}, {}", m, r);
        }
    }
    
    moves.first().map(|(m,r)| (*m,*r))
}

pub fn get_best_move_alpha_beta(board: &Board, depth: i8, mut alpha: i32, mut beta: i32, c: Color, return_move: Option<Move>) -> (i32, Option<Move>) {
    if depth == 0 {
        return (board.evaluate(), return_move);
    }
    if c == White {
        let mut best_move = None;
        for m in MoveGenerator::get_moves(board, White) {
            let mut b = board.clone();
            b.play_move(m);
            let (score, m1) = get_best_move_alpha_beta(&b, depth-1, alpha, beta, c.opposite_color(), Some(m));
            if score > alpha {
                alpha = score;
                best_move = m1;
                // Alpha-Beta Pruning cutoff
                if alpha >= beta {
                    break;
                }
            }
        }
        return (alpha, best_move);
    }
    else {
        let mut best_move = None;
        for m in MoveGenerator::get_moves(board, Black) {
            let mut b = board.clone();
            b.play_move(m);
            let (score, m1) = get_best_move_alpha_beta(&b, depth-1, alpha, beta, c.opposite_color(), Some(m));
            if score < beta {
                beta = score;
                best_move = m1;
                // Alpha-Beta Pruning cutoff
                if alpha >= beta {
                    break;
                }
            }
        }
        return (beta, best_move);
    }
}


pub struct MoveEvaluator<'a> {
    pub m: Move,
    pub b: &'a Board,
}

impl<'a> MoveEvaluator<'a> {
    pub fn evaluate(&self) -> i32 {
        let mut real_score = 0;
        if let Some(p) = self.get_immediate_capture() {
            real_score += p.get_value();
        }
        //Pinning pieces?
        let mut b = self.b.clone();
        b.play_move(self.m);
        for p in self.get_attacked_pieces(&b) {
            real_score += p.get_value() / 10;
        }

        let smaller_center = Board::get_smaller_center();
        let larger_center = Board::get_larger_center();
        if smaller_center.contains(&self.m.dst) {
            real_score += 50;
        } else if larger_center.contains(&self.m.dst) {
            real_score += 25;
        }
        if self.m.is_castle(&self.b) {
            real_score += 100;
        }

        let pawn = b.get(self.m.dst);
        if let Some(pawn) = pawn {
            if pawn.piece == Pawn {
                let mut is_endgame = false;
                if b.count_pieces() < 12 {
                    is_endgame = true;
                }
                if pawn.color == Color::White {
                    real_score += ((7-self.m.dst[0]) * 10) as i32;

                    if self.m.dst[0] == 0 {
                        real_score += 900;
                    }
                }
                else {
                    real_score += (self.m.dst[0] * 10) as i32;
                    if self.m.dst[0] == 7 {
                        real_score += 900;
                    }
                }
                if is_endgame {
                    real_score += 100;
                }
            }
        }
        //opponents move attacks something of ours
        //defending or moving or blocking gains some points
        //add to anticipated score


        real_score
    }

    pub fn get_immediate_capture(&self) -> Option<Piece> {
        let p = self.b.get(self.m.src).unwrap();
        if let Some(capture) = self.b.get(self.m.dst) {
            assert!(capture.color != p.color);
            return Some(capture.piece);
        }
        None
    }

    pub fn get_attacked_pieces(&self, b: &Board) -> Vec<Piece> { //play move before we enter this func
        let mut attacked_pieces = Vec::new();
        let piece = b.get(self.m.dst).unwrap();
        for m in MoveGenerator::get_moves_with_predicates(&b, piece.color, 
            |p| p == piece.piece, &|dst_state| dst_state != DestinationState::Occupied) {
            let e = MoveEvaluator {m, b};
            if let Some(capture) = e.get_immediate_capture() {
                attacked_pieces.push(capture);
            }
        }
        attacked_pieces
    }
    /* 
    pub fn is_check(&self) -> bool {
        self.get_attacked_pieces().contains(&King)
    }
    */
     
    pub fn is_legal_move(&self) -> bool {
        let piece = self.b.get(self.m.src);
        if let Some(piece) = piece {
            if piece.color == self.b.m {
                for m in MoveGenerator::get_moves(self.b, piece.color) {
                    m.print();
                    if Move::equal(&self.m, &m) {
                        return true;
                    }
                }
            }
        }

        false
    }
    
}

pub fn play(board: &Board, mut c: Color, mut use_book: bool) {
    let mut game_over = false;
    let mut move_number = 0;
    let mut move_history = String::new();
    let mut board = board.clone();
    let book = fs::read_to_string("src/book.txt").expect("bad read");
    let lines = book.lines().collect::<Vec<&str>>();
    //shuffle lines to get random opening


    for _ in 0..100 {
        let m = if use_book {
            book_moves(&move_history, &lines, move_number)
        } else {
            get_best_move(&board, c, 4).map(|(m,_)| m)
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

        let p = board.get(m.dst);
        if let Some(p) = p {
            let (_,attackers) = board.is_attacked(m.dst);
            let (_,defenders) = board.is_defended(m.dst);
            println!("{},{}", attackers, defenders);

            if matches!(p.piece, King) {
                game_over = true;
            }
        }
        
        m.print();
        board.play_move(m);
        move_history.push_str(&m.to_move_string());
        move_history.push(' ');

        move_number += 1;
        c = c.opposite_color();
        board.print();
        println!("\n");
        
        if game_over {
            println!("King captured");
            break;
        }
    }
}

pub fn play_alpha_beta(board: &Board, mut player: Color, mut use_book: bool) {
    let mut game_over = false;
    let mut move_number = 0;
    let mut move_history = String::new();
    let mut board = board.clone();
    let book = fs::read_to_string("src/book.txt").expect("bad read");
    let lines = book.lines().collect::<Vec<&str>>();
    //shuffle lines to get random opening


    for _ in 0..100 {
        let (_,m) = get_best_move_alpha_beta(&board, 4, -10000000, 10000000, player, None);
        if m.is_none() {
            if use_book {
                use_book = false;
                continue;
            } else {
                break;
            }

        }
        let m = m.unwrap();

        let p = board.get(m.dst);
        if let Some(p) = p {
            let (_,attackers) = board.is_attacked(m.dst);
            let (_,defenders) = board.is_defended(m.dst);
            println!("{},{}", attackers, defenders);

            if matches!(p.piece, King) {
                game_over = true;
            }
        }
        
        m.print();
        board.play_move(m);
        move_history.push_str(&m.to_move_string());
        move_history.push(' ');

        move_number += 1;
        player = player.opposite_color();
        board.print();
        println!("\n");
        
        if game_over {
            println!("King captured");
            break;
        }
    }
}

pub fn book_moves(move_order: &str, lines: &Vec<&str>, move_number: usize) -> Option<Move> {
    let mut best_length = "";
    dbg!(move_number, move_order);
    if move_number == 0 { 
        return Some(Move::chess_notation_to_move("e2e4"));
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
        return moves.get(move_number).map(|m| Move::chess_notation_to_move(m));
    }

    None
}
