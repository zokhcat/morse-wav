use std::{thread, time::Duration};

use rodio::{OutputStream, Sink};

use crate::{audio::{morse_dash, morse_dot}, morse::encode_string, utils::sleep_ms};

pub fn play_morse_from_text(text: &str) {
    let morse = encode_string(text);
    println!("Morse: {}", morse);
    play_from_morse(&morse);
}

pub fn play_from_morse(code: &str) {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    for c in code.chars() {
        match c {
            '.' => {
                sink.append(morse_dot());
                sink.sleep_until_end();
                sleep_ms(100);
            },
            '-' => {
                sink.append(morse_dash());
                sink.sleep_until_end();
                sleep_ms(100);
            },
            ' ' => thread::sleep(Duration::from_millis(300)), // letter gap
            '/' => thread::sleep(Duration::from_millis(700)), // word gap
            _ => {}
        }
    }
}