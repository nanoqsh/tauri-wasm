use tauri::Window;

#[tauri::command]
fn hello(message: String) -> String {
    println!("message from frontend received: {message}");
    "back!".to_owned()
}

#[tauri::command]
fn close(window: Window) {
    _ = window.close();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                use tauri::Manager;

                app.get_webview_window("app")
                    .expect("the app webview should exist")
                    .open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello, close])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
