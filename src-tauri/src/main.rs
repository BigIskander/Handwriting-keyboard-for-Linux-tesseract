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

// use tmp file? [value read from CLI]
static USE_TMP_FILE: Mutex<String> = {
    let use_tmp_file = String::new();
    Mutex::new(use_tmp_file)
};

// use PaddleOCR [value read from CLI]
static USE_PADDLE_OCR: Mutex<String> = {
    let use_paddle_ocr = String::new();
    Mutex::new(use_paddle_ocr)
};

// use ydotool? [value read from CLI]
static USE_YDOTOOL: Mutex<String> = {
    let use_ydotool = String::new();
    Mutex::new(use_ydotool)
};

// use shift+ctrl+v to paste the text [value read from CLI]
static USE_SHIFT_CTRL_V: Mutex<String> = {
    let use_shift_ctrl_v = String::new();
    Mutex::new(use_shift_ctrl_v)
};

#[tauri::command]
fn recognize_text(app: tauri::AppHandle, base_64_image: String, is_dark_theme: bool) -> Result<String, String> {
    let use_paddle_ocr = USE_PADDLE_OCR.lock().unwrap();
    let ocr_result: Result<String, String>;
    if !use_paddle_ocr.is_empty() {
        ocr_result = ocr::paddle_ocr_recognize_text(app, base_64_image, is_dark_theme);
    } else {
        ocr_result = ocr::tesseract_ocr_recognize_text(base_64_image, is_dark_theme);
    }
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() && ocr_result.is_err() {
        println!("{}", ocr_result.clone().unwrap_err());
    }
    return ocr_result;
}

#[tauri::command]
fn write_text(app: tauri::AppHandle, text: String, use_clipboard: bool) -> Result<(), String> {
    let use_ydotool = USE_YDOTOOL.lock().unwrap();
    let write_text_result: Result<(), String>;
    if !use_ydotool.is_empty() {
        write_text_result = sendinput::ydotool_write_text(app, text, use_clipboard);
    } else {
        write_text_result = sendinput::xdotool_write_text(app, text, use_clipboard);
    }
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() && write_text_result.is_err() {
        println!("{}", write_text_result.clone().unwrap_err());
    }
    return write_text_result;
}

#[tauri::command]
fn alt_tab() {
    let use_ydotool = USE_YDOTOOL.lock().unwrap();
    let alt_tab_result: Result<(), String>;
    if !use_ydotool.is_empty() {
        alt_tab_result = sendinput::ydotool_alt_tab();
    } else {
        alt_tab_result = sendinput::xdotool_alt_tab();
    }
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() && alt_tab_result.is_err() {
        println!("{}", alt_tab_result.clone().unwrap_err());
    }
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
                    let use_tmp_file = &matches.args.get("use-tmp-file").expect("Error reading CLI.").value;
                    if use_tmp_file == true {
                        USE_TMP_FILE.lock().unwrap().insert_str(0, "ok");
                    }
                    let use_paddle_ocr = &matches.args.get("use-paddle-ocr").expect("Error reading CLI.").value;
                    if use_paddle_ocr == true {
                        USE_PADDLE_OCR.lock().unwrap().insert_str(0, "ok");
                    }
                    let use_ydotool = &matches.args.get("use-ydotool").expect("Error reading CLI.").value;
                    if use_ydotool == true {
                        USE_YDOTOOL.lock().unwrap().insert_str(0, "ok");
                    }
                    let use_shift_ctrl_v = &matches.args.get("use-shift").expect("Error reading CLI.").value;
                    if use_shift_ctrl_v == true {
                        USE_SHIFT_CTRL_V.lock().unwrap().insert_str(0, "ok");
                    }
                }
                Err(_) => {}
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![recognize_text, write_text, alt_tab, open_keyboard_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

