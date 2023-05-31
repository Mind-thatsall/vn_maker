// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;
use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::{fs::File, io::Read};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![image_to_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn image_to_base64(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();

    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    let hex = vec.to_hex();

    return format!(
        "data:image/{};base64,{}",
        get_file_type(&hex),
        base64.replace("\r\n", "")
    );
}

pub fn get_file_type(hex: &str) -> &str {
    if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) {
        return "jpeg";
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {
        return "png";
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) {
        return "gif";
    }
    panic!("invalid file type")
}
