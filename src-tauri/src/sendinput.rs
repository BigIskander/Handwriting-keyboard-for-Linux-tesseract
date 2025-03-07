use std::process::Command;
use tauri::Manager;

// import global variables
use crate::DEBUG;
use crate::SKIP_TASKBAR;
use crate::USE_SHIFT_CTRL_V;

pub fn xdotool_write_text(app: tauri::AppHandle, text: String, use_clipboard: bool) -> Result<(), String> {
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() {
        println!("Writing text using xdotool.");
    }
    let in_focus = app.get_webview_window("local").unwrap().is_focused().unwrap();
    let use_shift_ctrl_v = USE_SHIFT_CTRL_V.lock().unwrap();
    let mut comm_args = [].to_vec();
    let skip_taskbar = SKIP_TASKBAR.lock().unwrap();
    if in_focus && skip_taskbar.is_empty() {
        comm_args.append(&mut ["key", "--delay", "100", "alt+Tab"].to_vec());
    }
    if use_clipboard {
        if !in_focus {
            comm_args.append(&mut ["key", "--delay", "100"].to_vec());
        } else {
            if !skip_taskbar.is_empty() {
                return Err("Can't paste the text.".to_string());
            }
        }
        if !use_shift_ctrl_v.is_empty() {
            comm_args.append(&mut ["shift+ctrl+v"].to_vec());
        } else {
            comm_args.append(&mut ["ctrl+v"].to_vec());
        }
    } else {
        comm_args.append(&mut ["type", "--delay", "300", &text].to_vec());
    }
    if !debug.is_empty() {
        println!("Executing command: xdotool");
        print!("Command args: ");
        println!("{:?}", comm_args);
    }
    let comm_exec = Command::new("xdotool")
        .args(comm_args)
        .output()
        .map_err(|err| "Xdotool call, Error: ".to_string() + &err.to_string())?;
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        return Err("Xdotool call, Error: ".to_string() + &comm_output_stderr);
    }
    return Ok(());
}

pub fn xdotool_alt_tab() -> Result<(), String> {
    let debug = DEBUG.lock().unwrap();
    let comm_args = ["key", "--delay", "100", "alt+Tab"];
    if !debug.is_empty() {
        println!("Triggering alt+Tab keypress using xdotool.");
        println!("Executing command: xdotool");
        print!("Command args: ");
        println!("{:?}", comm_args);
    }
    let comm_exec = Command::new("xdotool")
        .args(comm_args)
        .output()
        .map_err(|err| "Xdotool call, Error: ".to_string() + &err.to_string())?;
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        return Err("Xdotool call, Error: ".to_string() + &comm_output_stderr);
    }
    return Ok(());
}

pub fn ydotool_write_text(app: tauri::AppHandle, text: String, use_clipboard: bool) -> Result<(), String> {
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() {
        println!("Writing text using ydotool.");
    }
    let in_focus = app.get_webview_window("local").unwrap().is_focused().unwrap();
    let use_shift_ctrl_v = USE_SHIFT_CTRL_V.lock().unwrap();
    let mut comm_args = [].to_vec();
    let skip_taskbar = SKIP_TASKBAR.lock().unwrap();
    if in_focus && skip_taskbar.is_empty() {
        comm_args.append(&mut ["key", "56:1", "15:1", "56:0", "15:0"].to_vec());
    }
    if use_clipboard {
        if in_focus && !skip_taskbar.is_empty() {
            return Err("Can't paste the text.".to_string());
        }
        if !use_shift_ctrl_v.is_empty() {
            comm_args.append(&mut ["key", "42:1", "29:1", "47:1", "42:0", "29:0", "47:0"].to_vec());
        } else {
            comm_args.append(&mut ["key", "29:1", "47:1", "29:0", "47:0"].to_vec());
        }
    } else {
        comm_args.append(&mut ["type", &text].to_vec());
    }
    // comm_args.append(&mut ["type", &text].to_vec());
    if !debug.is_empty() {
        println!("Executing command: ydotool");
        print!("Command args: ");
        println!("{:?}", comm_args);
    }
    let comm_exec = Command::new("ydotool")
        .args(comm_args)
        .output()
        .map_err(|err| "Ydotool call, Error: ".to_string() + &err.to_string())?;
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        return Err("Ydotool call, Error: ".to_string() + &comm_output_stderr);
    } else {
        // ydotool outputs error in stdout instead of stderr in newer version
        let output =String::from_utf8_lossy(&comm_exec.stdout).to_string();
        if  output != "" {
            return Err("Ydotool call, Error: ".to_string() + &output);
        }
    }
    return Ok(());
}

pub fn ydotool_alt_tab() -> Result<(), String> {
    let debug = DEBUG.lock().unwrap();
    let comm_args = ["key", "56:1", "15:1", "56:0", "15:0"];
    if !debug.is_empty() {
        println!("Triggering alt+Tab keypress using ydotool.");
        println!("Executing command: ydotool");
        print!("Command args: ");
        println!("{:?}", comm_args);
    }
    let comm_exec = Command::new("ydotool")
        .args(comm_args)
        .output()
        .map_err(|err| "Ydotool call, Error: ".to_string() + &err.to_string())?;
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        return Err("Ydotool call, Error: ".to_string() + &comm_output_stderr);
    }
    return Ok(());
}