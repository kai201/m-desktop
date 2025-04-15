const COMMANDS: &[&str] = &["enable", "disable", "is_enabled"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
