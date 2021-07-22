#[derive(Debug, Clone)]
pub struct Board {
    pub arr: [[Option<ColoredPiece>; 8]; 8],
    pub m: Color,
    //who can castle? everyone
    //en passant available?
    
}


impl Board {

    pub fn play_move(mut self, m: Move) -> Self {
        let p = self.get(m.src);
        self.set(m.dst, p);
        self.set(m.src, None);
        self
    }
    pub fn new_board() -> Self {
        Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
    }

    pub fn parse_fen(s: &String) -> Self {
        let pos = first_word(&s);
        let fen_str = &s[1..pos-1];
        let fen = String::from(fen_str);
        Board::from_fen(fen)
    }

    pub fn to_fen(&self) -> String {
       let mut fen = String::new();
       let mut counter: i8 = 0;
       for i in 0..8 {
           for j in 0..8 {
               match self.arr[i][j] {
                   Some(p) =>  {
                       if counter != 0 {
                            fen.push_str(&counter.to_string());
                            counter = 0;
                       }
                       fen.push_str(p.to_str())
                   }
                   None => {
                    counter += 1;
                   }
               }
           }
           if counter != 0 {
               fen.push_str(&counter.to_string());
               counter = 0;
           }
           fen.push_str("/");
       }
       fen.push_str(" ");
       fen.push_str(self.m.to_str());
       fen.push_str(" - -");
       fen
    } 

    pub fn from_fen(fen_str: String) -> Self {
       let fen = String::new();
       let counter: i8 = 0;
       let mut b = [[None; 8]; 8];
       let mut x: usize = 0;
       let mut y: usize = 0; 
       let mut m: Color = White;
       let mut end = false;
       for c in fen_str.chars() {
           if c == ' ' {
               end = true;
               continue;
           }
           if c == 'w' && end  {
               m = White;
               break;
           }
           if c == 'b' && end {
               m = Black;
               break;
           }
           if c == '/' {
               y = y+1;
               x = 0;
               continue;
           }
           if 8 >= c as i32 - 0x30 && 0 <= c as i32 - 0x30  {
                for i in 0..c as i32 - 0x30 {
                    b[y][x] = None;
                    x = x + 1;
                }
           }
           else {
               b[y][x] = Some(ColoredPiece::from_char(c));
               x = x + 1;
           }
       }

       Self {arr: b, m}
    }


    pub fn find_piece(&self, look: ColoredPiece) -> bool{
       for i in 0..8 {
           for j in 0..8 {
               match self.arr[i][j] {
                   Some(p) => {
                       if p.to_char() == look.to_char() {
                           return true;
                       }
                   }
                   None => {},
               }
               }
       }
       false
    }

    pub fn from_str(sboard: &str, c: Color) -> Self {
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
       Self {arr: board, m: c}
    }

   pub fn from_str_piece(sboard: &str, c: Color) -> Self {
    let sboard: Vec<_> = sboard.chars().collect();
    let mut board = [[None; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            let index = 16*i + 2*j;
            if sboard[index] != '.' {
                let s = sboard[index].to_string();
                let p = &s[..];
                println!("{}", p);
                board[i][j] = Some(ColoredPiece::from_piece_str(p))
            }
        }
    }
    Self {arr: board, m: c}
}
   pub fn print(&self) {
       for i in 0..8 {
           for j in 0..8 {
               match self.arr[i][j] {
                   Some(p) => print!("{} ", p.to_piece()),
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
   //evaluating position as well as material such as controlling the center, having centralized pieces, pieces on your opponents side of the board
   pub fn get_advantage_print(&self, c: Color) -> i32 { //posibly could go to floating points
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

       if c == White {
           white_score - black_score
       }
       else {
           black_score - white_score
       }
   }
   
   pub fn get_advantage(&self, c: Color) -> i32 { //posibly could go to floating points
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
       if c == White {
           white_score - black_score
       }
       else {
           black_score - white_score
       }
   }

}

use Color::*; 
use Piece::*;

use crate::first_word;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColoredPiece {
    pub color: Color,
    pub piece: Piece,
}
/*                                                      8  ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜'
<UciProtocol (pid=35959)>: Unexpected engine output: '  7  ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎'
<UciProtocol (pid=35959)>: Unexpected engine output: '  6  . . . . . . . .'
<UciProtocol (pid=35959)>: Unexpected engine output: '  5  . . . . . . . .'
<UciProtocol (pid=35959)>: Unexpected engine output: '  4  . . . . . . . .'
<UciProtocol (pid=35959)>: Unexpected engine output: '  3  . . . . . . . .'
<UciProtocol (pid=35959)>: Unexpected engine output: '  2  ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙'
<UciProtocol (pid=35959)>: Unexpected engine output: '  1  ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖'
 */

impl ColoredPiece {

    pub fn to_piece(&self) -> &str {
        match (self.color, self.piece) {
            (White, Pawn) => "♟︎",
            (White, Knight) => "♞",
            (White, Bishop) => "♝",
            (White, Rook) => "♜",
            (White, Queen) => "♛",
            (White, King) => "♚",
            (Black, Pawn) => "♙",
            (Black, Knight) => "♘",
            (Black, Bishop) => "♗",
            (Black, Rook) => "♖",
            (Black, Queen) => "♕",
            (Black, King) => "♔",
        }
    }
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

            _ =>  {
                println!("{}", c);
                panic!("You fucked up")
            }
        };
        Self {color, piece}
    }

    pub fn from_piece_str(p: &str) -> Self {
        let (color,piece, value) = match p {
            "♟︎" => (White, Pawn, 1),
            "♞" => (White, Knight, 3),
            "♝" => (White, Bishop, 3),
            "♜" => (White, Rook, 5),
            "♛" => (White, Queen, 9),
            "♚" => (White, King, 100),

            "♙" => (Black, Pawn, 1),
            "♘" => (Black, Knight, 3),
            "♗" => (Black, Bishop, 3),
            "♖" => (Black, Rook, 5),
            "♕" => (Black, Queen, 9),
            "♔" => (Black, King, 100),

            _ =>  {
                println!("{}", p);
                panic!("You fucked up 2")
            }
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
    fn to_str(&self) -> &str {
        match (self.color,self.piece) {
            (White, Pawn) => "P",
            (White, Knight) => "N",
            (White, Bishop) => "B",
            (White, Rook) => "R",
            (White, Queen) => "Q",
            (White, King) => "K",
            (Black, Pawn) => "p",
            (Black, Knight) => "n",
            (Black, Bishop) => "b",
            (Black, Rook) => "r",
            (Black, Queen) => "q",
            (Black, King) => "k",
        }
    }


}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
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
    pub fn to_str(&self) -> &str {
        match self {
            White => "w",
            Black => "b",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
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



#[derive(Debug)]
pub struct Move {
    pub src: [i8; 2],
    pub dst: [i8; 2],
}

impl Move {
    pub fn print(&self) {
        println!("({},{}) ({},{})", self.src[0], self.src[1], self.dst[0], self.dst[1]);
    }
    pub fn same_as(&self, m: &Move) -> bool {
        if self.src == m.src {
            if self.dst == m.dst {
                return true
            }
        }
        false
    }
    pub fn parse_move_str(m: &str) -> [i8;2] {
        let mut arr: [i8;2] = [-1,-1];
        match &m[0..1] { 
            "a" => arr[1] = 0,
            "b" => arr[1] = 1,
            "c" => arr[1] = 2,
            "d" => arr[1] = 3,
            "e" => arr[1] = 4,
            "f" => arr[1] = 5,
            "g" => arr[1] = 6,
            "h" => arr[1] = 7,
            _ => panic!("invalid move"),
        }
        let num: i8 = m[1..2].parse().unwrap();
        arr[0] = num-1;
        arr
    }

    pub fn chess_notation_to_move(m: &str) -> Self {
        let first_half = &m[0..2];
        let second_half = &m[2..4];
        let s = Move::parse_move_str(first_half);
        let d = Move::parse_move_str(second_half);
        return Self {src: s, dst: d}
    }

    pub fn parse_moves(m: String) -> Vec<Self> {
        println!("{}", m);
        let s = &m[..];
        let v: Vec<&str> = s.split(' ').collect();
        let mut moves: Vec<Move> = Vec::new();
        for i in v {
            //Move::chess_notation_to_move(s).print();
            moves.push(Move::chess_notation_to_move(s));
        }
        moves
    }
}
