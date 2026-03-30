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

    pub const fn get_king_moves(from: Square) -> BitBoard {
        const OFFSETS: [(i8, i8); 8] = [
            (-1, -1), (-1, 0), (-1, 1), // Top
            (0, -1), (0, 1),            // Mid
            (1, -1), (1, 0), (1, 1),    // Bottom
        ];
        let mut i = 0;
        let mut moves = BitBoard::EMPTY;
        while i < OFFSETS.len() {
            moves = Self::try_add_move(moves, from, OFFSETS[i].0, OFFSETS[i].1);
            i += 1;
        }
        moves
    }

    pub const fn get_knight_moves(from: Square) -> BitBoard {
        const OFFSETS: [(i8, i8); 8] = [
            (-2, -1),
            (-2, 1), // North
            (2, -1),
            (2, 1), // South
            (1, 2),
            (-1, 2), // East
            (1, -2),
            (-1, -2), // West
        ];

        let mut i = 0;
        let mut moves = BitBoard::EMPTY;
        while i < OFFSETS.len() {
            moves = Self::try_add_move(moves, from, OFFSETS[i].0, OFFSETS[i].1);
            i += 1;
        }
        moves
    }

    pub const fn get_pawn_quiet(color: Color, from: Square, blockers: BitBoard) -> BitBoard {
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

    pub const fn get_pawn_attacks(color: Color, from: Square) -> BitBoard {
        const OFFSETS: [[(i8, i8); 2]; Color::NUM] = [[(-1, 1), (1, 1)], [(-1, -1), (1, -1)]];
        let mut moves = BitBoard::EMPTY;
        let mut i = 0;
        while i < OFFSETS[color as usize].len() {
            let (file, rank) = OFFSETS[color as usize][i];
            moves = Self::try_add_move(moves, from, rank, file);
            i += 1;
        }
        moves
    }

    #[inline]
    const fn try_add_move(
        mut moves: BitBoard,
        from: Square,
        rank_offset: i8,
        file_offset: i8,
    ) -> BitBoard {
        if let Some(sq) = Square::try_offset(from, rank_offset, file_offset) {
            moves.0 |= sq.bitboard().0;
        }
        moves
    }
}

#[cfg(test)]
#[path = "piece_move_tests.rs"]
mod tests;
