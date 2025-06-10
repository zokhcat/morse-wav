fn to_morse(c: char) -> Option<&'static str> {
    match c.to_ascii_uppercase() {
        'A' => Some(".-"),   'B' => Some("-..."), 'C' => Some("-.-."), 'D' => Some("-.."),
        'E' => Some("."),    'F' => Some("..-."), 'G' => Some("--."),  'H' => Some("...."),
        'I' => Some(".."),   'J' => Some(".---"), 'K' => Some("-.-"),  'L' => Some(".-.."),
        'M' => Some("--"),   'N' => Some("-."),   'O' => Some("---"),  'P' => Some(".--."),
        'Q' => Some("--.-"), 'R' => Some(".-."),  'S' => Some("..."),  'T' => Some("-"),
        'U' => Some("..-"),  'V' => Some("...-"), 'W' => Some(".--"),  'X' => Some("-..-"),
        'Y' => Some("-.--"), 'Z' => Some("--.."),
        '0' => Some("-----"),'1' => Some(".----"),'2' => Some("..---"),'3' => Some("...--"),
        '4' => Some("....-"),'5' => Some("....."),'6' => Some("-...."),'7' => Some("--..."),
        '8' => Some("---.."),'9' => Some("----."),
        ' ' => Some("/"),
        _ => None,
    }
}

pub fn encode_string(input: &str) -> String {
    input.chars()
         .filter_map(to_morse)
         .collect::<Vec<_>>()
         .join(" ")
}