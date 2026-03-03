use clap::Parser;
use ruchess_core::board::DEFAULT_FEN;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = DEFAULT_FEN)]
    pub fen: String,
}

impl Args {

    pub fn init() -> Self {
        let args = Args::parse();

        args
    }
}
