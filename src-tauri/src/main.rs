// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use std::time::Duration;
use std::{io, thread};
use std::str;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn testing_command() {
    // Just a simple print statement to test the command
    println!("Testing command");    
    //open_port();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, testing_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(dead_code)]
fn open_port() {
    // Open the first serialport available.
    let port_name = &serialport::available_ports().expect("No serial port")[0].port_name;
    println!("Using port: {}", port_name);
    let mut port = serialport::new(port_name, 115200)
        .open()
        .expect("Failed to open serial port");

    // Clone the port
    let mut clone = port.try_clone().expect("Failed to clone");

    // Send out 4 bytes every second
    thread::spawn(move || loop {
        clone
            .write_all("s1,1\n".as_bytes())
            .expect("Failed to write to serial port");
        thread::sleep(Duration::from_millis(3000));
    });

    // Read the four bytes back from the cloned port
    let mut buffer: [u8; 1] = [0; 1];
    let mut v: Vec<u8> = Vec::new();
    loop {
        match port.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 1 && buffer[0] < 128 {
                    v.push(buffer[0]);
                    if buffer[0] == 10 {
                        let txt = str::from_utf8(&v).unwrap();
                        let mut s = txt.to_string();
                        trim_newline(&mut s);
                        println!("{}", s);
                        v.clear();
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

#[allow(dead_code)]
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}