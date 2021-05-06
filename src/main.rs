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

    pub fn get_moves(&self, c: Color) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        let mut add_move = |m: Move|{
            if matches!(m.to,[0..=7,0..=7]) {
               vec.push(m);
            }
        };
        let mut get_piece = |pos: [i8; 2]| {
            if matches!(pos, [0..=7,0..=7]) {
                self.arr[i as usize][j as usize]
            } else {
                None
            }
        };

        let mut try_add_capture_move = |c: Color, m: Move| {
            if matches!(get_piece(m.to), Some(ColoredPiece{color, ..}) if color != c) {
                add_move(m);
            }
        };

        for i in 0..8_i8{
            for j in 0..8_i8 {
                match self.arr[i as usize][j as usize] {
                    Some(ColoredPiece{color, piece}) if color == c => {
                        match piece {
                            Pawn => {
                                let (pawn_start, dir): (i8, i8) = match color {
                                    White => (6,-1),
                                    Black => (1,1),
                                };
                                if i == pawn_start {
                                    add_move(Move {from: [i,j], to:[i, j+(2*dir)]});
                                }  
                                add_move(Move {from: [i,j], to: [i,j+dir]});

                                try_add_capture_move(c, Move {from: [i,j], to: [i+1,j+dir]});
                                try_add_capture_move(c, Move {from: [i,j], to: [i-1,j+dir]});

                                // if get_pice on diagonals returns opposite color then add that piece loc to moves
                            }
                            Knight => {
                                if matches!(get_piece(]i+1,j+2]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i+1,j+2]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i+1,j+2]});
                                }

                                if matches!(get_piece(]i+2,j+1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i+2,j+1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i+2,j+1]});
                                }

                                if matches!(get_piece(]i-1,j+2]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i-1,j+2]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i-1,j+2]});
                                }

                                if matches!(get_piece(]i-2,j+1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i-2,j+1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i-2,j+1]});
                                }

                                if matches!(get_piece(]i-1,j-2]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i-1,j-2]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i-1,j-2]});
                                }

                                if matches!(get_piece(]i-2,j-1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i-2,j-1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i-2,j-1]});
                                }

                                if matches!(get_piece(]i+1,j-2]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i+1,j-2]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i+1,j-2]});
                                }

                                if matches!(get_piece(]i+2,j-1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i+2,j-1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i+2,j-1]});
                                }
                            }

                            Bishop => {
                                let mut up_left = false;
                                let mut up_right = false;
                                let mut down_left = false;
                                let mut down_right = false;
                                let mut s = 1;
                                while up_left || up_right || down_left || down_right == false {
                                    if !Self::in_bounds([i+s,j+s]) {
                                        down_right = true;
                                    }
                                    if matches!(get_piece([i+s,j+s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i+s,j+s]});
                                        down_right = true;
                                    } 
                                    else if down_right == false {
                                        add_move(Move {from: [i,j], to: [i+s,j+s]});
                                    }

                                    if !Self::in_bounds([i-s,j+s]) {
                                        down_left = true;
                                    }
                                    if matches!(get_piece([i-s,j+s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j+s]});
                                        down_left = true;
                                    } 
                                    else if down_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j+s]});
                                    }

                                    if !Self::in_bounds([i-s,j-s]) {
                                        up_left = true;
                                    }
                                    if matches!(get_piece([i-s,j-s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j-s]});
                                        up_left = true;
                                    } 
                                    else if up_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j-s]});
                                    }

                                    if !Self::in_bounds([i+s,j-s]) {
                                        up_right = true;
                                    }
                                    if matches!(get_piece([i+s,j-s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i+s,j-s]});
                                        up_right = true;
                                    } 
                                    else if up_right == false {
                                        add_move(Move {from: [i,j], to: [i+s,j-s]});
                                    }
                                    s++;
                                }
                            }

                            Rook => {
                                let mut left = false;
                                let mut right = false;
                                let mut up = false;
                                let mut down = false;
                                let mut s = 1;
                                while left || right || up || down == false {
                                    if !Self::in_bounds([i+s,j]) {
                                        right = true;
                                    }
                                    if matches!(get_piece([i+s,j]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i+s,j]});
                                        right = true;
                                    } 
                                    else if right == false {
                                        add_move(Move {from: [i,j], to: [i+s,j]});
                                    }

                                    if !Self::in_bounds([i-s,j]) {
                                        left = true;
                                    }
                                    if matches!(get_piece([i-s,j]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j]});
                                        left = true;
                                    } 
                                    else if left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j]});
                                    }

                                    if !Self::in_bounds([i,j-s]) {
                                        up = true;
                                    }
                                    if matches!(get_piece([i,j-s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i,j-s]});
                                        up = true;
                                    } 
                                    else if up == false {
                                        add_move(Move {from: [i,j], to: [i,j-s]});
                                    }

                                    if !Self::in_bounds([i,j+s]) {
                                        down = true;
                                    }
                                    if matches!(get_piece([i,j+s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i,j+s]});
                                        down = true;
                                    } 
                                    else if down == false {
                                        add_move(Move {from: [i,j], to: [i,j+s]});
                                    }
                                    s++;
                                }
                            }

                            Queen => {
                                let mut up_left = false;
                                let mut up_right = false;
                                let mut down_left = false;
                                let mut down_right = false;
                                let mut s = 1;
                                while up_left || up_right || down_left || down_right == false {
                                    if !Self::in_bounds([i+s,j+s]) {
                                        down_right = true;
                                    }
                                    if matches!(get_piece([i+s,j+s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i+s,j+s]});
                                        down_right = true;
                                    } 
                                    else if down_right == false {
                                        add_move(Move {from: [i,j], to: [i+s,j+s]});
                                    }

                                    if !Self::in_bounds([i-s,j+s]) {
                                        down_left = true;
                                    }
                                    if matches!(get_piece([i-s,j+s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j+s]});
                                        down_left = true;
                                    } 
                                    else if down_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j+s]});
                                    }

                                    if !Self::in_bounds([i-s,j-s]) {
                                        up_left = true;
                                    }
                                    if matches!(get_piece([i-s,j-s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j-s]});
                                        up_left = true;
                                    } 
                                    else if up_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j-s]});
                                    }

                                    if !Self::in_bounds([i+s,j-s]) {
                                        up_right = true;
                                    }
                                    if matches!(get_piece([i+s,j-s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i+s,j-s]});
                                        up_right = true;
                                    } 
                                    else if up_right == false {
                                        add_move(Move {from: [i,j], to: [i+s,j-s]});
                                    }
                                    s++;
                                }

                                let mut left = false;
                                let mut right = false;
                                let mut up = false;
                                let mut down = false;
                                let mut s = 1;
                                while left || right || up || down == false {
                                    if !Self::in_bounds([i+s,j]) {
                                        right = true;
                                    }
                                    if matches!(get_piece([i+s,j]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i+s,j]});
                                        right = true;
                                    } 
                                    else if right == false {
                                        add_move(Move {from: [i,j], to: [i+s,j]});
                                    }

                                    if !Self::in_bounds([i-s,j]) {
                                        left = true;
                                    }
                                    if matches!(get_piece([i-s,j]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j]});
                                        left = true;
                                    } 
                                    else if left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j]});
                                    }

                                    if !Self::in_bounds([i,j-s]) {
                                        up = true;
                                    }
                                    if matches!(get_piece([i,j-s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i,j-s]});
                                        up = true;
                                    } 
                                    else if up == false {
                                        add_move(Move {from: [i,j], to: [i,j-s]});
                                    }

                                    if !Self::in_bounds([i,j+s]) {
                                        down = true;
                                    }
                                    if matches!(get_piece([i,j+s]]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i,j+s]});
                                        down = true;
                                    } 
                                    else if down == false {
                                        add_move(Move {from: [i,j], to: [i,j+s]});
                                    }
                                    s++;
                                }
                            }

                            King => {
                                if matches!(get_piece(]i+1,j+1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i+1,j+1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i+1,j+1]});
                                }

                                if matches!(get_piece(]i+1,j]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i+1,j]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i+1,j]});
                                }

                                if matches!(get_piece(]i,j+1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i,j+1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i,j+1]});
                                }

                                if matches!(get_piece(]i-1,j-1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i-1,j-1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i-1,j-1]});
                                }

                                if matches!(get_piece(]i-1,j]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i-1,j]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i-1,j]});
                                }

                                if matches!(get_piece(]i,j-1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i,j-1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i,j-1]});
                                }

                                if matches!(get_piece(]i+1,j-1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i+1,j-1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i+1,j-1]});
                                }

                                if matches!(get_piece(]i-1,j+1]), Some(ColoredPiece{color, ..})) {
                                    try_add_capture_move(c, Move {from: [i,j], to: [i-1,j+1]});
                                }
                                else {
                                    add_move(Move {from: [i,j], to: [i-1,j+1]});
                                }
                            }
                        }
                    },
                    _ => {},
                }
            }     
        }
        vec
    }

    pub fn in_bounds(pos: [i8;2]) -> bool {
        matches!(pos, [0..=7,0..=7])
    }
}



