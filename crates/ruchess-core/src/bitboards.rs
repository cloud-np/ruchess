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
//!     a  b  c  d  e  f  g  h
//! ```
//!
//! [`BitBoard`]: struct.BitBoard.html
use core::ops::*;
use crate::{Square, Rank, File};

// We use newtype struct instead of pub type Bitboard = u64
// to have a genuinely distinct type that also allows custom mehtods. 
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
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

macro_rules! impl_math_assign_ops {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for BitBoard {
            #[inline(always)]
            fn $fn(&mut self, rhs: Self) {
                self.0.$fn(rhs.0);
            }
        }
    )*};
}

impl_math_assign_ops! {
    BitOrAssign, bitor_assign;
}

impl BitBoard {

    pub const EMPTY: BitBoard = BitBoard(0);

    pub const FULL: BitBoard = BitBoard(!0);

    #[inline(always)]
    pub const fn has(self, square: Square) -> bool {
        (square.bitboard().0 & self.0) != Self::EMPTY.0
    }

    pub fn flip_horizontal(self) -> Self {
        let mut new = self.0;

        const K1: u64 = 0x5555555555555555;
        const K2: u64 = 0x3333333333333333;
        const K4: u64 = 0x0f0f0f0f0f0f0f0f;
        new = ((new >> 1) & K1) | ((new & K1) << 1);
        new = ((new >> 2) & K2) | ((new & K2) << 2);
        new = ((new >> 4) & K4) | ((new & K4) << 4);

        Self(new)
    }

    #[inline(always)]
    pub const fn has_overlap_with(self, other: BitBoard) -> bool {
        self.0 & other.0 == Self::EMPTY.0
    }
}

/// [`BitBoard`] literal macro.
/// ```
/// # use ruchess_core::*;
/// let bb = bitboard! {
///     . . . X . . . .
///     . . . X . . . .
///     . . . X . . . .
///     . . . X . . . .
///     . . . X . . . .
///     X X X . X X X X
///     . . . X . . . .
///     . . . X . . . .
/// };
/// assert_eq!(bb, File::D.bitboard() ^ Rank::Third.bitboard());
/// ```
#[macro_export]
macro_rules! bitboard {
    (
        $a8:tt $b8:tt $c8:tt $d8:tt $e8:tt $f8:tt $g8:tt $h8:tt
        $a7:tt $b7:tt $c7:tt $d7:tt $e7:tt $f7:tt $g7:tt $h7:tt
        $a6:tt $b6:tt $c6:tt $d6:tt $e6:tt $f6:tt $g6:tt $h6:tt
        $a5:tt $b5:tt $c5:tt $d5:tt $e5:tt $f5:tt $g5:tt $h5:tt
        $a4:tt $b4:tt $c4:tt $d4:tt $e4:tt $f4:tt $g4:tt $h4:tt
        $a3:tt $b3:tt $c3:tt $d3:tt $e3:tt $f3:tt $g3:tt $h3:tt
        $a2:tt $b2:tt $c2:tt $d2:tt $e2:tt $f2:tt $g2:tt $h2:tt
        $a1:tt $b1:tt $c1:tt $d1:tt $e1:tt $f1:tt $g1:tt $h1:tt
    ) => {
        $crate::bitboard! { @__inner
            $a1 $b1 $c1 $d1 $e1 $f1 $g1 $h1
            $a2 $b2 $c2 $d2 $e2 $f2 $g2 $h2
            $a3 $b3 $c3 $d3 $e3 $f3 $g3 $h3
            $a4 $b4 $c4 $d4 $e4 $f4 $g4 $h4
            $a5 $b5 $c5 $d5 $e5 $f5 $g5 $h5
            $a6 $b6 $c6 $d6 $e6 $f6 $g6 $h6
            $a7 $b7 $c7 $d7 $e7 $f7 $g7 $h7
            $a8 $b8 $c8 $d8 $e8 $f8 $g8 $h8
        }
    };
    (@__inner $($occupied:tt)*) => {{
        const BITBOARD: $crate::BitBoard = {
            let mut index = 0;
            let mut bitboard = $crate::BitBoard::EMPTY;
            $(
                if $crate::bitboard!(@__square $occupied) {
                    bitboard.0 |= 1 << index;
                }
                index += 1;
            )*
            let _ = index;
            bitboard
        };
        BITBOARD
    }};
    (@__square X) => { true };
    (@__square .) => { false };
    (@__square $token:tt) => {
        compile_error!(
            concat!(
                "Expected only `X` or `.` tokens, found `",
                stringify!($token),
                "`"
            )
        )
    };
    ($($token:tt)*) => {
        compile_error!("Expected 64 squares")
    };
}

impl core::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
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
