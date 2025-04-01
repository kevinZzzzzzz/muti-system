// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lib2;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command(invoke_message: String) {
    //   println!("I was invoked from JavaScript!");
    println!(
        "I was invoked from JavaScript, with this message: {}",
        invoke_message
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            lib2::get_printer_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
