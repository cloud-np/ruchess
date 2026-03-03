use crate::pieces::Piece;
use crate::{BitBoard, Square};

pub struct PieceMove {
    pub piece: Piece,
    pub from: Square,
    // All the potential squares it can go
    pub to: BitBoard,
}

impl PieceMove {
    pub const fn get(_piece: Piece, from: Square) -> BitBoard {
        PieceMove::get_pawn(from)
    }

    const fn get_pawn(from: Square) -> BitBoard {
        if let Some(sq) = Square::try_offset(from, 1, 1) {
            return sq.bitboard();
        }
        BitBoard::EMPTY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_moves() {
        let bb = PieceMove::get_pawn(Square::A2);
        println!("!! {:#}", bb);
        assert_eq!(bb, Square::B3.bitboard());
    }
}
