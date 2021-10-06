use std::fmt;
use std::convert::From;
use Color::*;
use Piece::*;
use DestinationState::*;
use crate::{move_generator::{DestinationState, MoveGenerator}};

#[derive(Debug, Clone)]
pub struct Board {
    pub arr: [[Option<ColoredPiece>; 8]; 8],
    pub m: Color,
    pub last_move: Option<Move>,
    pub castling: CastlingAbility,
    //who can castle? everyone
    //en passant available?
}

pub fn add(arr1: [i8; 2], arr2: [i8; 2]) -> [i8; 2] {
    [arr1[0] + arr2[0], arr1[1] + arr2[1]]
}

impl Board {

    pub fn get_larger_center() -> Vec<[i8;2]> {
        let mut spaces = Vec::new();
        for i in 2..6 {
            for j in 2..6{ 
                spaces.push([i,j]);
            }
        }
        spaces
    }
    pub fn get_smaller_center() -> Vec<[i8;2]> {
        let mut spaces = Vec::new();
        for i in 3..5 {
            for j in 3..5{ 
                spaces.push([i,j]);
            }
        }
        spaces
    }

    pub fn get_attacked_pieces(&self, attacker_color: Color) -> Vec<Piece> {
        let mut attacked_pieces = Vec::new();
        for m in MoveGenerator::get_moves(self, attacker_color) {
            let p = self.get(m.dst);
            if let Some(p) = p {
                let (a,_) = self.is_attacked(m.dst);
                if a {
                    attacked_pieces.push(p.piece);
                }
            }
        }
        attacked_pieces
    }
    
    pub fn is_attacked(&self, pos: [i8;2]) -> (bool,i32) {
        let mut b = false;
        let mut attackers = 0;
        let p = self.get(pos).unwrap();
        for m in MoveGenerator::get_moves(&self, p.color.opposite_color()) {
            if m.dst == pos {
                b = true;
                attackers += 1;
            }
        }
        (b,attackers)
    }
    
    pub fn is_defended(&self, pos:[i8;2]) -> (bool,i32) {
        let mut b = false;
        let mut defenders = 0;
        let p = self.get(pos).unwrap();
        for m in MoveGenerator::get_defense_moves(&self, p.color) {
            if m.dst == pos {
                b = true;
                defenders += 1;
            }
        }
        (b,defenders)
    }
    

    pub fn print_new_game() {
        Board::new().print();
    }
    
    pub fn get_piece_loc(&self, piece: ColoredPiece) -> Option<[i8; 2]> {
        for i in 0..8 {
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(p) => {
                        if p.piece == piece.piece {
                            if p.color == piece.color {
                                return Some([i as i8, j as i8]);
                            }
                        }
                    }
                    None => return None,
                }
            }
        }
        None
    }

    pub fn find_castle_rook_move(&mut self, king_move: Move) -> Move {
        //hardcode example
        match (king_move.src, king_move.dst) {
            ([0, 4], [0, 6]) => Move {
                src: [0, 7],
                dst: [0, 5],
                dst_state: Free,
            },
            ([0, 4], [0, 2]) => Move {
                src: [0, 0],
                dst: [0, 3],
                dst_state: Free,
            },
            ([7, 4], [7, 6]) => Move {
                src: [7, 7],
                dst: [7, 5],
                dst_state: Free,
            },
            ([7, 4], [7, 2]) => Move {
                src: [7, 0],
                dst: [7, 3],
                dst_state: Free,
            },
            _ => panic!("Not a castle move"),
        }
    }
    pub fn play_move(&mut self, m: Move) { //possibly swap the board so you're always looking from the players perspective
        let p = self.get(m.src);
        //castling
        if matches!(p, Some(ColoredPiece {color:_, piece: King })) {
            if (m.src[1] - m.dst[1]).abs() == 2 {
                let rook = self.find_castle_rook_move(m);
                if rook.src[0] == -1 {
                    return;
                }
                let r = self.get(rook.src);
                self.set(m.dst, p);
                self.set(m.src, None);
                self.last_move = Some(m);
                self.set(rook.dst, r);
                self.set(rook.src, None);
                self.last_move = Some(m);
                return;
            }
        }
        //promotion
        if matches!(p, Some(ColoredPiece {color:_, piece: Pawn})) {
            if let Some(p) = p {
                if p.color == White {
                    if m.dst[0] == 0 {
                        let promote = ColoredPiece {
                            color: White,
                            piece: Queen,
                        };
                        self.set(m.dst, Some(promote));
                        self.set(m.src, None);
                        self.last_move = Some(m);
                        return;
                    }
                } else if p.color == Black {
                    if m.dst[0] == 7 {
                        let promote = ColoredPiece {
                            color: Black,
                            piece: Queen,
                        };
                        self.set(m.dst, Some(promote));
                        self.set(m.src, None);
                        self.last_move = Some(m);
                        return;
                    }
                }
            }
        }

        self.set(m.dst, p);
        self.set(m.src, None);
        self.last_move = Some(m);
    }

    pub fn new() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut counter: i8 = 0;
        for i in 0..8 {
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(p) => {
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
        fen.push_str(" ");
        fen.push_str(&self.castling.to_string());
        fen
    }
    

    pub fn from_fen(fen_str: &str) -> Self {
        let mut arr = [[None; 8]; 8];
        if let [pos, color, castling, _, _, _] = *fen_str.split_whitespace().collect::<Vec<_>>() {
            for (rank, line) in pos.split('/').enumerate() {
                let mut file: usize = 0;
                for c in line.chars() {
                    if let Some(n) = c.to_digit(10) {
                        file += n as usize;
                    } else {
                        arr[rank][file] = Some(ColoredPiece::from_char(c));
                        file += 1;
                    }
                }
            }
            let m = Color::from_str(color);
            let castling = castling.into();
            let last_move = None;
            return Board {
                arr,
                m,
                last_move,
                castling,
            };
        }
        panic!("bad fen");
    }
    /*
    pub fn find_piece(&self, look: ColoredPiece) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(p) => {
                        if p.to_char() == look.to_char() {
                            return true;
                        }
                    }
                    None => {}
                }
            }
        }
        false
    }
    */
    pub fn from_str(sboard: &str, c: Color) -> Self {
        assert!(sboard.len() == 8 * 16);
        let sboard: Vec<_> = sboard.chars().collect();
        let mut board = [[None; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                let index = 16 * i + 2 * j;
                if sboard[index] != '.' {
                    board[i][j] = Some(ColoredPiece::from_char(sboard[index]))
                }
            }
        }
        Self {arr: board, m: c, last_move: None, castling: Default::default()}
    }
    /* 
    pub fn from_str_piece(sboard: &str, c: Color) -> Self {
        let sboard: Vec<_> = sboard.chars().collect();
        let mut board = [[None; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                let index = 16 * i + 2 * j;
                if sboard[index] != '.' {
                    let s = sboard[index].to_string();
                    let p = &s[..];
                    println!("{}", p);
                    board[i][j] = Some(ColoredPiece::from_piece_str(p))
                }
            }
        }
        Self {arr: board, m: c, last_move: None, castling: Default::default()}
    }
    */
    

    pub fn print(&self) { //add file and rank.
        for i in 0..8 {
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(p) => print!("{} ", p.to_piece()),
                    None => print!(". "),
                }
            }
            println!()
        }
        println!("Side: {}", self.m.to_str());
    }

    pub fn in_bounds(pos: [i8; 2]) -> bool {
        matches!(pos, [0..=7, 0..=7])
    }

    pub fn get(&self, pos: [i8; 2]) -> Option<ColoredPiece> {
        self.arr[pos[0] as usize][pos[1] as usize]
    }
    pub fn set(&mut self, pos: [i8; 2], p: Option<ColoredPiece>) {
        self.arr[pos[0] as usize][pos[1] as usize] = p;
    }
    //evaluating position as well as material such as controlling the center, having centralized pieces, pieces on your opponents side of the board
    /* 
    pub fn get_score(&self, c: Color) -> i32 {
        //posibly could go to floating points
        let mut white_score = 0;
        let mut black_score = 0;
        for i in 0..8 {
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(ColoredPiece {color: White, piece}) => white_score += piece.get_value(),
                    Some(ColoredPiece {color: Black, piece}) => black_score += piece.get_value(),
                    None => {}
                }
            }
        }
        if c == White {
            white_score - black_score
        } else {
            black_score - white_score
        }
    }
    */
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColoredPiece {
    pub color: Color,
    pub piece: Piece,
}

impl fmt::Display for ColoredPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.color, self.piece) {
            (White, Pawn) => write!(f, "White Pawn"),
            (White, Knight) => write!(f, "White Knight"),
            (White, Bishop) => write!(f, "White Bishop"),
            (White, Rook) => write!(f, "White Rook"),
            (White, Queen) => write!(f, "White Queen"),
            (White, King) => write!(f, "White King"),
            (Black, Pawn) => write!(f, "Black Pawn"),
            (Black, Knight) => write!(f, "Black Knight"),
            (Black, Bishop) => write!(f, "Black Bishop"),
            (Black, Rook) => write!(f, "Black Rook"),
            (Black, Queen) => write!(f, "Black Queen"),
            (Black, King) => write!(f, "Black King"),
        }
    }
}

impl ColoredPiece {
    pub fn get_value(&self) -> i32 {
        match self.piece {
            Pawn   => 100,
            Knight => 300,
            Bishop => 300,
            Rook   => 500,
            Queen  => 900,
            King   => 9000,
        }
    }

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
        let (color, piece) = match c {
            'P' => (White, Pawn),
            'N' => (White, Knight),
            'B' => (White, Bishop),
            'R' => (White, Rook),
            'Q' => (White, Queen),
            'K' => (White, King),

            'p' => (Black, Pawn),
            'n' => (Black, Knight),
            'b' => (Black, Bishop),
            'r' => (Black, Rook),
            'q' => (Black, Queen),
            'k' => (Black, King),

            _ => {
                println!("{}", c);
                panic!("You fucked up")
            }
        };
        Self { color, piece }
    }
    /* 
    pub fn from_piece_str(p: &str) -> Self {
        let (color, piece) = match p {
            "♟︎" => (White, Pawn),
            "♞" => (White, Knight),
            "♝" => (White, Bishop),
            "♜" => (White, Rook),
            "♛" => (White, Queen),
            "♚" => (White, King),

            "♙" => (Black, Pawn),
            "♘" => (Black, Knight),
            "♗" => (Black, Bishop),
            "♖" => (Black, Rook),
            "♕" => (Black, Queen),
            "♔" => (Black, King),

            _ => {
                println!("{}", p);
                panic!("You fucked up 2")
            }
        };
        Self { color, piece }
    }
    */
    pub fn to_str(&self) -> &str {
        match (self.color, self.piece) {
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

    pub fn from_str(s: &str) -> Self {
        match s {
            "w" => White,
            "b" => Black,
            _ => panic!("not a color: {}", s),
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
            Pawn   => 100,
            Knight => 300,
            Bishop => 300,
            Rook   => 500,
            Queen  => 900,
            King   => 9000,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pawn => write!(f, "Pawn"),
            Knight => write!(f, "Knight"),
            Bishop => write!(f, "Bishop"),
            Rook => write!(f, "Rook"),
            Queen => write!(f, "Queen"),
            King => write!(f, "King"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub src: [i8; 2],
    pub dst: [i8; 2],
    pub dst_state: DestinationState,
}

impl Move {

    pub fn get_dst_state(b: &Board, dst: [i8;2], player_color: Color) -> DestinationState {
        use DestinationState::*;
        if !Board::in_bounds(dst) {
            return OutOfBounds;
        }
        match b.get(dst) {
            Some(ColoredPiece {color, ..}) if color != player_color => Capturable,
            Some(_) => Occupied,
            None => Free,
        }
    }

    pub fn chess_notation_to_move(m: &str) -> Self {
        let first_half = &m[0..2];
        let second_half = &m[2..4];
        let s = Self::parse_move_str(first_half);
        let d = Self::parse_move_str(second_half);
        return Self { src: s, dst: d, dst_state: Free}; //user parsing move
    }

    pub fn is_castle(&self, b: &Board) -> bool {
        let p = b.get(self.src).unwrap();
        if p.piece == King {
            if (self.src[0] - self.dst[0]).abs() == 2 {
                return true;
            }
        }
        false
    }
    /* 
    pub fn print_option(m: Option<Move>) {
        match m {
            Some(m1) => {
                println!(
                    "({},{}) ({},{})",
                    m1.src[0], m1.src[1], m1.dst[0], m1.dst[1]
                );
            }
            None => {
                println!("Error, not a move");
            }
        }
    }
    */
    pub fn print(&self) {
        println!(
            "({},{}) ({},{})",
            self.src[0], self.src[1], self.dst[0], self.dst[1]
        );
    }
    pub fn equal(m1: &Move, m2: &Move) -> bool {
        if m1.to_string() == m2.to_string() {
            return true;
        }
        false
    }
    pub fn parse_move_str(m: &str) -> [i8; 2] {
        let mut arr: [i8; 2] = [-1, -1];
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
        let fix = 8 - num;
        arr[0] = fix;
        arr
    }


    pub fn to_string(&self) -> String {
        let s = format!(
            "({},{}) ({},{})",
            self.src[0], self.src[1], self.dst[0], self.dst[1]
        );
        s
    }

    pub fn to_move_string(&self) -> String {
        fn square_to_move(square: [i8;2]) -> String {
            let mut s = String::new();
            s.push(b"abcdefgh"[square[1] as usize] as char);
            s.push(b"87654321"[square[0] as usize] as char);
            s
        }
        format!("{}{}", square_to_move(self.src), square_to_move(self.dst))
    }
    
}

#[derive(Debug, Clone)]

pub struct CastlingAbility {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Default for CastlingAbility {
    fn default() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

impl fmt::Display for CastlingAbility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.white_kingside  { write!(f, "K")?; }
        if self.white_queenside { write!(f, "Q")?; }
        if self.black_kingside  { write!(f, "k")?; }
        if self.black_queenside { write!(f, "q")?; }  
        Ok(())
    }
}

impl From<&str> for CastlingAbility {
    fn from(s: &str) -> Self {
        let mut c = CastlingAbility {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        };
        if s.contains("K") {
            c.white_kingside = true;
        }
        if s.contains("Q") {
            c.white_queenside = true;
        }
        if s.contains("k") {
            c.black_kingside = true;
        }
        if s.contains("q") {
            c.black_queenside = true;
        }
        c
    }
}
