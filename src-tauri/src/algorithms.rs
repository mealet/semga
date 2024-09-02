// Semga | by mealet
// https://github.com/mealet/semga
// =======================================
// Project licensed under the MIT License.
// See more in License file.
//
// Original algorithm by `sem-rs` | https://github.com/mealet/sem-rs

use chrono::prelude::*;

const ALPHABET: [char; 64] = [
    ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '!', '?', '.', ',', '/', '(', ')', '[', ']', '@', '#',
    '$', '%', '^', '&', '*', '-', '_', '+', '=', '`', '~', '"', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '0', ':', ';', '\\', '\n',
];

pub const DEVCODE: &str = "0x7D9:";

pub fn encrypt(input: String, token_string: String) -> String {
    let token = token_string.chars().collect::<Vec<char>>();
    let mut output = String::new();

    let formatted_input = format!(
        "{}--[timestamp:{}]",
        input.clone().trim(),
        Utc::now().timestamp()
    );

    for c in formatted_input.chars() {
        let alphabet_position = ALPHABET.iter().position(|r| *r == c);
        match alphabet_position {
            Some(t) => {
                output += &token[t].to_string();
            }
            None => {}
        };
    }

    let reversed_output = output.chars().rev().collect::<String>();

    return reversed_output;
}

pub fn decrypt(input: String, token_string: String) -> String {
    let mut token = token_string.clone();
    let mut output = String::new();

    let mut dev_mode = false;

    if token.starts_with(DEVCODE) {
        dev_mode = true;
        token = token.replace(format!("{}", DEVCODE).as_str(), "");
    }

    let token_chars = token.chars().collect::<Vec<char>>();
    let reversed_input = input.trim().chars().rev().collect::<String>();

    for c in reversed_input.chars() {
        let token_position = &token_chars.iter().position(|r| *r == c);
        match token_position {
            Some(t) => {
                output += ALPHABET[*t].to_string().as_str();
            }
            None => {}
        };
    }

    if !dev_mode {
        output = output.split("--[timestamp:").next().unwrap().to_string();
    }

    return output;
}
