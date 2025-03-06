use base64::decode;
use std::io::Write;
use std::process::Child;
use std::process::{Command, Stdio};
use std::io::Cursor;
use image::ImageReader;
use image::imageops::colorops;
use regex::Regex;
use tauri::Manager;
use tauri::path::BaseDirectory;

// import global variables
use crate::DEBUG;
// tesseract-ocr specific variables
use crate::TESSDATA_DIR;
use crate::LANG;
use crate::USE_TMP_FILE;

// invert image colors
fn invert_colors(vec8_image: Vec<u8>) -> Vec<u8> {
    let cursor_image = Cursor::new(vec8_image.clone());
    let mut image = ImageReader::new(cursor_image).with_guessed_format().unwrap().decode().unwrap();
    colorops::invert(&mut image);
    let mut cursor_image2 = Cursor::new(Vec::new());
    image.write_to(&mut cursor_image2, image::ImageFormat::Png).unwrap();
    return cursor_image2.get_ref().to_vec();
}

// recognize text using tesseract-ocr
pub fn tesseract_ocr_recognize_text(base_64_image: String, is_dark_theme: bool) -> Result<String, String> {
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() {
        println!("Recognizing text using Tesseract OCR.");
    }
    let mut vec8_image = decode(base_64_image).unwrap();
    if is_dark_theme {
        if !debug.is_empty() {
            println!("Inverting image colors.");
        }
        // invert color
        vec8_image = invert_colors(vec8_image);   
    }
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
    if !debug.is_empty() {
        println!("Executing command: tesseract");
        print!("Command args: ");
        println!("{:?}", comm_args);
    }
    // call tesseract, send image via stdio and get results
    let mut comm_exec = Command::new("tesseract")
        .args(comm_args)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| "Tesseract OCR api call, Error: ".to_string() + &err.to_string())?;
    let mut comm_stdin = comm_exec.stdin.take().unwrap();
    if !debug.is_empty() {
        println!("Sending image to Tesseract OCR via stdin.");
    }
    comm_stdin.write_all(&vec8_image).unwrap();
    drop(comm_stdin);
    let comm_output = comm_exec.wait_with_output().unwrap();
    let comm_output_stderr = String::from_utf8_lossy(&comm_output.stderr).to_string();
    if comm_output_stderr != "" {
        return Err("Tesseract OCR api call, Error: ".to_string() + &comm_output_stderr);
    }
    let output = String::from_utf8_lossy(&comm_output.stdout).to_string();
    return Ok(output);
}

pub fn paddle_ocr_recognize_text(app: tauri::AppHandle, base_64_image: String, is_dark_theme: bool) -> Result<String, String> {
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() {
        println!("Recognizing text using PaddleOCR.");
    }
    let mut vec8_image = decode(base_64_image).unwrap();
    if is_dark_theme {
        if !debug.is_empty() {
            println!("Inverting image colors.");
        }
        // invert color
        vec8_image = invert_colors(vec8_image);
    }
    let use_tmp_file = USE_TMP_FILE.lock().unwrap();
    let mut comm_exec: Child;
    if !use_tmp_file.is_empty() {
        //save temp image file
        let cursor_image = Cursor::new(vec8_image.clone());
        let image = ImageReader::new(cursor_image).with_guessed_format().unwrap().decode().unwrap();
        let image_path = "/tmp/handwriting-keyboard-t_temp_image.png";
        if !debug.is_empty() {
            println!("Saving canvas as temporary image file: {}", image_path);
        }
        image.save(image_path).map_err(|err| "Can't save canvas as temporary file: ".to_string() + &err.to_string())?;
        // call PaddleOCR
        let comm_args = ["--image_dir", image_path, "--use_angle_cls", "true", "--det", "false", "--lang", "ch"];
        if !debug.is_empty() {
            println!("Executing command: paddleocr");
            print!("Command args: ");
            println!("{:?}", comm_args);
        }
        comm_exec = Command::new("paddleocr")
            .args(comm_args)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| "PaddleOCR api call, Error: ".to_string() + &err.to_string())?;
    } else {
        // get python script location
        let resource_path = app.path()
            .resolve("python/run_paddle_ocr.py", BaseDirectory::Resource)
            .map_err(|err| err.to_string())?;
        let run_file = resource_path.to_str().unwrap();
        let comm_args = [run_file, "ch"];
        if !debug.is_empty() {
            println!("Executing command: python3");
            print!("Command args: ");
            println!("{:?}", comm_args);
        }
        // call PaddleOCR, send image via stdio
        comm_exec = Command::new("python3").args(comm_args)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| "PaddleOCR api call, Error: ".to_string() + &err.to_string())?;
        let mut comm_stdin = comm_exec.stdin.take().unwrap();
        if !debug.is_empty() {
            println!("Sending image to python script via stdin.");
        }
        comm_stdin.write_all(&vec8_image).unwrap();
        drop(comm_stdin);
    }
    // process output
    let comm_output = comm_exec.wait_with_output().unwrap();
    let comm_output_stderr = String::from_utf8_lossy(&comm_output.stderr).to_string();
    let output = String::from_utf8_lossy(&comm_output.stdout).to_string();
    // parse PaddleOCR stderr output 
    let err_re: Regex;
    if !use_tmp_file.is_empty() { 
        err_re = Regex::new(r"paddleocr:\s{0,}error:\s{0,}(?<w>.{0,})\s{0,}$").unwrap();
    } else {
        err_re = Regex::new(r"Error:(?<w>.{0,})").unwrap();
    }
    if !debug.is_empty() {
        println!("Parsing stderr output using regex.");
        print!("Regex: ");
        println!("{:?}", err_re);
    }
    let err_found = err_re.captures_iter(&comm_output_stderr).map(|m| {
        m.name("w").unwrap().as_str()
    }).collect::<Vec<&str>>().join(" ");
    if !err_found.is_empty() {
        return Err("PaddleOCR api call, Error: ".to_string() + &err_found.to_string());
    }
    // parse PaddleOCR stdout output
    let re = Regex::new(r"ppocr\s{0,}INFO:\s{0,}\(\'(?<w>.{0,})\'\,.{0,}\)").unwrap();
    if !debug.is_empty() {
        println!("Parsing stdout output using regex.");
        print!("Regex: ");
        println!("{:?}", re);
    }
    let found = re.captures_iter(&output).map(|m| {
        m.name("w").unwrap().as_str()
    }).collect::<Vec<&str>>().join(" ");
    // return the result
    return Ok(found);
}