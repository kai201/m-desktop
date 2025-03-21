use enigo::{Enigo, Keyboard, Settings};

#[tauri::command]
pub fn send_text(txt: String) {
    println!("{}", txt);
    if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
        enigo
            .text(&txt)
            .map_err(|e| format!("鼠标操作失败: {}", e))
            .unwrap();
    }
}
