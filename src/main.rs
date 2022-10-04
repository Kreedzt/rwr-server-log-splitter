extern crate notify;

use notify::{Watcher, RecursiveMode, watcher};
use chrono::prelude::*;
use std::io::{Read, Seek};
use std::os::windows::prelude::MetadataExt;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::{self, File};

// static FILE_NAME: &'static str = "target_file.txt";
static FILE_NAME: &'static str = "rwr_server.log";

static LOG_DIR: &'static str = "watch-log";

fn main() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();

    watcher.watch(FILE_NAME, RecursiveMode::NonRecursive).unwrap();

    // Create folder
    fs::create_dir_all(LOG_DIR).unwrap();

    println!("===Started watch: {} ===", FILE_NAME);

    let mut prev_file_size: u64 = 0;
    let mut next_seek_start: u64 = 0;

    loop {
        match rx.recv() {
            Ok(event) => {
                let local = Local::now();
                let current_time = local.format("%Y-%m-%d-%H-%M-%S").to_string();
                println!("===WatchEvent: {}===", current_time);
                let local = Local::now();

                println!("Event: {:?}", event);

                let mut content = File::open(FILE_NAME).unwrap();

                println!("Seek start: {}", next_seek_start);

                content.seek(std::io::SeekFrom::Start(next_seek_start)).unwrap();

                // meta data
                println!("---METADATA---");

                let meta = content.metadata().unwrap();

                println!("{:?}", meta);

                let current_file_size = meta.file_size();

                println!("File size: {}", current_file_size);


                // 若与上次相同, 终止后续操作
                if prev_file_size == current_file_size {
                    println!("Detected same size as prev, skipped.");
                    continue;
                }

                // Get content
                println!("---Started copy file--");

                let mut res_string = String::new();

                content.read_to_string(&mut res_string).unwrap();

                // Get next seek start
                let bytes = content.bytes();

                let count = bytes.count();

                next_seek_start = meta.len();

                // Write to new file
                let target_file_log_name = format!("{}/{}-{}.log", LOG_DIR, FILE_NAME, current_time);

                println!("ready to copy to: {}", target_file_log_name);

                fs::write(target_file_log_name, res_string).unwrap();

                println!("---Copy file completed---");

                prev_file_size = current_file_size;
            },
            Err(e) => println!("watch error: {:?}", e)
        }
    }
}
