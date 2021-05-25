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

#[derive(Debug)]
struct MoveGenerator<'a> {
    board: &'a Board,
    start_pos: [i8;2],
    piece: ColoredPiece,
    // add moves in here
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


    fn get_pawnmoves(&self, moves: &mut Vec<Move>, board: &'a Board) {
        let (pawn_start, dir): (i8, i8) = match piece.color {
            White => (6,-1),
            Black => (1,1),
        };
        if self.start_pos[1] == pawn_start {
            if !matches!(board.arr[self.start_pos[0]][3], Some(ColoredPiece{})) {
                if !matches!(board.arr[self.start_pos[0]][4], Some(ColoredPiece{})) {
                    let m = Move {
                        from: self.start_pos,
                        to: [self.start_pos[0]][self.start_pos[1] + 2*dir],
                    };
                    moves.push(m)
                }
            }
        }
        else {
            if !matches!(board.arr[self.start_pos[0]][self.start_pos[1] + dir], Some(ColoredPiece{})) {
                let m = Move {
                    from: self.start_pos,
                    to: [self.start_pos[0]][self.start_pos[1] + dir],
                };
                if in_bounds([self.start_pos[0], self.start_pos[1] + dir]) {
                    moves.push(m)
                }
            }
        }
        if !matches!(board.arr[self.start_pos[0] + 1][self.start_pos[1] + dir], Some(ColoredPiece{color})) {
            if color != self.piece.color {
                let m = Move {
                    from: self.start_pos,
                    to: [self.start_pos[0] + 1][self.start_pos[1] + dir],
                }; 
                moves.push(m)
            }
        }
        else if !matches!(board.arr[self.start_pos[0] - 1][self.start_pos[1] + dir], Some(ColoredPiece{color})) {
            if color != self.piece.color {
                let m = Move {
                    from: self.start_pos,
                    to: [self.start_pos[0] - 1][self.start_pos[1] + dir],
                }; 
                moves.push(m)
            }
        }
    }

    fn get_knightmoves(&self, moves: &mut Vec<Moves>, board: &'a Board) {
        let xvals = [2,2,1,1,-1,-1,-2,-2];
        let yvals = [1,-1,2,-2,2,-2,1,-1];
        for i in 0..7 {
            let m = Move {
                from: self.start_pos,
                to: [self.start_pos[0] + xvals[i]][self.start_pos[1] + yvals[i]],
            };

            if in_bounds([self.start_pos[0] + xvals[i], self.start_pos[1] + yvals[i]]) {
                if matches!(board.arr[self.start_pos[0] + xvals[i]][self.start_pos[1] + yvals[i]], Some(ColoredPiece{color})) {
                    if color != self.piece.color {

                        moves.push(m)
                    }
                }
                else {
                    moves.push(m)
                }
            }
        }
    }

    fn get_linemoves(&self, moves: &mut Vec<[i8;2]>, dirs: &[[i8;2]]) {
        use DesitinationState::*;
        for dir in dirs {
            for i in 1..7 {
                let dst = [self.start_pos[0] + (i*dir[0]), self.start_pos[1] + (i*dir[1])];
                match self.get_dst_state(dst) {
                    Free => moves.push(dst),
                    Occupied | OutOfBounds => break,
                    Capturable => {
                        moves.push(dst);
                        break
                    }
                }
            }
        }
    }
    //for changing operators pass parameters as arrays or could pass lamdas

    fn get_bishopmoves(&self, moves: &mut Vec<[i8;2]>, board: &'a Board) {
        self.get_linemoves(moves, &[[1,1],[-1,1],[-1,-1],[1,-1]])
    }

    fn get_rookmoves(&self, moves: &mut Vec<[i8;2]>, board: &'a Board) {
        self.get_linemoves(moves ,&[[1,0],[-1,0],[0,-1],[0,1]])
    }

    fn get_queenmoves(&self, moves: &mut Vec<[i8;2]>, board: &'a Board) {
        self.get_linemoves(moves ,&[[1,0],[-1,0],[0,-1],[0,1],[1,1],[-1,1],[-1,-1],[1,-1]])
    }

    fn get_piece_moves(&self, moves: &mut Vec<[i8;2]>, board: &'a Board) {
         match self.piece {
            Pawn => self.get_pawnmoves(&self, &mut moves, & board),
            Knight => self.get_knightmoves(&self, &mut moves, & board),
            Bishop => self.get_bishopmoves(&self, &mut mobes, & board),
        }
    }
    fn get_moves(c: Color, board: &'a Board) -> Vec<[i8;2]> {
        let mut results = Vec::new();
        for i in 0..8_i8 {
            for j in 0..8_i8 {
                match board.arr[i as usize][j as usize] { //back to match
                    Some(ColoredPiece{color, piece}) if color == c => {
                        let gen = Self {
                            board, 
                            start_pos: [i,j], 
                            piece: ColoredPiece{color, piece},
                        };
                        gen.get_piece_moves(& board, &mut results);
                    }
                    _ => {}
               }
            }
        }
        results // this vec is only for a specific piece, gets overflowed when multiple pieces come into play
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


    pub fn get_moves(&self, c: Color) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        let mut add_move_raw = |c: Color, m: Move, allow_capture: bool| -> bool  { //if true we keep going, false when capturing piece to stop searching 
            if matches!(m.to,[0..=7,0..=7]) {
                match get_piece(m.to) {
                    Some(ColoredPiece{color, ..}) if color != c => {
                        if allow_capture {
                            vec.push(m);
                        }
                        false
                    }
                    Some(_) => {
                        //this will be a defender 
                        false
                    }
                    None => {
                        vec.push(m);
                        true
                    }
                } 
            } 
        };

        let mut add_move = |c: Color, m: Move| {
            add_move_raw(c,m,true)
        };

        let mut get_piece = |pos: [i8; 2]| {
            if matches!(pos, [0..=7,0..=7]) {
                self.arr[i as usize][j as usize]
            } else {
                None
            }
        };

        let mut 


        for i in 0..8_i8{
            for j in 0..8_i8 {
                match self.arr[i as usize][j as usize] {
                    Some(ColoredPiece{color, piece}) if color == c => {
                        match piece {
                            
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
                                    if matches!(get_piece([i-s,j+s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j+s]});
                                        down_left = true;
                                    } 
                                    else if down_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j+s]});
                                    }

                                    if !Self::in_bounds([i-s,j-s]) {
                                        up_left = true;
                                    }
                                    if matches!(get_piece([i-s,j-s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j-s]});
                                        up_left = true;
                                    } 
                                    else if up_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j-s]});
                                    }

                                    if !Self::in_bounds([i+s,j-s]) {
                                        up_right = true;
                                    }
                                    if matches!(get_piece([i+s,j-s]), Some(ColoredPiece{color, ..})) {
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
                                    if matches!(get_piece([i-s,j]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j]});
                                        left = true;
                                    } 
                                    else if left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j]});
                                    }

                                    if !Self::in_bounds([i,j-s]) {
                                        up = true;
                                    }
                                    if matches!(get_piece([i,j-s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i,j-s]});
                                        up = true;
                                    } 
                                    else if up == false {
                                        add_move(Move {from: [i,j], to: [i,j-s]});
                                    }

                                    if !Self::in_bounds([i,j+s]) {
                                        down = true;
                                    }
                                    if matches!(get_piece([i,j+s]), Some(ColoredPiece{color, ..})) {
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
                                    if matches!(get_piece([i-s,j+s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j+s]});
                                        down_left = true;
                                    } 
                                    else if down_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j+s]});
                                    }

                                    if !Self::in_bounds([i-s,j-s]) {
                                        up_left = true;
                                    }
                                    if matches!(get_piece([i-s,j-s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j-s]});
                                        up_left = true;
                                    } 
                                    else if up_left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j-s]});
                                    }

                                    if !Self::in_bounds([i+s,j-s]) {
                                        up_right = true;
                                    }
                                    if matches!(get_piece([i+s,j-s]), Some(ColoredPiece{color, ..})) {
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
                                    if matches!(get_piece([i-s,j]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i-s,j]});
                                        left = true;
                                    } 
                                    else if left == false {
                                        add_move(Move {from: [i,j], to: [i-s,j]});
                                    }

                                    if !Self::in_bounds([i,j-s]) {
                                        up = true;
                                    }
                                    if matches!(get_piece([i,j-s]), Some(ColoredPiece{color, ..})) {
                                        try_add_capture_move(c, Move {from: [i,j], to: [i,j-s]});
                                        up = true;
                                    } 
                                    else if up == false {
                                        add_move(Move {from: [i,j], to: [i,j-s]});
                                    }

                                    if !Self::in_bounds([i,j+s]) {
                                        down = true;
                                    }
                                    if matches!(get_piece([i,j+s]), Some(ColoredPiece{color, ..})) {
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
                                for x in -1..1 {
                                    for y in -1..1 {
                                        if x == 0 && y == 0 {
                                            continue;
                                        }
                                        if matches!(get_piece([i+1,j+1]), Some(ColoredPiece{color, ..})) {
                                            try_add_capture_move(c, Move {from: [i,j], to: [i+x,j+y]});
                                        }
                                        else {
                                            add_move(Move {from: [i,j], to: [i+x,j+y]});
                                        }
                                    }
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

}



