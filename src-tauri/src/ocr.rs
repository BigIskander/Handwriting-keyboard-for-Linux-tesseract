use base64::decode;
use std::io::Write;
use std::process::{Command, Stdio};
use std::io::Cursor;
use image::ImageReader;
use image::imageops::colorops;
use regex::Regex;

// import global variables
use crate::DEBUG;
// tesseract-ocr specific variables
use crate::TESSDATA_DIR;
use crate::LANG;

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
    let mut vec8_image = decode(base_64_image).unwrap();
    if is_dark_theme {
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
            println!("Tesseract api call, Error: {}", &comm_output_stderr);
        }
        return Err("Tesseract api call, Error: ".to_string() + &comm_output_stderr);
    }
    let output = String::from_utf8_lossy(&comm_output.stdout).to_string();
    return Ok(output);
}

pub fn paddle_ocr_recognize_text(base_64_image: String, is_dark_theme: bool) -> Result<String, String> {
    let mut vec8_image = decode(base_64_image).unwrap();
    if is_dark_theme {
        // invert color
        vec8_image = invert_colors(vec8_image);
    }
    let cursor_image = Cursor::new(vec8_image.clone());
    let image = ImageReader::new(cursor_image).with_guessed_format().unwrap().decode().unwrap();
    _ = image.save("/tmp/temp_image.png").map_err(|err| "Can't save temp image file: ".to_string() + &err.to_string());
    //
    let comm_args = ["--image_dir", "/tmp/temp_image.png", "--det", "false"]; 
    // call PaddleOCR, send image via stdio and get results
    let mut comm_exec = Command::new("paddleocr")
        .args(comm_args)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| "PaddleOCR api call, Error: ".to_string() + &err.to_string())?;
    let mut comm_stdin = comm_exec.stdin.take().unwrap();
    comm_stdin.write_all(&vec8_image).unwrap();
    drop(comm_stdin);
    let comm_output = comm_exec.wait_with_output().unwrap();
    let comm_output_stderr = String::from_utf8_lossy(&comm_output.stderr).to_string();
    // if comm_output_stderr != "" {
    //     let debug = DEBUG.lock().unwrap();
    //     if !debug.is_empty() {
    //         println!("PaddleOCR api call, Error: {}", &comm_output_stderr);
    //     }
    //     return Err("PaddleOCR api call, Error: ".to_string() + &comm_output_stderr);
    // }
    let output = String::from_utf8_lossy(&comm_output.stdout).to_string();
    println!("{}", output);
    // let test_text = " [2025/03/02 15:58:28] ppocr INFO: **********/tmp/temp_image.png********** \n
    //                         [2025/03/02 15:58:28] ppocr INFO: ('你你们', 0.9777671694755554)           \n
    //                         [2025/03/02 15:58:28] ppocr INFO: **********/tmp/temp_image.png**********
    //                         [2025/03/02 15:58:28] ppocr INFO: ('我们', 0.9777671694755554)";
    // println!("{}", test_text);
    // parse PaddleOCR output
    let re = Regex::new(r"ppocr\sINFO:\s\(\'(?<w>.{0,})\'\,.{0,}\)").unwrap();
    let found = re.captures_iter(&output).map(|m| {
        m.name("w").unwrap().as_str()
    }).collect::<Vec<&str>>().join(" ");
        // r"sendtextadb:(.{0,}$)").unwrap();
    // let Some(caps) = re.captures(&test_text) else { 
    //     return Ok("not Ok".to_string()); 
    // };
    println!("found something maybe: ");
    println!("{}", found);
    // println!("{:?}", caps);
    // return Ok(output);
    return Ok(found);
}