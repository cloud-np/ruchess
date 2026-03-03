use ruchess_cli::Args;
use ruchess_core::Board;

fn main() {
    let args = Args::init();

    let board = match Board::parse_fen(Some(&args.fen)) {
        Ok(board) => board,
        Err(e) => {
            print!("{}", e);
            return;
        }
    };

    println!("{}", board);
}
