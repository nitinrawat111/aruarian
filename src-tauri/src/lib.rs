use tauri::Manager;
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
        .with_handler(|app, shortcut, _event| {
            if shortcut.matches(Modifiers::SUPER, Code::Space) {
                // Open the launcher window when the shortcut is pressed
                // "launcher" is the label of the window defined in tauri.conf.json
                app.get_webview_window("launcher")
                    .expect("Failed to get launcher window")
                    .show()
                    .expect("Failed to show launcher window");
            }
        })
        .build();

    tauri::Builder::default()
        .plugin(global_shortcut_plugin)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Earlier i thought of opening a new window when the shortcut is pressed and then closing it
            // But that seemed expensive, so instead decided to hide/unhide the same window

            // "launcher" is the label of the window defined in tauri.conf.json
            let launcher_window = app
                .get_webview_window("launcher")
                .expect("Failed to get launcher window");
            // Hide the launcher window on startup
            launcher_window
                .hide()
                .expect("Failed to hide launcher window on startup");

            // Also attach an event listener to hide the launcher window when it loses focus
            launcher_window.clone().on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(false) = event {
                    launcher_window
                        .hide()
                        .expect("Failed to hide launcher window");
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
