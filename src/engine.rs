use crate::board::{Board, ColoredPiece, Color, Piece, Move};
use crate::move_generator::{DesitinationState, MoveGenerator};
use Color::*; 
use Piece::*;

//breath first search
//teach checkmate, always go to eat king
pub fn get_best_move(board: &Board, c: Color, depth: i8) -> Option<Move> {
    if depth == 0 {
        return None;
    }
    let mut checks = Vec::new();
    let mut captures = Vec::new();
    let mut attacks = Vec::new();
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
        if new_score >= best_score {
            if m.is_check(board) {
                checks.push(m);
            }
            if m.is_capture(board) {
                captures.push(m);
            }
            if m.is_attack(board) {
                attacks.push(m);
            }
        }
    }
    if matches!(best_move, None) {
        if !checks.is_empty() {
            return Some(checks[0]);
        }
        if !captures.is_empty() {
            return Some(captures[0]);
        }
        if !attacks.is_empty() {
            return Some(attacks[0]);
        }
    }
    best_move
}