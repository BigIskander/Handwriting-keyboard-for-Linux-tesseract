// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::sync::Mutex;
use tauri_plugin_cli::CliExt;
use gtk::prelude::GtkWindowExt;
use tauri::{window::Color, Manager};
use tauri::LogicalSize;

mod ocr;
mod sendinput;

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

// skip taskbar? [value read from CLI]
static SKIP_TASKBAR: Mutex<String> = {
    let skip = String::new();
    Mutex::new(skip)
};

// dark theme? [value read from CLI]
static DARK_THEME: Mutex<String> = {
    let dark_theme = String::new();
    Mutex::new(dark_theme)
};

#[tauri::command]
fn recognize_text(base_64_image: String, is_dark_theme: bool) -> Result<String, String> {
    return ocr::paddle_ocr_recognize_text(base_64_image, is_dark_theme);
    // return ocr::tesseract_ocr_recognize_text(base_64_image, is_dark_theme);
}

#[tauri::command]
fn write_text(text: String, in_focus: bool, use_clipboard: bool) -> Result<(), String> {
    return sendinput::write_text(text, in_focus, use_clipboard);
}

#[tauri::command]
fn alt_tab() {
    sendinput::alt_tab();
}

// workaround
// set_accept_focus(false) property doesn't work in main window
#[tauri::command]
fn open_keyboard_window(app: tauri::AppHandle) {
    let url = tauri::WebviewUrl::App("keyboard.html".into());
    let window = tauri::webview::WebviewWindowBuilder::new(&app, "local", url)
    .use_https_scheme(true)
    .build()
    .unwrap();
    window.set_title("手写").unwrap();
    window.set_always_on_top(true).unwrap();
    let min_size: LogicalSize<u32> = tauri::LogicalSize::from((800, 300));
    window.set_min_size(Some(min_size)).unwrap();
    window.set_size(min_size).unwrap();
    let skip_taskbar = SKIP_TASKBAR.lock().unwrap();
    if !skip_taskbar.is_empty() {
        window.set_skip_taskbar(true).unwrap();
    }
    let dark_theme = DARK_THEME.lock().unwrap();
    if !dark_theme.is_empty() {
        window.set_background_color(Some(Color(0, 0, 0, 0))).unwrap();
    }
    let gtk_window = window.gtk_window().unwrap();
    gtk_window.set_accept_focus(false);
    window.show().unwrap();
    // window.set_focus().unwrap();
    app.get_webview_window("main").unwrap().close().unwrap();
}

fn main() {
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
            // panic!("panic panic");
            let main_window = app.get_webview_window("main").unwrap();
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
                    let debug = &matches.args.get("debug").expect("Error reading CLI.").value;
                    if debug == true {
                        DEBUG.lock().unwrap().insert_str(0, "ok");
                    }
                    let dark_theme = &matches.args.get("dark-theme").expect("Error reading CLI.").value;
                    if dark_theme == true {
                        main_window.set_background_color(Some(Color(0, 0, 0, 0))).unwrap();
                        DARK_THEME.lock().unwrap().insert_str(0, "ok");
                    }
                    let skip_taskbar = &matches.args.get("skip-taskbar").expect("Error reading CLI.").value;
                    if skip_taskbar == true {
                        main_window.set_skip_taskbar(true).unwrap();
                        SKIP_TASKBAR.lock().unwrap().insert_str(0, "ok");
                    }
                }
                Err(_) => {}
            }
            // main_window.hide().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![recognize_text, write_text, alt_tab, open_keyboard_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

