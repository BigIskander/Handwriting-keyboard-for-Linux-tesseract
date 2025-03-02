use base64::decode;
use std::io::Write;
use std::process::{Command, Stdio};
use std::io::Cursor;
use image::ImageReader;
use image::imageops::colorops;

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