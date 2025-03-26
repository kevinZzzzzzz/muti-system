use printers::{get_default_printer, get_printers};
use serde::Serialize;

#[derive(Serialize)]
pub struct PrinterInfo {
    name: String,
    driver: Option<String>,
    state: Option<String>,
    is_default: bool,
}

#[tauri::command]
pub fn get_printer_list() -> Vec<PrinterInfo> {
    let default_printer = get_default_printer();
    get_printers()
        .into_iter()
        .map(|printer| PrinterInfo {
            name: printer.name.clone(),
            driver: Some(printer.driver_name.clone()),
            state: Some(format!("{:?}", printer.state)),
            is_default: default_printer.as_ref().map_or(false, |default| printer.name == default.name),
        })
        .collect()
}