// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use base64::decode;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Mutex;

// languages traindata dir for tesseract [value read from CLI]
static TESSDATA_DIR: Mutex<String> = {
    let dir = String::new();
    Mutex::new(dir)
};

// language for recognition [value read from CLI]
static LANG: Mutex<String> = {
    let lang = String::new();
    Mutex::new(lang)
};

// debug, i.e. output errors to console or not [value read from CLI]
static DEBUG: Mutex<String> = {
    let debug = String::new();
    Mutex::new(debug)
};

#[tauri::command]
fn recognize_text(base_64_image: String) -> Result<String, String> {
    let vec8_image = decode(base_64_image).unwrap();
    // working with CLI parameters
    let cli_lang = LANG.lock().unwrap();
    let mut lang = "chi_all".to_string();
    if !cli_lang.is_empty() {
        lang = cli_lang.to_string();
    }
    let mut comm_args = [
        "-l", &lang, "--dpi", "96", "--psm", "7", "--oem", "3", "-", "stdout",
    ]
    .to_vec();
    let cli_tessdata_dir = TESSDATA_DIR.lock().unwrap();
    if !cli_tessdata_dir.is_empty() {
        comm_args.insert(0, "--tessdata-dir");
        comm_args.insert(1, &cli_tessdata_dir);
    }
    // call tesseract, send image via stdio and get results
    let mut comm_exec = Command::new("tesseract")
        .args(comm_args)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| "Tesseract api call, Error: ".to_string() + &err.to_string())?;
    let mut comm_stdin = comm_exec.stdin.take().unwrap();
    comm_stdin.write_all(&vec8_image).unwrap();
    drop(comm_stdin);
    let comm_output = comm_exec.wait_with_output().unwrap();
    let comm_output_stderr = String::from_utf8_lossy(&comm_output.stderr).to_string();
    if comm_output_stderr != "" {
        let debug = DEBUG.lock().unwrap();
        if !debug.is_empty() {
            println!("Xdotool call, Error: {}", &comm_output_stderr);
        }
        return Err("Tesseract api call, Error: ".to_string() + &comm_output_stderr);
    }
    let output = String::from_utf8_lossy(&comm_output.stdout).to_string();
    return Ok(output);
}

#[tauri::command]
fn write_text(text: String, in_focus: bool, use_clipboard: bool) -> Result<(), String> {
    let mut comm_args = [].to_vec();
    if in_focus {
        comm_args.append(&mut ["key", "--delay", "100", "alt+Tab"].to_vec());
    }
    if use_clipboard {
        if !in_focus {
            comm_args.append(&mut ["key", "--delay", "100"].to_vec());
        }
        comm_args.append(&mut ["ctrl+v"].to_vec());
    } else {
        comm_args.append(&mut ["type", "--delay", "300", &text].to_vec());
    }
    let comm_exec = Command::new("xdotool")
        .args(comm_args)
        .output()
        .map_err(|err| "Xdotool call, Error: ".to_string() + &err.to_string())?;
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        let debug = DEBUG.lock().unwrap();
        if !debug.is_empty() {
            println!("Xdotool call, Error: {}", &comm_output_stderr);
        }
        return Err("Xdotool call, Error: ".to_string() + &comm_output_stderr);
    }
    return Ok(());
}

#[tauri::command]
fn alt_tab() {
    let comm_exec = Command::new("xdotool")
        .args(["key", "--delay", "100", "alt+Tab"])
        .output()
        .map_err(|err| "Xdotool call, Error: ".to_string() + &err.to_string())
        .unwrap();
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        let debug = DEBUG.lock().unwrap();
        if !debug.is_empty() {
            println!("Xdotool call, Error: {}", &comm_output_stderr);
        }
    }
}

fn main() {
    use tauri_plugin_cli::CliExt;
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            println!("-------------------------------------------------------------------------------\n\
                        Handwriting keyboard for Linux X11 desktop environment. \n\
                        To recognize handwritten pattern program uses tesseract-ocr. \n\
                        Github page: \n\
                        https://github.com/BigIskander/Handwriting-keyboard-for-Linux-tesseract \n\
                        App version: 1.2.0 \n\
                      -------------------------------------------------------------------------------");
            match app.cli().matches() {
                Ok(matches) => {
                    let cli_tessdata_dir = &matches.args.get("tessdata-dir").expect("Error reading CLI.").value;
                    if cli_tessdata_dir.is_string() {
                        TESSDATA_DIR.lock().unwrap().insert_str(0, cli_tessdata_dir.as_str().expect("Error reading CLI."));
                    }
                    let cli_lang = &matches.args.get("lang").expect("Error reading CLI.").value;
                    if cli_lang.is_string() {
                        LANG.lock().unwrap().insert_str(0, cli_lang.as_str().expect("Error reading CLI."));
                    }
                    let debug = &matches.args.get("lang").expect("Error reading CLI.").value;
                    if debug == true {
                        DEBUG.lock().unwrap().insert_str(0, "ok");
                    }
                }
                Err(_) => {}
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![recognize_text, write_text, alt_tab])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
