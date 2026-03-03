use crate::{BitBoard, File, Rank};

ruchess_helpers::simple_enum! {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Square {
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8
    }
}


impl Square {
    #[inline(always)]
    pub const fn new(rank: Rank, file: File) -> Self {
        Self::index_const((rank as usize) << 3 | file as usize)
    }

    #[inline(always)]
    pub const fn file(self) -> File {
        File::index_const(self as usize & 0b000111)
    }

    #[inline(always)]
    pub const fn rank(self) -> Rank {
        Rank::index_const(self as usize >> 3)
    }

    #[inline(always)]
    pub const fn bitboard(self) -> BitBoard {
        BitBoard(1 << self as u8)
    }

    // Trying to go to given offset
    #[inline(always)]
    pub const fn try_offset(self, file_offset: i8, rank_offset: i8) -> Option<Square> {
        let file_index = self.file() as i8 + file_offset;
        let rank_index = self.rank() as i8 + rank_offset;
        if file_index < 0 || file_index >= 8 || rank_index < 0 || rank_index >= 8 {
            return None;
        }
        let file = File::index_const(file_index as usize);
        let rank = Rank::index_const(rank_index as usize);
        Some(Square::new(rank, file))
    }
}

impl core::fmt::Display for Square {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

