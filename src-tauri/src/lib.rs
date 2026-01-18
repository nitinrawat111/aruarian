use tauri_plugin_global_shortcut::{Code, Modifiers};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Register the global shortcut for the launcher
    // Reference: https://crates.io/crates/tauri-plugin-global-shortcut
    let global_shortcut_plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts(["Super+Space"])
        // It's ok to use unwrap here
        // with_shortcuts only panics when the provided shortcut string is invalid
        // This will never panic (source: trust me bro)
        .unwrap()
        .with_handler(|app, shortcut, event| {
            if shortcut.matches(Modifiers::SUPER, Code::Space) {
                // TODO: Create window here
                println!("shortcut pressed");
            }
        })
        .build();

    tauri::Builder::default()
        .plugin(global_shortcut_plugin)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
