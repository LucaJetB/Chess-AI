use std::fmt;
use std::convert::From;
use Color::*;
use Piece::*;
use DestinationState::*;
use crate::{engine::MoveEvaluator, move_generator::{DestinationState, MoveGenerator}};

#[derive(Debug, Clone)]
pub struct Board {
    pub arr: [[Option<ColoredPiece>; 8]; 8],
    /// Used to dtermine whos move it is.
    pub m: Color, 
    pub last_move: Option<Move>,
    pub castling: CastlingAbility,
    pub castled: bool,
    //who can castle? everyone
    //en passant available?
}

pub fn add(arr1: [i8; 2], arr2: [i8; 2]) -> [i8; 2] {
    [arr1[0] + arr2[0], arr1[1] + arr2[1]]
}

impl Board {

    pub fn get_pawn_value(&self, pos: [i8;2], c: Color) -> i32{
        let mut value = 0;
        let mut y = pos[0];
        let mut x = pos[1];
        let mut passed_pawn = true;
        let mut dir = 0;
        if c == White {
            dir = -1;
        } else {
            dir = 1;
        }
        // Checks if pawn is a passed pawn by looking from its pos "up" on its three rows for opposite pawn
        while y > 0 && y < 7 {
            for i in -1..1 {
                if i == -1 && x == 0 {
                    x = x;
                }
                else {
                    x = x + i;
                }
                match self.arr[y as usize][x as usize] {
                    Some(ColoredPiece{color , piece}) => {
                        if piece == Pawn && color == c.opposite_color() {
                            passed_pawn = false;
                        }

                    },
                    None => {},
                }

            }
            y += dir;
        }
        // Pawn value increases as it moves up the board
        // They also become important in the endgame
        // Extra points assigned if its a passed pawn 
        let mut is_endgame = false;
        if self.count_pieces() < 12 {
            is_endgame = true;
        }
        if c == Color::White {
            value += (7-pos[0] * 10) as i32;

        }
        else {
            value += (pos[0] * 10) as i32;
        }

        if is_endgame {
            value += 100;
        }

        if passed_pawn {
            value *= 2;
        }
        value
    }
    pub fn get_relative_value(&self, p: ColoredPiece, pos: [i8;2]) -> i32 {
        let mut value = 0;
        // Value of piece
        value += p.get_value();

        if p.piece == Pawn {
            value += self.get_pawn_value(pos, p.color);
        }


        let (attackers,num_attackers) = self.is_attacked(pos);
        let (_, num_defenders) = self.is_defended(pos);
        // Pieces that are fortifed and take up opponents attacks are somewhat valuable
        if num_attackers > num_defenders {
            value += p.get_value() / 10;
        }
        // Hanging pieces are worthless unless moved or defended.
        if num_attackers > 0 && num_defenders == 0 {
            value = 0;
        }
        // If the piece is capturable by something worth less than it, it's hanging
        // And can only be worth as much as its least valuable attacker
        else if num_attackers < num_defenders {
            for attacker in attackers {
                if attacker.get_value() < p.get_value() {
                    value = attacker.get_value();
                    return value;
                }
            }
        }

        // The more space a player has the better
        // Open files, connected rooks
        // Pinned pieces
        // Open kings
        value
    }


    pub fn evaluate(&self) -> i32 { 
        let mut white_score = 0;
        let mut black_score = 0;
        let mut white_bishop_pair = 0;
        let mut black_bishop_pair = 0;
        for i in 0..8 {
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(p) => {
                        if p.color == White {
                            white_score += self.get_relative_value(p, [i as i8,j as i8]);
                            if p.piece == Bishop {
                                white_bishop_pair += 1;
                            }
                            if white_bishop_pair == 2 {
                                white_score += 10;
                            }
                        }
                        else {
                            black_score -= self.get_relative_value(p, [i as i8,j as i8]);
                            if p.piece == Bishop {
                                black_bishop_pair += 1;
                            }
                            if black_bishop_pair == 2 {
                                black_score += 10;
                            }
                        }
                    }
                    None =>  {},
                }
            }
        }

        white_score + black_score
    }

    pub fn count_pieces(&self) -> i32 {
        let mut count = 0;
        for i in 0..8 {
            for j in 0..8 {
                if matches!(self.arr[i][j], Some(_)) {
                    count += 1;
                }
            }
        }
        count
    }

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
                let (_,attackers) = self.is_attacked(m.dst);
                if attackers > 0 {
                    attacked_pieces.push(p.piece);
                }
            }
        }
        attacked_pieces
    }
    
    pub fn is_attacked(&self, pos: [i8;2]) -> (Vec<Piece>,i32) {
        let mut pieces = Vec::new();
        let mut attackers = 0;
        let p = self.get(pos).unwrap();
        // Searches for only moves by the opponent that attack one of the players pieces
        for m in MoveGenerator::get_moves_with_predicates(&self, p.color.opposite_color(), 
            |_| true, &|dst_state| dst_state == DestinationState::Capturable) {
            if m.dst == pos {
                pieces.push(self.get(m.src).unwrap().piece);
                attackers += 1;
            }
        }
        (pieces,attackers)
    }
    
    pub fn is_defended(&self, pos:[i8;2]) -> (Vec<Piece>,i32) {
        let mut pieces = Vec::new();
        let mut defenders = 0;
        let p = self.get(pos).unwrap();
        // Searches only for the players defending "moves"
        for m in MoveGenerator::get_moves_with_predicates(&self, p.color, 
            |_| true, &|dst_state| dst_state == DestinationState::Occupied) {
            if m.dst == pos {
                pieces.push(self.get(m.src).unwrap().piece);
                defenders += 1;
            }
        }
        (pieces,defenders)
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
                self.castled = true;
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
                castled: false,
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
        Self {arr: board, m: c, last_move: None, castling: Default::default(), castled: false}
    }
    #[cfg(test)]
    pub fn from_str_piece(sboard: &str, c: Color) -> Self {
        let sboard: Vec<_> = sboard.chars().collect();
        let mut board = [[None; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                let index = 16 * i + 2 * j;
                if sboard[index] != '.' {
                    let s = sboard[index].to_string();
                    let p = &s[..];
                    board[i][j] = Some(ColoredPiece::from_piece_str(p))
                }
            }
        }
        Self {arr: board, m: c, last_move: None, castling: Default::default(), castled: false}
    }

    

    pub fn print(&self) { // TODO add file and rank.
        println!("  0 1 2 3 4 5 6 7");
        for i in 0..8 {
            print!("{} ", i);
            for j in 0..8 {
                match self.arr[i][j] {
                    Some(p) => print!("{} ", p.to_piece()),
                    None => print!(". "),
                }
            }
            print!("{}", i);
            println!()
        }
        println!("  0 1 2 3 4 5 6 7");
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
    #[cfg(test)]
    pub fn from_piece_str(p: &str) -> Self {
        let (color, piece) = match p {
            "♟" => (White, Pawn),
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
                panic!("Wrong piece char '{}'", p);
            }
        };
        Self { color, piece }
    }

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub src: [i8; 2],
    pub dst: [i8; 2],
    pub dst_state: DestinationState,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}) ({},{})", self.src[0], self.src[1], self.dst[0], self.dst[1])
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}) ({},{})", self.src[0], self.src[1], self.dst[0], self.dst[1])
    }
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
