pub mod audio;
pub mod morse;
pub mod player;
pub mod utils;

use crate::player::{play_from_morse, play_morse_from_text};

fn main() {
    let input = "Akankshya Mishra";
    play_morse_from_text(input);
}
