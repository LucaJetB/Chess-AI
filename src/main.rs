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
    b1.get_advantage();
    println!("");
    println!("Board 2: ");
    b2.get_advantage();
    println!("");
    b1.print();
    for m in MoveGenerator::get_moves(White, &b1) {
        let mut b = b1.clone();
        let p = b.get(m.src);
        b.set(m.dst, p);
        b.set(m.src, None);
        b.print();
        println!("");
    }
    
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
    fn to_char(&self) -> char {
        match (self.color,self.piece) {
            (White, Pawn) => 'P',
            (White, Knight) => 'N',
            (White, Bishop) => 'B',
            (White, Rook) => 'R',
            (White, Queen) => 'Q',
            (White, King) => 'K',
            (Black, Pawn) => 'p',
            (Black, Knight) => 'n',
            (Black, Bishop) => 'b',
            (Black, Rook) => 'r',
            (Black, Queen) => 'q',
            (Black, King) => 'k',
        }
    }


}
 // create pos as a struct or tuple for easy arithmetic funcs on it
 //
 #[derive(Debug)]
struct Move {
    src: [i8; 2],
    dst: [i8; 2],
}


#[derive(Debug)]
struct MoveGenerator<'a> {
    board: &'a Board,
    start_pos: [i8;2],
    piece: ColoredPiece,
    moves: &'a mut Vec<Move>, //if this contains all moves should it be in this struct with piece specfic values?
}

enum DesitinationState {
    Free,
    Occupied,
    Capturable,
    OutOfBounds,
}

impl<'a> MoveGenerator<'a> {
    fn add_move(&mut self, dst: [i8;2]) {
        let m = Move {
            src: self.start_pos,
            dst
        };
        self.moves.push(m);
    }

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
        use DesitinationState::*;
        let (pawn_start, dir): (i8, i8) = match self.piece.color {
            White => (6,-1),
            Black => (1, 1),
        };
        let dst = [self.start_pos[0] + (1*dir), self.start_pos[1]];
        if matches!(self.get_dst_state(dst), Free) {
            self.add_move(dst);
            if self.start_pos[1] == pawn_start {
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

    fn get_knightmoves(&mut self) {
        use DesitinationState::*;
        let xvals = [2,2,1,1,-1,-1,-2,-2];
        let yvals = [1,-1,2,-2,2,-2,1,-1];
        for i in 0..7 {
            let dst = [self.start_pos[0] + xvals[i], self.start_pos[1] + yvals[i]];
            if matches!(self.get_dst_state(dst), Free | Capturable) {
                self.add_move(dst)
            }
        }
    }


    fn get_linemoves(&mut self, dirs: &[[i8;2]]) {
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
            if matches!(self.get_dst_state(dst), Free | Capturable) {
                self.add_move(dst);
            }
        }
        //castling (check under attack sqaures)
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
    
    fn get_moves(c: Color, board: &'a Board) -> Vec<Move> {
        movegen_get_moves(c,board)
    }
}
// TODO prune some illegal moves with castling (only going to be 1 square else lose the game) 
fn movegen_get_moves(c: Color, board: &Board) -> Vec<Move> { 
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

#[derive(Debug, Clone)]
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
    fn print(&self) {
        for i in 0..8 {
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(p) => print!("{} ", p.to_char()),
                    None => print!(". "),
                }
            }
            println!()
        }        
    }

    pub fn in_bounds(pos: [i8;2]) -> bool {
        matches!(pos, [0..=7,0..=7])
    }

    pub fn get(&self, pos: [i8; 2]) -> Option<ColoredPiece> {
        self.arr[pos[0] as usize][pos[1] as usize]
    }
    pub fn set(&mut self, pos: [i8; 2], p: Option<ColoredPiece>) {
        self.arr[pos[0] as usize][pos[1] as usize] = p;
    }

    fn get_advantage(&self) {
        let mut white_score = 0;
        let mut black_score = 0;
        for i in 0..8{
            for j in 0..8 {
                match self.arr[i][j] {
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