// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::sync::Mutex;
use tauri_plugin_cli::CliExt;
use gtk::{prelude::GtkWindowExt, traits::WidgetExt};
use tauri::Manager;

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
                }
                Err(_) => {}
            }
            // let old_window = app.get_webview_window("main").unwrap();
            // old_window.close()?;

            // let window = tauri::webview::WebviewWindowBuilder::from_config(app, &app.config()
            // .app
            // .windows
            // .get(0)
            // .unwrap()
            // .clone());
            // println!("app config:");
            // println!("{:?}", &app.config().app.windows.get(0).unwrap());
            // .unwrap()
            // .build()
            // .unwrap();
            let window = tauri::webview::WebviewWindowBuilder::new(app, "local", tauri::WebviewUrl::App("keyboard.html".into())).build().unwrap();
            // let window = app.get_webview_window("手写").unwrap();
            window.set_title("手写")?;
            window.set_always_on_top(true)?;
            // window.set_skip_taskbar(true)?;
            let gtk_window = window.gtk_window().unwrap();
            gtk_window.set_accept_focus(false);
            // gtk_window.set_default_size(800, 300);
            // gtk_window.set_size_request(800, 300);
            // // gtk_window.set_focus_visible(false);
            // gtk_window.set_focus_on_map(false);
            window.show()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![recognize_text, write_text, alt_tab])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//"windows": [
    // {
    //     "focus": false,
    //     "title": "手写",
    //     "width": 800,
    //     "height": 300,
    //     "minWidth": 800,
    //     "minHeight": 300,
    //     "alwaysOnTop": true,
    //     "useHttpsScheme": true
    //   }
    // ],
