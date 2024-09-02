use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;

pub const TOKEN_LEN: usize = 64;

pub fn remove_duplicates(input: &String) -> String {
    let mut seen = HashSet::new();
    input.chars().filter(|&c| seen.insert(c)).collect()
}

pub fn check_token(token: String) -> Result<(), String> {
    let formatted_token = token.replace(format!("{}", crate::algorithms::DEVCODE).as_str(), "");
    if formatted_token.len() < TOKEN_LEN {
        return Err(format!(
            "Token's length must be at least {} symbols",
            TOKEN_LEN
        ));
    }
    if remove_duplicates(&formatted_token).len() < formatted_token.len() {
        return Err("Token contains duplicates! Remove it and repeat".to_string());
    }

    return Ok(());
}

pub fn generate_token(length: usize) -> String {
    let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()-_=+[]{};:',.<>?|`~".chars().collect();

    let mut rng = thread_rng();
    let mut shuffled_chars = chars.clone();
    shuffled_chars.shuffle(&mut rng);

    let random_string: String = shuffled_chars.iter().take(length).collect();

    return random_string;
}
