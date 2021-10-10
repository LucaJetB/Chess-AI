use crate::board::{Board, ColoredPiece, Color, Piece, Move, add};
use Color::*;
use Piece::*;
use DestinationState::*;

pub struct MoveGenerator<'a> {
    pub board: &'a Board,
    pub start_pos: [i8;2],
    pub piece: ColoredPiece,
    pub moves: &'a mut Vec<Move>,
    pub dst_filter: &'static dyn Fn(DestinationState) -> bool,
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationState {
    Free,
    Occupied,
    Capturable,
    OutOfBounds,
}

impl<'a> MoveGenerator<'a> {
    fn add_move(&mut self, dst: [i8;2], dst_state: DestinationState) {
        if !(self.dst_filter)(dst_state) {
            return;
        }
        let m = Move {
            src: self.start_pos,
            dst,
            dst_state,
        };
        self.moves.push(m);
    }


    fn get_pawnmoves(&mut self) { 
        use DestinationState::*;
        let (pawn_start, dir): (i8, i8) = match self.piece.color {
            White => (6,-1),
            Black => (1, 1),
        };
        let dst = [self.start_pos[0] + (1*dir), self.start_pos[1]];
        if matches!(Move::get_dst_state(self.board, dst, self.piece.color), Free) {
            self.add_move(dst, Free);
            if self.start_pos[0] == pawn_start {
                let dst = [self.start_pos[0] + (2*dir), self.start_pos[1]];
                if matches!(Move::get_dst_state(self.board, dst, self.piece.color), Free) {
                     self.add_move(dst, Free)
                }
            }
        }
        for hdir in &[-1,1] {
            let dst = [(self.start_pos[0] + (1*dir)), (self.start_pos[1] + *hdir)];
            let dst_state = Move::get_dst_state(self.board, dst, self.piece.color);
            if matches!(dst_state, Capturable | Occupied) {
                self.add_move(dst, dst_state);
            } 

        }
        // TODO en passant
    }

    fn get_knightmoves(&mut self) {
        use DestinationState::*;
        let xvals = [2,2,1,1,-1,-1,-2,-2];
        let yvals = [1,-1,2,-2,2,-2,1,-1];
        for i in 0..7 {
            let dst = [self.start_pos[0] + yvals[i], self.start_pos[1] + xvals[i]];
            let dst_state = Move::get_dst_state(self.board, dst, self.piece.color);
            if dst_state != OutOfBounds {
                self.add_move(dst, dst_state);
            }
        }
    }


    fn get_linemoves(&mut self, dirs: &[[i8;2]]) {
        use DestinationState::*;
        for dir in dirs {
            for i in 1..7 {
                let dst = [self.start_pos[0] + (i*dir[0]), self.start_pos[1] + (i*dir[1])];
                match Move::get_dst_state(self.board, dst, self.piece.color) {
                    Free => self.add_move(dst, Free),
                    Occupied => {
                        if self.defend {
                            self.add_move(dst, Occupied)
                        }
                        break;
                    },
                    OutOfBounds => break,
                    Capturable => {
                        self.add_move(dst, Capturable);
                        break
                    }
                }
            }
        }
    }
    fn get_bishopmoves(&mut self) {
        self.get_linemoves(&[[1,1],[-1,1],[-1,-1],[1,-1]])
    }

    fn get_rookmoves(&mut self) {
        self.get_linemoves(&[[1,0],[-1,0],[0,-1],[0,1]])
    }

    fn get_queenmoves(&mut self) {
        self.get_linemoves(&[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]])
    }

    fn get_kingmoves(&mut self) {
        use DestinationState::*;
        let dirs = &[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]];
        for dir in dirs {        
            let dst = [self.start_pos[0] + (dir[0]), self.start_pos[1] + (dir[1])];
            match Move::get_dst_state(self.board, dst, self.piece.color) {
                Free => self.add_move(dst, Free),
                Occupied => {
                    if self.defend {
                        self.add_move(dst, Occupied)
                    }
                },
                Capturable => {
                    let piece = Board::get(self.board, dst);
                    if let Some(_) = piece {
                        self.add_move(dst, Capturable);
                    }

                },
                OutOfBounds => {},
            }
        }
    }
    

    fn get_castle_moves(&mut self) {
        fn maybe_add_move(mg: &mut MoveGenerator, dst: Option<[i8;2]>) {
            if let Some(dst) = dst {
                // If the dst is Some() then its dst_state must be Free because of check we do in get_castle()
                mg.add_move(dst, Free) 
            }
        }

        if self.start_pos[1] != 4 {
            return;
        }
        let c = &self.board.castling;
        if self.piece.color == White {
            if c.white_kingside {
                let r = self.board.get([7,7]);
                if let Some(_) = r {
                    let dst = self.get_kingside_castle();
                    maybe_add_move(self, dst);
                }
            }
            if c.white_queenside {
                let r = self.board.get([7,0]);
                if let Some(_) = r {
                    let dst = self.get_queenside_castle();
                    maybe_add_move(self, dst);
                }
            }
        } else {
            if c.black_kingside {
                let r = self.board.get([0,7]);
                if let Some(_) = r {
                    let dst = self.get_kingside_castle();
                    maybe_add_move(self, dst);
                }
            }
            if c.black_queenside {
                let r = self.board.get([0,0]);
                if let Some(_) = r {
                    let dst = self.get_queenside_castle();
                    maybe_add_move(self, dst);
                }
            }
        }
    }

    fn get_castle(&mut self, dir: i8) -> Option<[i8;2]> { 
        let p1 = add(self.start_pos, [0,(1*dir)]);
        let p2 = add(self.start_pos, [0,(2*dir)]);
        if self.board.get(p1) != None || self.board.get(p2) != None {
            return None;
        }
        for m in MoveGenerator::get_moves_with_predicates(self.board, self.piece.color.opposite_color(), false, 
        |p| p != King, &|_| true) {
            if m.dst == p1 || m.dst == p2 {
                return None;
            }
        }
        Some(p2)
    }

    fn get_kingside_castle(&mut self) -> Option<[i8;2]> {
        self.get_castle(1)
    }
    fn get_queenside_castle(&mut self) -> Option<[i8;2]> {
        self.get_castle(-1)
    }

    fn get_pieces_moves(&mut self) {
         match self.piece.piece {
            Pawn => self.get_pawnmoves(),
            Knight => self.get_knightmoves(),
            Bishop => self.get_bishopmoves(),
            Rook => self.get_rookmoves(),
            Queen => self.get_queenmoves(),
            King => {
                self.get_kingmoves();
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
            defend: false,
            dst_filter: &|_| true,
        };
        gen.get_pieces_moves();
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
    pub fn get_moves_with_predicates(board: &Board, color: Color, defend: bool, piece_filter: impl Fn(Piece) -> bool, dst_filter: &'static dyn Fn(DestinationState) -> bool) -> Vec<Move> { 
        let mut results = Vec::new();
        for i in 0..8_i8 {
            for j in 0..8_i8 {
                match board.arr[i as usize][j as usize] { // ifmatch!
                    Some(ColoredPiece{color: c, piece}) if c == color => {
                        if !piece_filter(piece) {
                            continue;
                        }
                        let mut gen = MoveGenerator {
                            board, 
                            start_pos: [i,j],
                            piece: ColoredPiece {piece, color,},
                            moves: &mut results,
                            defend,
                            dst_filter,
                        };
                        gen.get_pieces_moves();
                    }
                    _ => {}
                }
            }
        }
        results
    }

    pub fn get_moves(board: &Board, color: Color) -> Vec<Move> {
        Self::get_moves_with_predicates(board, color, false, 
            |_| true, &|dst_state| dst_state != Occupied)
    }

    pub fn get_defense_moves(board: &Board, color: Color) -> Vec<Move> {
        Self::get_moves_with_predicates(board, color, true, 
            |_| true, &|dst_state| dst_state == Occupied)
    }

}

