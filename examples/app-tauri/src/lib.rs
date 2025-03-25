use tauri::{Window, ipc::Request};

#[tauri::command]
fn hello(message: String) -> String {
    println!("message from frontend received: {message}");
    message
}

#[tauri::command]
fn headers(req: Request<'_>) -> String {
    let mut values = vec![];
    for (key, value) in req.headers() {
        if !key.as_str().starts_with("app-") {
            continue;
        }

        let Ok(value) = value.to_str() else {
            return "invalid header".to_owned();
        };

        values.push(value);
    }

    values.sort();
    values.join(".")
}

#[tauri::command]
fn close(window: Window) {
    _ = window.close();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Listener;

            #[cfg(debug_assertions)]
            {
                use tauri::Manager;

                app.get_webview_window("app")
                    .expect("the app webview should exist")
                    .open_devtools();
            }

            app.listen("test-event", |event| {
                let payload = event.payload();
                println!("test-event: {payload}");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello, headers, close])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
