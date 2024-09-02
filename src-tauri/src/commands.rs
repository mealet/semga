#[tauri::command]
pub fn check_config(input: String, token: String, mode: String) -> Result<(), String> {
    if let (0, 0, 0) = (input.len(), token.len(), mode.len()) {
        return Err("All fields are empty!".to_string());
    }

    return crate::functions::check_token(token);
}

#[tauri::command]
pub fn generate_token() -> String {
    return crate::functions::generate_token(crate::functions::TOKEN_LEN);
}
