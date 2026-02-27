use crate::{bitboards::BitBoard, pieces::Piece};

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0";

pub fn parse_fen(fen: &str) {
    let mut pos: u8 = 0;
    let mut blacks: BitBoard = BitBoard::EMPTY;
    for c in fen.chars() {
        let piece_type = Piece::char_to_piece(c);
        blacks = blacks << BitBoard(1 << pos);
        print!("{}", piece_type);
        pos += c.to_digit(10).unwrap_or(1) as u8;
        break;
    }
    println!("\npos: {}, {:?}", pos, blacks);
    todo!()
}
