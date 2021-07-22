use std::thread::park_timeout;

use crate::board::{Board, ColoredPiece, Color, Piece, Move};
use Color::*;
use Piece::*;

// create pos as a struct or tuple for easy arithmetic funcs on it
 //


#[derive(Debug)]
pub struct MoveGenerator<'a> {
    pub board: &'a Board,
    pub start_pos: [i8;2],
    pub piece: ColoredPiece,
    pub moves: &'a mut Vec<Move>, //if this contains all moves should it be in this struct with piece specfic values?
}

pub enum DesitinationState {
    Free,
    Occupied,
    Capturable,
    OutOfBounds,
}


impl<'a> MoveGenerator<'a> {
    pub fn add_move(&mut self, dst: [i8;2]) {
        let m = Move {
            src: self.start_pos,
            dst
        };
        self.moves.push(m);
    }

    pub fn get_dst_state(&self, pos: [i8;2]) -> DesitinationState {
        use DesitinationState::*;
        if !Board::in_bounds(pos) {
            return OutOfBounds;
        }
        match self.board.get(pos) {
            Some(ColoredPiece {color, ..}) if color != self.piece.color => Capturable,
            Some(_) => Occupied,
            None => Free,
        }
    }

    pub fn get_pawnmoves(&mut self) { 
        use DesitinationState::*;
        let (pawn_start, dir): (i8, i8) = match self.piece.color {
            White => (6,-1),
            Black => (1, 1),
        };
        let dst = [self.start_pos[0] + (1*dir), self.start_pos[1]];
        if matches!(self.get_dst_state(dst), Free) {
            self.add_move(dst);
            if self.start_pos[0] == pawn_start {
                let dst = [self.start_pos[0] + (2*dir), self.start_pos[1]];
                if matches!(self.get_dst_state(dst), Free) {
                     self.add_move(dst)
                }
            }
        }
        for hdir in &[-1,1] {
            let dst = [(self.start_pos[0] + (1*dir)), (self.start_pos[1] + *hdir)];
            if matches!(self.get_dst_state(dst), Capturable) {
                self.add_move(dst)
            } 
        }
        // TODO en passant
    }

    pub fn get_knightmoves(&mut self) {
        use DesitinationState::*;
        let xvals = [2,2,1,1,-1,-1,-2,-2];
        let yvals = [1,-1,2,-2,2,-2,1,-1];
        for i in 0..7 {
            let dst = [self.start_pos[0] + yvals[i], self.start_pos[1] + xvals[i]];
            if matches!(self.get_dst_state(dst), Free | Capturable) {
                self.add_move(dst)
            }
        }
    }


    pub fn get_linemoves(&mut self, dirs: &[[i8;2]]) {
        use DesitinationState::*;
        for dir in dirs {
            for i in 1..7 {
                let dst = [self.start_pos[0] + (i*dir[0]), self.start_pos[1] + (i*dir[1])];
                match self.get_dst_state(dst) {
                    Free => self.add_move(dst),
                    Occupied | OutOfBounds => break,
                    Capturable => {
                        self.add_move(dst);
                        break
                    }
                }
            }
        }
    }
    //for changing operators pass parameters as arrays or could pass lamdas
    //rook must be teleported onto other side of king
    pub fn get_bishopmoves(&mut self) {
        self.get_linemoves(&[[1,1],[-1,1],[-1,-1],[1,-1]])
    }

    pub fn get_rookmoves(&mut self) {
        self.get_linemoves(&[[1,0],[-1,0],[0,-1],[0,1]])
    }

    pub fn get_queenmoves(&mut self) {
        self.get_linemoves(&[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]])
    }

    pub fn get_kingmoves(&mut self) {
        use DesitinationState::*;
        let dirs = &[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]];
        for dir in dirs {        
            let dst = [self.start_pos[0] + (dir[0]), self.start_pos[1] + (dir[1])];
            if matches!(self.get_dst_state(dst), Free | Capturable) {
                self.add_move(dst);
            }
        }
        //castling (check under attack sqaures)
        //if king has not moved && rook has not moved
        //if king has line of sight of rook
        //if sqaure to kings right/left is not targeted
        //king can move two square towards rook, rook must be teleported onto other side of king
    }

    pub fn get_piece_moves(&mut self) {
         match self.piece.piece {
            Pawn => self.get_pawnmoves(),
            Knight => self.get_knightmoves(),
            Bishop => self.get_bishopmoves (),
            Rook => self.get_rookmoves(),
            Queen => self.get_queenmoves(),
            King => self.get_kingmoves(),
        }
    }
    
    pub fn get_moves(c: Color, board: &'a Board) -> Vec<Move> {
        movegen_get_moves(c,board)
    }
    pub fn get_all_moves(board: &'a Board) -> Vec<Move> {
        movegen_get_all_moves(board)
    }
}
// TODO prune some illegal moves with castling (only going to be 1 square else lose the game) 
pub fn movegen_get_moves(c: Color, board: &Board) -> Vec<Move> { 
    let mut results = Vec::new();
    for i in 0..8_i8 {
        for j in 0..8_i8 {
            match board.arr[i as usize][j as usize] { // ifmatch!
                Some(ColoredPiece{color, piece}) if color == c => {
                    let mut gen = MoveGenerator {
                        board, 
                        start_pos: [i,j], 
                        piece: ColoredPiece{color, piece},
                        moves: &mut results,
                    };
                    gen.get_piece_moves();

                }
                _ => {}
           }
        }
    }
    results
}

pub fn movegen_get_all_moves(board: &Board) -> Vec<Move> { 
    let mut results = Vec::new();
    for i in 0..8_i8 {
        for j in 0..8_i8 {
            match board.arr[i as usize][j as usize] { // ifmatch!
                Some(ColoredPiece{color, piece}) => {
                    let mut gen = MoveGenerator {
                        board, 
                        start_pos: [i,j], 
                        piece: ColoredPiece{color, piece},
                        moves: &mut results,
                    };
                    gen.get_piece_moves();

                }
                _ => {}
           }
        }
    }
    results
}