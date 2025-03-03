use std::process::Command;

// import global variables
use crate::DEBUG;
use crate::SKIP_TASKBAR;

pub fn write_text(text: String, in_focus: bool, use_clipboard: bool) -> Result<(), String> {
    let mut comm_args = [].to_vec();
    let skip_taskbar = SKIP_TASKBAR.lock().unwrap();
    if in_focus && skip_taskbar.is_empty() {
        comm_args.append(&mut ["key", "--delay", "100", "alt+Tab"].to_vec());
    }
    if use_clipboard {
        if !in_focus {
            comm_args.append(&mut ["key", "--delay", "100"].to_vec());
        }
        comm_args.append(&mut ["ctrl+v"].to_vec());
    } else {
        comm_args.append(&mut ["type", "--delay", "300", &text].to_vec());
    }
    let comm_exec = Command::new("xdotool")
        .args(comm_args)
        .output()
        .map_err(|err| "Xdotool call, Error: ".to_string() + &err.to_string())?;
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        let debug = DEBUG.lock().unwrap();
        if !debug.is_empty() {
            println!("Xdotool call, Error: {}", &comm_output_stderr);
        }
        return Err("Xdotool call, Error: ".to_string() + &comm_output_stderr);
    }
    return Ok(());
}

pub fn alt_tab() {
    let comm_exec = Command::new("xdotool")
        .args(["key", "--delay", "100", "alt+Tab"])
        .output()
        .map_err(|err| "Xdotool call, Error: ".to_string() + &err.to_string())
        .unwrap();
    let comm_output_stderr = String::from_utf8_lossy(&comm_exec.stderr).to_string();
    if comm_output_stderr != "" {
        let debug = DEBUG.lock().unwrap();
        if !debug.is_empty() {
            println!("Xdotool call, Error: {}", &comm_output_stderr);
        }
    }
}