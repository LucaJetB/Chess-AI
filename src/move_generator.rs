use std::thread::park_timeout;

use crate::board::{Board, ColoredPiece, Color, Piece, Move, add};
use Color::*;
use Piece::*;

#[derive(Debug)]
pub struct MoveGenerator<'a> {
    pub board: &'a Board,
    pub start_pos: [i8;2],
    pub piece: ColoredPiece,
    pub moves: &'a mut Vec<Move>,
}

pub enum DesitinationState {
    Free,
    Occupied,
    Capturable,
    OutOfBounds,
}

/*
#28 0x000055555556783a in chess::move_generator::MoveGenerator::get_pieces_moves ()
#29 0x0000555555567c25 in chess::move_generator::movegen_get_moves ()
#30 0x0000555555568274 in chess::board::Board::is_defended ()
*/

impl<'a> MoveGenerator<'a> {
    pub fn maybe_add_move(&mut self, dst: Option<[i8;2]>) {
        if let Some(dst) = dst {
            self.add_move(dst)
        }
    }
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
            match self.get_dst_state(dst) {
                Free => self.add_move(dst),
                Occupied => {},
                Capturable => {
                    let piece = Board::get(self.board, dst);
                    if let Some(piece) = piece {
                        self.add_move(dst);
                    }

                },
                OutOfBounds => {},
            }
        }
    }
    

    pub fn get_castle_moves(&mut self) {
        if self.start_pos[1] != 4 {
            return;
        }
        let c = &self.board.castling;
        if self.piece.color == White {
            if c.white_kingside {
                let r = self.board.get([7,7]);
                if let Some(r) = r {
                    let dst = self.get_kingside_castle();
                    self.maybe_add_move(dst);
                }
            }
            if c.white_queenside {
                let r = self.board.get([7,0]);
                if let Some(r) = r {
                    let dst = self.get_queenside_castle();
                    self.maybe_add_move(dst);
                }
            }
        } else {
            if c.black_kingside {
                let r = self.board.get([0,7]);
                if let Some(r) = r {
                    let dst = self.get_kingside_castle();
                    self.maybe_add_move(dst);
                }
            }
            if c.black_queenside {
                let r = self.board.get([0,0]);
                if let Some(r) = r {
                    let dst = self.get_queenside_castle();
                    self.maybe_add_move(dst);
                }
            }
        }
    }

    pub fn get_castle(&mut self, dir: i8) -> Option<[i8;2]> { 
        let p1 = add(self.start_pos, [0,(1*dir)]);
        let p2 = add(self.start_pos, [0,(2*dir)]);
        if self.board.get(p1) != None || self.board.get(p2) != None {
            return None;
        }
        for m in MoveGenerator::get_moves(self.piece.color.opposite_color(), self.board) {
            if m.dst == p1 || m.dst == p2 {
                return None;
            }
        }
        Some(p2)
    }

    pub fn get_kingside_castle(&mut self) -> Option<[i8;2]> {
        self.get_castle(1)
    }
    pub fn get_queenside_castle(&mut self) -> Option<[i8;2]> {
        self.get_castle(-1)
    }

    pub fn get_pieces_moves(&mut self) {
         match self.piece.piece {
            Pawn => self.get_pawnmoves(),
            Knight => self.get_knightmoves(),
            Bishop => self.get_bishopmoves(),
            Rook => self.get_rookmoves(),
            Queen => self.get_queenmoves(),
            King => {
                self.get_kingmoves();
                //fix castling, make sure it works without stack overflowing... getmoves with movegen, constrain to only look at castle sqaures
                self.get_castle_moves();
            }
        }
    }


    pub fn get_moves_no_king(c: Color, board: &'a Board) -> Vec<Move> {
        movegen_get_moves_no_king(c,board)
    }

    pub fn get_moves(c: Color, board: &'a Board) -> Vec<Move> {
        movegen_get_moves(c,board)
    }
    pub fn get_all_moves(board: &'a Board) -> Vec<Move> {
        movegen_get_all_moves(board)
    }
    
    pub fn movegen_get_piece_moves(board: &Board, p: ColoredPiece, start_pos: [i8;2]) -> Vec<Move> { 
        let mut results = Vec::new();
        let mut gen = MoveGenerator {
            board, 
            start_pos, 
            piece: p,
            moves: &mut results,
        };
        gen.get_pieces_moves();
        results
    }
}

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
                    gen.get_pieces_moves();

                }
                _ => {}
           }
        }
    }
    results
}

pub fn movegen_get_moves_no_king(c: Color, board: &Board) -> Vec<Move> { 
    let mut results = Vec::new();
    for i in 0..8_i8 {
        for j in 0..8_i8 {
            match board.arr[i as usize][j as usize] { // ifmatch!
                Some(ColoredPiece{color, piece}) if color == c => {
                    if piece != King {
                        let mut gen = MoveGenerator {
                            board, 
                            start_pos: [i,j], 
                            piece: ColoredPiece{color, piece},
                            moves: &mut results,
                        };
                        gen.get_pieces_moves();
                    }
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
                    gen.get_pieces_moves();

                }
                _ => {}
           }
        }
    }
    results
}
