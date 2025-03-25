use printers::{get_printers};

// use serde::Serialize;
use printers;

// #[derive(Serialize)]
// struct PrinterInfo {
//     name: String,
//     is_default: bool,
//     state: String,
//     driver: Option<String>
// }


#[tauri::command]
pub fn get_printer_list() {
  let printers = get_printers();
  // let default_name = get_default_printer();
  // for printer in printers {
  
  //     print!("{:?}", printer);
  // }
  println!("{:?}", printers);
  // printers.map(|printer| {
  //   Ok(PrinterInfo {
  //       name: printer.name().to_string(),
  //       is_default: printer.name() == default_name,
  //       state: match printer.state() {
  //           Some(state) => format!("{:?}", state), // 将枚举转为字符串
  //           None => "未知状态".into()
  //       },
  //       driver: printer.driver().map(|s| s.to_string())
  //   })
  // })
  // .collect()
}