use crate::{Color, File, Rank, Square, bitboards::BitBoard, pieces::Piece, PieceMove};
use ruchess_helpers::simple_error;


#[derive(Debug)]
pub struct Board {
    pub pieces: [Option<(Piece, Color)>; Square::NUM],
    pub color_bbs: [BitBoard; 2],
    pub pieces_bbs: [BitBoard; 8],
}

#[repr(i8)]
pub enum Direction {
    North = 8,
    South = -8,
    East = 1,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}

// const FILE_ABB: BitBoard = BitBoard(0x0101010101010101);
// const FILE_BBB: BitBoard = BitBoard(FILE_ABB.0 << 1);
// const FILE_CBB: BitBoard = BitBoard(FILE_ABB.0 << 2);
// const FILE_DBB: BitBoard = BitBoard(FILE_ABB.0 << 3);
// const FILE_EBB: BitBoard = BitBoard(FILE_ABB.0 << 4);
// const FILE_FBB: BitBoard = BitBoard(FILE_ABB.0 << 5);
// const FILE_GBB: BitBoard = BitBoard(FILE_ABB.0 << 6);
// const FILE_HBB: BitBoard = BitBoard(FILE_ABB.0 << 7);

// const RANK1_BB: BitBoard = BitBoard(0xFF);
// const RANK2_BB: BitBoard = BitBoard(RANK1_BB.0 << 8);
// const RANK3_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 2));
// const RANK4_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 3));
// const RANK5_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 4));
// const RANK6_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 5));
// const RANK7_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 6));
// const RANK8_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 7));

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/3P4/2N2NP1/PPP1PP1P/R1BQKB1R w KQkq - 0 1";
// "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0";

simple_error! {
    pub enum ParseFenError {
        TooManyParts = "FEN string has too many parts, expected at most 6\n",
        TooFewParts  = "FEN string has too few parts, expected at least 1\n"
    }
}

impl std::default::Default for Board {
    fn default() -> Self {
        Board { pieces: [None; 64], color_bbs: [BitBoard::EMPTY; 2], pieces_bbs: [BitBoard::EMPTY; 8] }
    }

}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for &rank in Rank::ALL.iter().rev() {
            write!(f, " {} ", rank)?;
            for &file in File::ALL.iter() {
                let sq = Square::new(rank, file);
                let ch = match self.pieces[sq as usize] {
                    Some((Piece::King,   Color::White)) => '♔',
                    Some((Piece::Queen,  Color::White)) => '♕',
                    Some((Piece::Rook,   Color::White)) => '♖',
                    Some((Piece::Bishop, Color::White)) => '♗',
                    Some((Piece::Knight, Color::White)) => '♘',
                    Some((Piece::Pawn,   Color::White)) => '♙',
                    Some((Piece::King,   Color::Black)) => '♚',
                    Some((Piece::Queen,  Color::Black)) => '♛',
                    Some((Piece::Rook,   Color::Black)) => '♜',
                    Some((Piece::Bishop, Color::Black)) => '♝',
                    Some((Piece::Knight, Color::Black)) => '♞',
                    Some((Piece::Pawn,   Color::Black)) => '♟',
                    None => '·',
                };
                write!(f, " {ch}")?;
            }
            writeln!(f)?;
        }
        write!(f, "    a b c d e f g h")
    }
}

impl Board {

    pub fn parse_fen(user_fen: Option<&str>) -> Result<Board, ParseFenError> {
        let mut board = Board::default();
        let fen: Vec<&str> = match user_fen {
            Some(fen) => fen.split_whitespace().collect(),
            None => DEFAULT_FEN.split_whitespace().collect(),
        };

        // must have 6 parts :
        // [ Piece Placement, Side to Move, Castling Ability, En Passant square, Half moves, full moves]
        if fen.is_empty() {
            return Err(ParseFenError::TooFewParts);
        }
        if fen.len() > 6 {
            return Err(ParseFenError::TooManyParts);
        }

        // Safety: guarded by the is_empty() check above
        let board_pieces = fen.first().unwrap();
        board.parse_pieces(board_pieces);
        Ok(board)
    }

    #[inline(always)]
    fn add_piece(&mut self, square: Square, piece: Piece, color: Color) {
        self.pieces[square as usize] = Some((piece, color));
        self.color_bbs[color as usize] |= square.bitboard();
        self.pieces_bbs[piece as usize] |= square.bitboard();
    }

    fn parse_pieces(&mut self, board_pieces: &str) {
        let mut rank: u8 = 7;
        let mut file: u8 = 0;
        for c in board_pieces.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
                continue;
            }
            if let Some(skip) = c.to_digit(10) {
                file += skip as u8;
                continue;
            }
            let piece_type = Piece::try_from(c.to_ascii_lowercase());
            if let Ok(piece) = piece_type {
                let color = if c.is_ascii_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                self.add_piece(Square::index_const((rank * 8 + file) as usize), piece, color);
            }
            file += 1;
        }
    }

    pub const fn moves(from: Square) -> BitBoard {
        // PieceMove::get(Piece::Pawn, from)
        todo!()
    }
}

