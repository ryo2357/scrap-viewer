#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    let name = name.trim();

    if name.is_empty() {
        return Err("確認する文字を入力してください。".to_string());
    }

    Ok(format!("Rustで受け取りました：{name}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
