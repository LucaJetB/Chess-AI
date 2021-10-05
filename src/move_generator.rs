use std::thread::park_timeout;

use crate::board::{Board, ColoredPiece, Color, Piece, Move, add};
use Color::*;
use Piece::*;
use DesitinationState::*;

#[derive(Debug)]
pub struct MoveGenerator<'a> {
    pub board: &'a Board,
    pub start_pos: [i8;2],
    pub piece: ColoredPiece,
    pub moves: &'a mut Vec<Move>,
    //TODO add defend bool here
}

pub struct MoveGenerator2<'a, DstFilter: Fn(DesitinationState) -> bool> {
    pub board: &'a Board,
    pub start_pos: [i8;2],
    pub piece: ColoredPiece,
    pub moves: &'a mut Vec<Move>,
    pub dst_filter: DstFilter,
    //TODO add defend bool here
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesitinationState {
    Free,
    Occupied,
    Capturable,
    OutOfBounds,
}

impl<'a> MoveGenerator<'a> {
    pub fn maybe_add_move(&mut self, dst: Option<[i8;2]>, player_color: Color) {
        if let Some(dst) = dst {
            self.add_move(dst, Free, player_color)
        }
    }
    pub fn add_move(&mut self, dst: [i8;2], dst_state: DesitinationState, player_color: Color) {
        let dst_state = Move::get_dst_state(&self.board, dst, player_color);
        let m = Move {
            src: self.start_pos,
            dst,
            dst_state,
        };
        self.moves.push(m);
    }


    pub fn get_pawnmoves(&mut self, defend: bool) { 
        use DesitinationState::*;
        let (pawn_start, dir): (i8, i8) = match self.piece.color {
            White => (6,-1),
            Black => (1, 1),
        };
        let dst = [self.start_pos[0] + (1*dir), self.start_pos[1]];
        if matches!(Move::get_dst_state(self.board, dst, self.piece.color), Free) {
            self.add_move(dst, Free, self.piece.color);
            if self.start_pos[0] == pawn_start {
                let dst = [self.start_pos[0] + (2*dir), self.start_pos[1]];
                if matches!(Move::get_dst_state(self.board, dst, self.piece.color), Free) {
                     self.add_move(dst, Free, self.piece.color)
                }
            }
        }
        for hdir in &[-1,1] {
            let dst = [(self.start_pos[0] + (1*dir)), (self.start_pos[1] + *hdir)];
            if matches!(Move::get_dst_state(self.board, dst, self.piece.color), Capturable) {
                self.add_move(dst, Capturable, self.piece.color)
            } 
            if defend {
                if matches!(Move::get_dst_state(self.board, dst, self.piece.color), Occupied) {
                    self.add_move(dst, Occupied, self.piece.color)
                }
            }
        }
        // TODO en passant
    }

    pub fn get_knightmoves(&mut self, defend: bool) {
        use DesitinationState::*;
        let xvals = [2,2,1,1,-1,-1,-2,-2];
        let yvals = [1,-1,2,-2,2,-2,1,-1];
        for i in 0..7 {
            let dst = [self.start_pos[0] + yvals[i], self.start_pos[1] + xvals[i]];
            match Move::get_dst_state(self.board, dst, self.piece.color) {
                Free => self.add_move(dst, Free, self.piece.color),
                Occupied => {
                    if defend {
                        self.add_move(dst, Occupied, self.piece.color)

                    }
                },
                Capturable => self.add_move(dst, Capturable, self.piece.color),
                OutOfBounds => {},
            }
        }
    }


    pub fn get_linemoves(&mut self, dirs: &[[i8;2]], defend: bool) {
        use DesitinationState::*;
        for dir in dirs {
            for i in 1..7 {
                let dst = [self.start_pos[0] + (i*dir[0]), self.start_pos[1] + (i*dir[1])];
                match Move::get_dst_state(self.board, dst, self.piece.color) {
                    Free => self.add_move(dst, Free, self.piece.color),
                    Occupied => {
                        if defend {
                            self.add_move(dst, Occupied, self.piece.color)
                        }
                        break;
                    },
                    OutOfBounds => break,
                    Capturable => {
                        self.add_move(dst, Capturable, self.piece.color);
                        break
                    }
                }
            }
        }
    }
    pub fn get_bishopmoves(&mut self, defend: bool) {
        self.get_linemoves(&[[1,1],[-1,1],[-1,-1],[1,-1]], defend)
    }

    pub fn get_rookmoves(&mut self, defend: bool) {
        self.get_linemoves(&[[1,0],[-1,0],[0,-1],[0,1]], defend)
    }

    pub fn get_queenmoves(&mut self, defend: bool) {
        self.get_linemoves(&[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]], defend)
    }

    pub fn get_kingmoves(&mut self, defend: bool) {
        use DesitinationState::*;
        let dirs = &[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]];
        for dir in dirs {        
            let dst = [self.start_pos[0] + (dir[0]), self.start_pos[1] + (dir[1])];
            match Move::get_dst_state(self.board, dst, self.piece.color) {
                Free => self.add_move(dst, Free, self.piece.color),
                Occupied => {
                    if defend {
                        self.add_move(dst, Occupied, self.piece.color)
                    }
                },
                Capturable => {
                    let piece = Board::get(self.board, dst);
                    if let Some(piece) = piece {
                        self.add_move(dst, Capturable, self.piece.color);
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
                    self.maybe_add_move(dst, self.piece.color);
                }
            }
            if c.white_queenside {
                let r = self.board.get([7,0]);
                if let Some(r) = r {
                    let dst = self.get_queenside_castle();
                    self.maybe_add_move(dst, self.piece.color);
                }
            }
        } else {
            if c.black_kingside {
                let r = self.board.get([0,7]);
                if let Some(r) = r {
                    let dst = self.get_kingside_castle();
                    self.maybe_add_move(dst, self.piece.color);
                }
            }
            if c.black_queenside {
                let r = self.board.get([0,0]);
                if let Some(r) = r {
                    let dst = self.get_queenside_castle();
                    self.maybe_add_move(dst,self.piece.color);
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
        for m in MoveGenerator::get_moves_with_predicate(self.board, self.piece.color.opposite_color(), false, |p| p != King) {
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

    pub fn get_pieces_moves(&mut self, defend: bool) {
         match self.piece.piece {
            Pawn => self.get_pawnmoves(defend),
            Knight => self.get_knightmoves(defend),
            Bishop => self.get_bishopmoves(defend),
            Rook => self.get_rookmoves(defend),
            Queen => self.get_queenmoves(defend),
            King => {
                self.get_kingmoves(defend);
                //fix castling, make sure it works without stack overflowing... getmoves with movegen, constrain to only look at castle sqaures
                self.get_castle_moves();
            }
        }
    }




    
    pub fn get_moves_for_piece(board: &Board, pos: [i8;2]) -> Vec<Move> { 
        let p = board.get(pos).unwrap();
        let mut results = Vec::new();
        let mut gen = MoveGenerator {
            board, 
            start_pos: pos,
            piece: p,
            moves: &mut results,
        };
        gen.get_pieces_moves(false);
        results
    }
    /* 
    pub fn get_moves_for_color(board: &Board, color: Color, skip_king: bool) -> Vec<Move> { 
        let mut results = Vec::new();
        for i in 0..8_i8 {
            for j in 0..8_i8 {
                match board.arr[i as usize][j as usize] { // ifmatch!
                    Some(ColoredPiece{color: c, piece}) if c == color => {
                        if skip_king && piece == King {
                            continue;
                        }
                        let mut gen = MoveGenerator {
                            board, 
                            start_pos, 
                            piece: p,
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
    */
    //add a dst state filter to be able to search for dst of certain type
    //maybe put lamda in mg itself 
    pub fn get_moves_with_predicate(board: &Board, color: Color, defend: bool, filter: impl Fn(Piece) -> bool) -> Vec<Move> { 
        let mut results = Vec::new();
        for i in 0..8_i8 {
            for j in 0..8_i8 {
                match board.arr[i as usize][j as usize] { // ifmatch!
                    Some(ColoredPiece{color: c, piece}) if c == color => {
                        if !filter(piece) {
                            continue;
                        }
                        let mut gen = MoveGenerator {
                            board, 
                            start_pos: [i,j],
                            piece: ColoredPiece {piece, color,},
                            moves: &mut results,
                            //dst_filter: filter,
                        };
                        gen.get_pieces_moves(defend);
                    }
                    _ => {}
                }
            }
        }
        results
    }

    pub fn get_moves(board: &Board, color: Color) -> Vec<Move> {
        Self::get_moves_with_predicate(board, color, false, |_| true)
    }

    pub fn get_moves_with_defense(board: &Board, color: Color) -> Vec<Move> {
        Self::get_moves_with_predicate(board, color, true, |_| true)
    }

    
}

