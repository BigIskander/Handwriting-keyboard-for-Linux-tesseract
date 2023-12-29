// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use base64::decode;
use image::load_from_memory;
use rusty_tesseract::{Args, Image, image_to_string};
use std::collections::HashMap;
use tempfile::tempfile;

#[tauri::command]
fn recognize_text(base_64_image: String) -> String {
    // println!("{base_64_image}");
    let vec8_image = decode(base_64_image).unwrap();
    let dynamic_image = load_from_memory(&vec8_image).unwrap();
    //
    let tempfile = tempfile::Builder::new()
            .prefix("rusty-tesseract")
            .suffix(".png")
            .tempfile()
            .unwrap();
    let path = tempfile.path();
    dynamic_image.save(path); // saving image to temp path is slow...

    // let path_str = path;
    // println!("{path_str}");

    //
    // let image = Image::from_dynamic_image(&dynamic_image).unwrap(); // slow function
    // let image = Image::from_path(path).unwrap();
    return  "respond...".to_string();
    // let my_args = Args {
    //     lang: "chi_all".to_string(),
    //     config_variables: HashMap::new(),
    //     dpi: Some(
    //         96,
    //     ),
    //     psm: Some(
    //         13,
    //     ),
    //     oem: Some(
    //         3,
    //     ),
    // };
    // let output = image_to_string(&image, &my_args).unwrap();
    // return output;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![recognize_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
