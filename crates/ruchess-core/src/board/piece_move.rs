use crate::pieces::Piece;
use crate::{BitBoard, Color, File, Rank, Square};

pub struct PieceMove {
    pub piece: Piece,
    pub from: Square,
    // All the potential squares it can go
    pub to: BitBoard,
}

impl PieceMove {
    pub const fn get(_piece: Piece, color: Color, from: Square, blockers: BitBoard) -> BitBoard {
        PieceMove::get_pawn_quiet(color, from, blockers)
    }

    // const OFFSETS: [(i8, i8); 2] = [(1, 1), (-1, 1)];
    // let mut i = 0;
    // while i < OFFSETS.len() {
    //     i += 1;
    //     let (file, rank) = OFFSETS[i];
    //     if let Some(sq) = Square::try_offset(from, file, rank) {
    //         sq.bitboard();
    //     }
    // }
    const fn get_pawn_quiet(color: Color, from: Square, blockers: BitBoard) -> BitBoard {
        let from_bb = from.bitboard().0;

        let single = match color {
            Color::White => from_bb << File::NUM,
            Color::Black => from_bb >> File::NUM,
        };
        let single = single & !blockers.0;

        let double = if Rank::Second.relative_to(color).bitboard().has(from) {
            let d = match color {
                Color::White => single << File::NUM,
                Color::Black => single >> File::NUM,
            };
            d & !blockers.0
        } else {
            0
        };

        BitBoard(single | double)
    }

    const fn get_pawn_attacks(color: Color, from: Square) -> BitBoard {
        const OFFSETS: [[(i8, i8); 2]; Color::NUM] = [[(-1, 1), (1, 1)], [(-1, -1), (1, -1)]];
        let mut moves = BitBoard::EMPTY;
        let mut i = 0;
        while i < OFFSETS[color as usize].len() {
            let (file, rank) = OFFSETS[color as usize][i];
            moves = Self::try_add_move(moves, from, file, rank);
            i += 1;
        }
        moves
    }

    #[inline(always)]
    const fn try_add_move(
        mut moves: BitBoard,
        from: Square,
        file_offset: i8,
        rank_offset: i8,
    ) -> BitBoard {
        if let Some(sq) = Square::try_offset(from, file_offset, rank_offset) {
            moves.0 |= sq.bitboard().0;
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_moves_no_blockers() {
        let bb = PieceMove::get(Piece::Pawn, Color::White, Square::B2, BitBoard::EMPTY);
        assert_eq!(bb, Square::B3.bitboard() | Square::B4.bitboard());
    }

    #[test]
    fn pawn_moves_blockers() {
        let blockers = Square::B4.bitboard() | Square::B1.bitboard();
        let bb = PieceMove::get(Piece::Pawn, Color::White, Square::B2, blockers);
        assert_eq!(bb, Square::B3.bitboard());
    }

    #[test]
    fn pawn_moves_no_invalid_two_step() {
        let blockers = Square::B3.bitboard();
        let bb = PieceMove::get(Piece::Pawn, Color::White, Square::B2, blockers);
        println!("{:#}", bb);
        assert_eq!(bb, BitBoard::EMPTY);
    }

    #[test]
    fn white_pawn_attacks_center() {
        let bb = PieceMove::get_pawn_attacks(Color::White, Square::D4);
        assert_eq!(bb, Square::C5.bitboard() | Square::E5.bitboard());
    }

    #[test]
    fn black_pawn_attacks_center() {
        let bb = PieceMove::get_pawn_attacks(Color::Black, Square::D4);
        assert_eq!(bb, Square::C3.bitboard() | Square::E3.bitboard());
    }

    #[test]
    fn white_pawn_attacks_a_file() {
        let bb = PieceMove::get_pawn_attacks(Color::White, Square::A2);
        assert_eq!(bb, Square::B3.bitboard());
    }

    #[test]
    fn white_pawn_attacks_h_file() {
        let bb = PieceMove::get_pawn_attacks(Color::White, Square::H4);
        assert_eq!(bb, Square::G5.bitboard());
    }

    #[test]
    fn black_pawn_attacks_a_file() {
        let bb = PieceMove::get_pawn_attacks(Color::Black, Square::A7);
        assert_eq!(bb, Square::B6.bitboard());
    }

    #[test]
    fn black_pawn_attacks_h_file() {
        let bb = PieceMove::get_pawn_attacks(Color::Black, Square::H5);
        assert_eq!(bb, Square::G4.bitboard());
    }
}
