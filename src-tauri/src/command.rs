use enigo::{Enigo, Keyboard, Settings};

use crate::winx::{activate, get_active_window, get_windows};

#[tauri::command]
pub fn send_text(txt: String) {
    // get_active_window();
    let ws = get_windows();

    for n in ws {
        println!("{} , {}", n.process_id, n.app_name)
    }

    activate(String::from("35815"));
    println!("{}", txt);

    // if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
    //     enigo
    //         .text(&txt)
    //         .map_err(|e| format!("鼠标操作失败: {}", e))
    //         .unwrap();
    // }
}
