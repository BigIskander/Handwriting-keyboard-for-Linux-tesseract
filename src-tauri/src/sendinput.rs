use std::process::Command;
use tauri::Manager;
use std::{thread, time};

// import global variables
use crate::DEBUG;
use crate::SKIP_TASKBAR;
use crate::USE_SHIFT_CTRL_V;
use crate::RETURN_KEYBOARD;

pub fn xdotool_write_text(app: tauri::AppHandle, text: String, use_clipboard: bool) -> Result<(), String> {
    let return_keyboard = RETURN_KEYBOARD.lock().unwrap();
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
    // return focus back to keyboard's window
    if !return_keyboard.is_empty() && in_focus && skip_taskbar.is_empty() {
        return xdotool_keypress(None, Some(!debug.is_empty()), "alt+Tab".to_string());
    }
    return Ok(());
}

pub fn xdotool_keypress(app: Option<tauri::AppHandle>, to_debug: Option<bool>, key: String) -> Result<(), String> {
    let is_debug: bool;
    if let Some(debug_debug) = to_debug {
        is_debug = debug_debug;
    } else {
        is_debug = !DEBUG.lock().unwrap().is_empty();
    }
    let mut key_to_send: String = "".to_string();
    match key.clone() {
        val if val == "alt+Tab".to_string() => { key_to_send = "alt+Tab".to_string() },
        val if val == "Enter".to_string() => { key_to_send = "Return".to_string() },
        val if val == "Backspace".to_string() => { key_to_send = "BackSpace".to_string() },
        _ => {}
    }
    let mut comm_args = [].to_vec();
    let mut in_focus: bool = false;
    if key != "alt+Tab".to_string() {
        if let Some(app_hand) = app {
            in_focus = app_hand.get_webview_window("local").unwrap().is_focused().unwrap();
        }
    }
    if in_focus == true {
        comm_args.append(&mut ["key", "--delay", "100", "alt+Tab", &key_to_send].to_vec());
    } else {
        comm_args.append(&mut ["key", "--delay", "100", &key_to_send].to_vec());
    }
    if is_debug == true {
        println!("Triggering {} keypress using xdotool.", key);
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
    let return_keyboard = RETURN_KEYBOARD.lock().unwrap();
    let debug = DEBUG.lock().unwrap();
    if !debug.is_empty() {
        println!("Writing text using ydotool.");
    }
    let in_focus = app.get_webview_window("local").unwrap().is_focused().unwrap();
    let use_shift_ctrl_v = USE_SHIFT_CTRL_V.lock().unwrap();
    let mut comm_args = [].to_vec();
    let skip_taskbar = SKIP_TASKBAR.lock().unwrap();
    if in_focus && skip_taskbar.is_empty() {
        let alt_tab = ydotool_keypress(None, Some(!debug.is_empty()), "alt+Tab".to_string());
        if alt_tab.is_err() {
            return alt_tab;
        }
        thread::sleep(time::Duration::from_millis(100));
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
    // return focus back to keyboard's window
    if !return_keyboard.is_empty() && in_focus && skip_taskbar.is_empty() {
        // thread::sleep(time::Duration::from_millis(100));
        return ydotool_keypress(None, Some(!debug.is_empty()), "alt+Tab".to_string());
    }
    return Ok(());
}

pub fn ydotool_keypress(app: Option<tauri::AppHandle>, to_debug: Option<bool>, key: String) -> Result<(), String> {
    let is_debug: bool;
    if let Some(debug_debug) = to_debug {
        is_debug = debug_debug;
    } else {
        is_debug = !DEBUG.lock().unwrap().is_empty();
    }
    let mut in_focus: bool = false;
    if key != "alt+Tab".to_string() {
        if let Some(app_hand) = app {
            in_focus = app_hand.get_webview_window("local").unwrap().is_focused().unwrap();
        }
    }
    if in_focus == true {
        let alt_tab = ydotool_keypress(None, to_debug, "alt+Tab".to_string());
        if alt_tab.is_err() {
            return alt_tab;
        }
        thread::sleep(time::Duration::from_millis(100));
    } 
    let mut comm_args = [].to_vec();
    match key.clone() {
        val if val == "alt+Tab".to_string() => comm_args.append(&mut ["key", "56:1", "15:1", "56:0", "15:0"].to_vec()),
        val if val == "Enter".to_string() => comm_args.append(&mut ["key", "14:1", "14:0"].to_vec()),
        val if val == "Backspace".to_string() => comm_args.append(&mut ["key", "28:1", "28:0"].to_vec()),
        _ => {}
    }
    if is_debug {
        println!("Triggering {} keypress using ydotool.", key);
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