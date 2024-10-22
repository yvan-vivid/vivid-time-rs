use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // to - to vivid
    // from - from vivid
    Json,
    Now {
        #[arg(short, long)]
        long: bool,

        #[arg(short, long)]
        precision: Option<usize>,

        #[arg(short, long)]
        full: bool,
    },
    Today {
        #[arg(short, long)]
        long: bool,
    },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
