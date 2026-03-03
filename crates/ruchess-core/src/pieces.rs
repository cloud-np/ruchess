use ruchess_helpers::simple_enum;
use ruchess_helpers::enum_char_conv;

simple_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Piece {
        Pawn, Knight, Bishop, Rook, Queen, King
    }
}

enum_char_conv! {
    Piece, PieceParseError {
        Pawn = 'p',
        Knight = 'n',
        Bishop = 'b',
        Rook = 'r',
        Queen = 'q',
        King = 'k'
    }
}

simple_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Color {
        White, Black
    }
}
