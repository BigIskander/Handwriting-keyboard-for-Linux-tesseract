// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)]

use std::ptr::null;

use base64::decode;
use image::load_from_memory;
use rusty_tesseract::Args;
use rusty_tesseract::Image;
use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// testing
#[tauri::command]
fn recognize_text(base_64_image: String) {
    println!("{base_64_image}");
    let vec8_image = decode(base_64_image).unwrap();
    let dynamic_image = load_from_memory(&vec8_image).unwrap();
    let image = Image::from_dynamic_image(&dynamic_image).unwrap();
    let mut my_args = Args {
        lang: "chi_all".to_string(),
        config_variables: HashMap::new(),
        dpi: Some(
            900,
        ),
        psm: Some(
            13,
        ),
        oem: Some(
            3,
        ),
    };
    let output = rusty_tesseract::image_to_string(&image, &my_args).unwrap();
    println!("The String output is: {:?}", output);
    // dbg!(my_args);
    // dbg!(&image);
    // println!("{the_argument}");
    println!("test ok");
}

fn main() {
    // my_custom_command("BIG SHIT...".to_string());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![recognize_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
