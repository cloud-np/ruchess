use crate::{Square, bitboards::BitBoard, pieces::Piece};
use ruchess_helpers::simple_error;

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0";

simple_error! {
    pub enum ParseFenError {
        TooManyParts = "FEN string has too many parts, expected at most 6"
    }
}

pub fn parse_fen(fen: &str) -> Result<(), ParseFenError> {
    let seperated_fen: Vec<&str> = fen.split_whitespace().collect();

    // must have 6 parts :
    // [ Piece Placement, Side to Move, Castling Ability, En Passant square, Half moves, full moves]
    if seperated_fen.len() > 6 {
        return Err(ParseFenError::TooManyParts);
    }

    let mut pos: u8 = 0;
    let mut pieces: BitBoard = BitBoard::EMPTY;
    for c in seperated_fen.iter().next().unwrap().chars() {
        let piece_type = Piece::try_from(c.to_ascii_lowercase());
        if piece_type.is_ok() {
            pieces = pieces | Square::index_const(pos as usize).bitboard();
        }
        if c != '/' {
            pos += c.to_digit(10).unwrap_or(1) as u8;
        }
        if c == ' ' {
            break;
        }
    }
    println!("\n{:#}", pieces);
    Ok(())
}
