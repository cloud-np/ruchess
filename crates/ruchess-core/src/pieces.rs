pub const ALL_PIECES: u8 = 0;
// Number of piece_types
pub const PIECE_TYPE_NB: u8 = 8;
pub const PIECE_MASK: u8 = 7;

#[derive(Debug)]
#[repr(u8)]
pub enum PieceType {
    NoPiece = 0b0,
    Pawn = 0b1,
    Knight = 0b10,
    Bishop = 0b11,
    Rook = 0b100,
    Queen = 0b101,
    King = 0b111, // skips 6
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Piece {
    NoPiece = 0,
    WPawn = 1,
    WKnight = 2,
    WBishop = 3,
    WRook = 4,
    WQueen = 5,
    WKing = 6,
    BPawn = 9,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
    PieceNb = 16,
}

impl Piece {
    pub const fn char(&self) -> char {
        match self {
            Piece::NoPiece | Piece::PieceNb => ' ',
            Piece::WPawn => 'P',
            Piece::WKnight => 'N',
            Piece::WBishop => 'B',
            Piece::WRook => 'R',
            Piece::WQueen => 'Q',
            Piece::WKing => 'K',
            Piece::BPawn => 'p',
            Piece::BKnight => 'n',
            Piece::BBishop => 'b',
            Piece::BRook => 'r',
            Piece::BQueen => 'q',
            Piece::BKing => 'k',
        }
    }

    pub const fn char_to_piece(c: char) -> Piece {
        match c {
            'P' => Piece::WPawn,
            'N' => Piece::WKnight,
            'B' => Piece::WBishop,
            'R' => Piece::WRook,
            'Q' => Piece::WQueen,
            'K' => Piece::WKing,
            'p' => Piece::BPawn,
            'n' => Piece::BKnight,
            'b' => Piece::BBishop,
            'r' => Piece::BRook,
            'q' => Piece::BQueen,
            'k' => Piece::BKing,
            _ => Piece::NoPiece,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

pub enum Color {
    White = 0b0,
    Black = 0b1000,
}
