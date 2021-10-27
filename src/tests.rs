#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::engine::*;
    use Color::*; 

    #[test] 
    fn test_basic1() {
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
        let board3 = 
        ". . . . . . . .\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        . . . q . b . .\n\
        . n . . P . . .\n\
        P . . . . . . .\n\
        . . . . . . . .\n\
        ";
        let _board4 = 
        "♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖\n\
        ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        . . . . . . . .\n\
        ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟\n\
        ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜\n\
        ";
        let b2 = Board::from_str(board2, White);
        let _b3 = Board::from_str(board3, White);
        let _b4 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        play(&b2,White, true);
        /*
        b4.print();
        let f = b1.to_fen();
        println!("{}", f);
        dbg!(&b1);
        assert_eq!(b1.arr[0][6], Some(ColoredPiece{color: Black, piece: King}));
        println!("");
        b1.print();
        let best_move = get_best_move(&b1, Black, 4);
        Move::print_option(best_move);
        println!("\n");
        let fen = b2.to_fen();
        println!("{}", fen);
        let best_board = get_best_move(&b1, White, 2);
        let from_fen = Board::from_fen(&fen);
        from_fen.print();
        */
    }

    #[test] 
    fn test_engine() {
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
            ". . . . . . k .\n\
            . . . . . Q p p\n\
            . . . . . . b .\n\
            . . . . n . . .\n\
            . . . . . . . .\n\
            . . . . . . . .\n\
            P . . . . P . .\n\
            . . . . . . K .\n\
            ";
            
        let b1 = Board::from_str(board1, White);
        let b2 = Board::from_str(board2, White);
        /*
        let m = get_best_move(&b1, White, 2);
        if let Some(m) = m {
            m.print();
        }
        */
        let m2 = get_best_move(&b2, Black, 4);
        dbg!(m2);
        let m1 = get_best_move(&b1, White, 4);
        dbg!(m1);
        let (_, n) = b1.is_defended([1,5]);
        println!("{}",n);

    }

    #[test] 
    // TODO Simple boards where we know outcomes (finding basic checkmates)
    // Expand book
    // A board per test
    fn test_bad_rook_move() {
        let board = 
       ". . . . . . ♔ .\n\
        . . . . . . ♙ .\n\
        ♙ . . ♟ . . . .\n\
        . ♙ . . . . ♙ .\n\
        . . ♟ . . ♙ . ♟\n\
        . . ♖ . . . . .\n\
        ♖ . . . . . . .\n\
        . ♚ . . . . . .\n\
        ";

        let mut b = Board::from_str_piece(board, Black);
        b.castled = true;
        b.print();
        let (m,_) = get_best_move(&b, Black, 3).unwrap();
        dbg!(m);
        play(&b, Black, false);
    }
}