//! Module containing the `BitBoard` and associated functions / constants.
//!
//! A [`BitBoard`] is a set of bits, where the index of each bit represents a square on the
//! Board. We use this to mark whether or not something is residing at a certain square. For
//! example, if we are using it to map the positions of the white pawns, and there exists a
//! pawn at square B2, the bit at index 9 will be set to '1'. The lack of a piece is marked
//! with a '0' instead.
//!
//! Each bit's index of a [`BitBoard`] maps to the following squares:
//!
//! ```md,ignore
//! 8 | 56 57 58 59 60 61 62 63
//! 7 | 48 49 50 51 52 53 54 55
//! 6 | 40 41 42 43 44 45 46 47
//! 5 | 32 33 34 35 36 37 38 39
//! 4 | 24 25 26 27 28 29 30 31
//! 3 | 16 17 18 19 20 21 22 23
//! 2 | 8  9  10 11 12 13 14 15
//! 1 | 0  1  2  3  4  5  6  7
//!   -------------------------
//!      a  b  c  d  e  f  g  h
//! ```
//!
//! [`BitBoard`]: struct.BitBoard.html
use core::ops::*;
use crate::{Square, Rank, File};

// We use newtype struct instead of pub type Bitboard = u64
// to have a genuinely distinct type that also allows custom mehtods. 
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BitBoard (
    pub u64
);

impl core::ops::Shl<BitBoard> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: Self) -> BitBoard {
        Self(self.0 << rhs.0)
    }
}

macro_rules! impl_math_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for BitBoard {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: Self) -> Self::Output {
                Self($trait::$fn(self.0, rhs.0))
            }
        }
    )*};
}

impl_math_ops! {
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}

impl BitBoard {

    pub const EMPTY: BitBoard = BitBoard(0);

    pub const FULL: BitBoard = BitBoard(!0);

    fn has(self, square: Square) -> bool {
        false
    }
}

impl core::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            write!(f, "bitboard! {{")?;

            for &rank in Rank::ALL.iter().rev() {
                write!(f, "\n   ")?;
                for &file in File::ALL.iter() {
                    if self.has(Square::new(rank, file)) {
                        write!(f, " X")?;
                    } else {
                        write!(f, " .")?;
                    }
                }
            }
            write!(f, "\n}}")?;
            Ok(())
        } else {
            write!(f, "BitBoard({:#018X})", self.0)
        }
    }
}
