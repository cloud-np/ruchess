use crate::bitboards::BitBoard;
use crate::pieces::Piece;

pub struct Board {
    pub pieces: [Piece; 64],
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

const FILE_ABB: BitBoard = BitBoard(0x0101010101010101);
const FILE_BBB: BitBoard = BitBoard(FILE_ABB.0 << 1);
const FILE_CBB: BitBoard = BitBoard(FILE_ABB.0 << 2);
const FILE_DBB: BitBoard = BitBoard(FILE_ABB.0 << 3);
const FILE_EBB: BitBoard = BitBoard(FILE_ABB.0 << 4);
const FILE_FBB: BitBoard = BitBoard(FILE_ABB.0 << 5);
const FILE_GBB: BitBoard = BitBoard(FILE_ABB.0 << 6);
const FILE_HBB: BitBoard = BitBoard(FILE_ABB.0 << 7);

const RANK1_BB: BitBoard = BitBoard(0xFF);
const RANK2_BB: BitBoard = BitBoard(RANK1_BB.0 << 8);
const RANK3_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 2));
const RANK4_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 3));
const RANK5_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 4));
const RANK6_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 5));
const RANK7_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 6));
const RANK8_BB: BitBoard = BitBoard(RANK1_BB.0 << (8 * 7));
