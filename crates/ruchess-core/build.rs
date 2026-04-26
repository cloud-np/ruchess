#[derive(Clone, Copy)]
struct BitBoard(u64);

impl BitBoard {
    const EMPTY: BitBoard = BitBoard(0);
}

const SLIDING_MOVE_TABLE_SIZE: usize = 87988;

fn write_moves(
    table: &mut [BitBoard],
    relevant_blockers: impl Fn(Square) -> BitBoard,
    table_index: impl Fn
) {
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut table = [BitBoard::EMPTY; SLIDING_MOVE_TABLE_SIZE];
}
