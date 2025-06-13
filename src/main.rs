pub mod audio;
pub mod morse;
pub mod player;
pub mod utils;

use crate::{morse::decode_morse, player::{play_from_morse, play_morse_from_text}};
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "morse_wav")]
#[command(about ="morse code encoder and decoder", long_about = None)]
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
    },
    Decode {
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
        Commands::Decode { code } => {
            let decoded_str = decode_morse(&code);
            println!("Decoded string: {}", decoded_str);
        }
    }
}
