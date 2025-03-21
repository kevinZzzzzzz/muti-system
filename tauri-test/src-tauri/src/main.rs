// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// fn main() {
//     tauri_test_lib::run()
// }


use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();
      // 注册全局快捷键 Ctrl+Alt+Shift+I
      window
        .register_accelerator("Ctrl+Alt+Shift+I", move |_| {
          window.open_devtools();
          Ok(())
        })
        .expect("注册快捷键失败");
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("运行应用失败");
}