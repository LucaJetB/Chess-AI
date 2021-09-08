use crate::board::{Board, ColoredPiece, Color, Piece, Move};
use crate::move_generator::{DesitinationState, MoveGenerator};
use Color::*; 
use Piece::*;

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

pub fn book_moves(move_order_S: String, lines: &Vec<&str>, move_number: i32) -> Option<Move> {
    let move_order = &move_order_S[..];
    if move_number == 0 { 
        return Some(chess_notation_to_move("e2e4"));
    }
    for line in lines {
        if line.contains(move_order) {
            let moves = line.split(' ').collect::<Vec<&str>>();
            return Some(chess_notation_to_move(moves[move_number as usize]));
        }
    }

    None
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