#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
fn main() {
}
//TODO add values to pieces
//tally total value for a given color
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
    let b1 = Board::from_str(board1);
    let b2 = Board::from_str(board2);
    dbg!(&b1);
    assert_eq!(b1.arr[0][6], Some(ColoredPiece{color: Black, piece: King}));
    println!("Board 1: ");
    Board::get_advantage(b1);
    println!("");
    println!("Board 2: ");
    Board::get_advantage(b2);
    println!("");
    Board::print_board(board1);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn get_value(&self) -> i32 {
        match self {
            Pawn => 1,
            Knight => 3,
            Bishop => 3,
            Rook => 5,
            Queen => 9,
            King => 10000,
        }
    }

}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,

}

impl Color {
    pub fn opposite_color(&self) -> Self {
        match self {
            White => Black,
            Black => White,
        }
    }
}


use std::any::Any;

use Color::*; 
use Piece::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColoredPiece {
    color: Color,
    piece: Piece,
}

impl ColoredPiece {
    pub fn from_char(c: char) -> Self {
        let (color,piece, value) = match c {
            'P' => (White, Pawn, 1),
            'N' => (White, Knight, 3),
            'B' => (White, Bishop, 3),
            'R' => (White, Rook, 5),
            'Q' => (White, Queen, 9),
            'K' => (White, King, 100),

            'p' => (Black, Pawn, 1),
            'n' => (Black, Knight, 3),
            'b' => (Black, Bishop, 3),
            'r' => (Black, Rook, 5),
            'q' => (Black, Queen, 9),
            'k' => (Black, King, 100),

            _ => panic!("You fucked up"),
        };
        Self {color, piece}
    }


}
 // create pos as a struct or tuple for easy arithmetic funcs on it
 //

#[derive(Debug)]
struct MoveGenerator<'a> {
    board: &'a Board,
    start_pos: [i8;2],
    piece: ColoredPiece,
    moves: Vec<[i8; 2]>,
}

enum DesitinationState {
    Free,
    Occupied,
    Capturable,
    OutOfBounds,
}

impl<'a> MoveGenerator<'a> {
    fn get_dst_state(&self, pos: [i8;2]) -> DesitinationState {
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

    fn get_pawnmoves(&mut self) {
        let mut double = true;
        use DesitinationState::*;
        let (pawn_start, dir): (i8, i8) = match self.piece.color {
            White => (6,-1),
            Black => (1, 1),
        };
        let mut up = [self.start_pos[0] + (1*dir), self.start_pos[1]];
        match self.get_dst_state(up) {
            Free => self.moves.push(up),
            Occupied | OutOfBounds => double = false,
            Capturable => panic!("capturing forward"),
        }
        if double && self.start_pos[1] == pawn_start {
            up = [up[0] + (1*dir), up[1]];
            match self.get_dst_state(up) {
                Free => self.moves.push(up),
                Occupied | OutOfBounds => {},
                Capturable => panic!("capturing forward"),
            }
        }
        let diag1 = [(self.start_pos[0] + (1*dir)), (self.start_pos[1] - 1)];
        match self.get_dst_state(diag1) {
            Free => {},
            Occupied | OutOfBounds => {},
            Capturable => self.moves.push(diag1),
        }
        let diag2 = [(self.start_pos[0] + (1*dir)), (self.start_pos[1] + 1)];
        match self.get_dst_state(diag2) {
            Free => {},
            Occupied | OutOfBounds => {},
            Capturable => self.moves.push(diag2),
        }
    }
    fn get_knightmoves(&mut self) {
        use DesitinationState::*;
        let xvals = [2,2,1,1,-1,-1,-2,-2];
        let yvals = [1,-1,2,-2,2,-2,1,-1];
        for i in 0..7 {
            let dst = [self.start_pos[0] + xvals[i], self.start_pos[1] + yvals[i]];
            match self.get_dst_state(dst) {
                Free => self.moves.push(dst),
                Occupied | OutOfBounds => {},
                Capturable => self.moves.push(dst),
            }
        }
    }


    fn get_linemoves(&mut self, dirs: &[[i8;2]]) {
        use DesitinationState::*;
        for dir in dirs {
            for i in 1..7 {
                let dst = [self.start_pos[0] + (i*dir[0]), self.start_pos[1] + (i*dir[1])];
                match self.get_dst_state(dst) {
                    Free => self.moves.push(dst),
                    Occupied | OutOfBounds => break,
                    Capturable => {
                        self.moves.push(dst);
                        break
                    }
                }
            }
        }
    }
    //for changing operators pass parameters as arrays or could pass lamdas

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
        use DesitinationState::*;
        let dirs = &[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]];
        for dir in dirs {        
            let dst = [self.start_pos[0] + (dir[0]), self.start_pos[1] + (dir[1])];
            match self.get_dst_state(dst) {
                Free => self.moves.push(dst),
                Occupied | OutOfBounds => break,
                Capturable => {
                    self.moves.push(dst);
                    break
                }
            }
        }
    }

    fn get_piece_moves(&mut self) {
         match self.piece.piece {
            Pawn => self.get_pawnmoves(),
            Knight => self.get_knightmoves(),
            Bishop => self.get_bishopmoves (),
            Rook => self.get_rookmoves(),
            Queen => self.get_queenmoves(),
            King => self.get_kingmoves(),
        }
    }
    fn get_moves(c: Color, board: &'a Board) {
        let results = Vec::new();
        for i in 0..8_i8 {
            for j in 0..8_i8 {
                match board.arr[i as usize][j as usize] { //back to match
                    Some(ColoredPiece{color, piece}) if color == c => {
                        let mut gen = Self {
                            board, 
                            start_pos: [i,j], 
                            piece: ColoredPiece{color, piece},
                            moves: results,
                        };
                        gen.get_piece_moves();
                    }
                    _ => {}
               }
            }
        }
    }
}
#[derive(Debug)]
struct Move {
    from: [i8; 2],
    to: [i8; 2],
}

#[derive(Debug)]
struct Board {
     arr: [[Option<ColoredPiece>; 8]; 8],
}


impl Board {
    fn from_str(sboard: &str) -> Self {
        assert!(sboard.len() == 8*16);
        let sboard: Vec<_> = sboard.chars().collect();
        let mut board = [[None; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                let index = 16*i + 2*j;
                if sboard[index] != '.' {
                    board[i][j] = Some(ColoredPiece::from_char(sboard[index]))
                }
            }
        }
        Self {arr: board}
    }
    fn print_board(sboard: &str) {
        println!("{}", sboard);
        
    }

    pub fn in_bounds(pos: [i8;2]) -> bool {
        matches!(pos, [0..=7,0..=7])
    }

    pub fn get(&self, pos: [i8; 2]) -> Option<ColoredPiece> {
        self.arr[pos[0] as usize][pos[1] as usize]
    }

    fn get_advantage(b: Board) {
        let mut white_score = 0;
        let mut black_score = 0;
        for i in 0..8{
            for j in 0..8 {
                match b.arr[i][j] {
                    Some(ColoredPiece{color: White, piece}) => white_score += piece.get_value(),
                    Some(ColoredPiece{color: Black, piece}) => black_score += piece.get_value(),
                    None => {},
                    }
            }     
        }
        println!("Whites score is {}", white_score);
        println!("Blacks score is {}", black_score);
        if white_score == black_score {
            println!("Material is equal");
        }
        else if white_score > black_score {
            println!("White has advantage");
        }
        else {
            println!("Black has advantage"); 
        }
    }

}



