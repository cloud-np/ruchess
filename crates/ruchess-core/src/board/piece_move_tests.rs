use super::*;
use crate::pieces::Piece;
use crate::{bitboard, BitBoard, Color, Square};

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

#[test]
fn knight_moves_center() {
    let moves = PieceMove::get_knight_moves(Square::D3);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . X . X . . .
            . X . . . X . .
            . . . . . . . .
            . X . . . X . .
            . . X . X . . .
        }
    );
}

#[test]
fn knight_moves_a_file() {
    let moves = PieceMove::get_knight_moves(Square::A4);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . . .
            . . . . . . . .
            . X . . . . . .
            . . X . . . . .
            . . . . . . . .
            . . X . . . . .
            . X . . . . . .
            . . . . . . . .
        }
    );
}

#[test]
fn knight_moves_h_file() {
    let moves = PieceMove::get_knight_moves(Square::H4);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . . .
            . . . . . . . .
            . . . . . . X .
            . . . . . X . .
            . . . . . . . .
            . . . . . X . .
            . . . . . . X .
            . . . . . . . .
        }
    );
}

#[test]
fn knight_moves_corner() {
    let moves = PieceMove::get_knight_moves(Square::A1);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . X . . . . . .
            . . X . . . . .
            . . . . . . . .
        }
    );
}

#[test]
fn king_moves_center() {
    let moves = PieceMove::get_king_moves(Square::E4);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . X X X . .
            . . . X . X . .
            . . . X X X . .
            . . . . . . . .
            . . . . . . . .
        }
    );
}

#[test]
fn king_moves_corner_a1() {
    let moves = PieceMove::get_king_moves(Square::A1);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            X X . . . . . .
            . X . . . . . .
        }
    );
}

#[test]
fn king_moves_corner_h8() {
    let moves = PieceMove::get_king_moves(Square::H8);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . X .
            . . . . . . X X
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        }
    );
}

#[test]
fn king_moves_a_file() {
    let moves = PieceMove::get_king_moves(Square::A4);
    assert_eq!(
        moves,
        bitboard! {
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            X X . . . . . .
            . X . . . . . .
            X X . . . . . .
            . . . . . . . .
            . . . . . . . .
        }
    );
}
