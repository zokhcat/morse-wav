pub mod audio;
pub mod morse;
pub mod player;
pub mod utils;

use crate::player::{play_from_morse, play_morse_from_text};
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cdbe")]
#[command(about = "A simple columnar database implementation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Text {
        input: String,
    },
    Play {
        code: String
    }
}    

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Text { input } => {
            play_morse_from_text(&input);
        }
        Commands::Play { code } => {
            play_from_morse(&code);
        }
    }
}
