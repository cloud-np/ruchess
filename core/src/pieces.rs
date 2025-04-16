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
    King = 0b111,
}

impl PieceType {
    pub const fn char(self, color: Color) -> char {
        let c = match self {
            PieceType::NoPiece => ' ',
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };
        match color {
            Color::Black => c.to_ascii_lowercase(),
            Color::White => c,
        }
    }
}

pub enum Color {
    White = 0b0,
    Black = 0b1000,
}
