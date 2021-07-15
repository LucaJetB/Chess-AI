#[derive(Debug, Clone)]
pub struct Board {
    pub arr: [[Option<ColoredPiece>; 8]; 8],
    //who's move
    //who can castle
    
}


impl Board {

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
       fen
   } 

   pub fn from_fen(fen_str: String) -> Self {
       let fen = String::new();
       let counter: i8 = 0;
       let mut b = [[None; 8]; 8];
       let mut x: usize = 0;
       let mut y: usize = 0; 
       for c in fen_str.chars() {
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
       Self {arr: b}
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

   pub fn from_str(sboard: &str) -> Self {
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
   pub fn print(&self) {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColoredPiece {
    pub color: Color,
    pub piece: Piece,
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

            _ =>  {
                println!("{}", c);
                panic!("You fucked up")
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
/* 
impl Move {
    pub fn from_str(m: &str) -> Move { //e3f6
        if let [x1, y1, x2, y2] = *m.chars().collect::<Vec<_>>() {

        }
    }
    pub fn char_to_loc(c:char) -> i8 {
        match c {
            
        }
    }
}
*/