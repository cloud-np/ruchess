use ruchess_cli::Args;
use ruchess_core::{Rank, Square};
use ruchess_core::fen::{ParseFenError, parse_fen};

fn main() {
    let args = Args::init();
    if let Err(e) = parse_fen(&args.fen) {
        print!("{}", e);
    }
    
    // println!("{:#}", Square::E4.bitboard() | Square::A1.bitboard());
}
