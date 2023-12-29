// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use base64::decode;
use std::io::{Write};
use std::process::{Command, Stdio};

#[tauri::command]
fn recognize_text(base_64_image: String) -> String {
    let vec8_image = decode(base_64_image).unwrap();
    let mut comm_exec = Command::new("tesseract").args(["-l", "chi_all", "--dpi", "96", "--psm", "13", "--oem", "3", "-", "stdout"]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Error: can't call tesseract");
    let comm_stdin = comm_exec.stdin.as_mut().expect("Error: can't connect to tesseract stdin");
    comm_stdin.write_all(&vec8_image).expect("Error: can't write to tesseract stdin");
    let comm_output = comm_exec.wait_with_output().expect("Error: can't get tesseract output");
    let output = String::from_utf8_lossy(&comm_output.stdout).to_string();
    return output;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![recognize_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
