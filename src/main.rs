extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::fs;
use std::fs::{File};
use std::io::{Write};
use termprogress::prelude::*;

const PAD_DIR: &str = "pads";

fn title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(&format!(" (v{}) ", env!("CARGO_PKG_VERSION")).to_string());
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn usage() {
    println!("{}", title());
    println!("Usage: {} <number of pads> <size in bytes>", env!("CARGO_PKG_NAME"));
}

fn generate_pads(pad_count: &str, pad_size: &str) {
    let pad_count: i32 = pad_count.parse().expect("[ ERROR ] Invalid number of pads");
    let pad_size: i32 = pad_size.parse().expect("[ ERROR ] Invalid pad size");
    fs::create_dir_all(PAD_DIR).expect(&format!("[ ERROR ] Unable to make directory {}", PAD_DIR).to_string());

    println!("Generating {} one-time pads with {} bytes each", pad_count, pad_size);

    for pad_number in 0..pad_count {
        generate_pad(pad_number, pad_size);
    }
}

fn generate_pad(pad_id: i32, size: i32) {
    let mut progress = Bar::default();
    progress.set_title(&format!("Generating pad {}...", pad_id).to_string());

    let mut rng = thread_rng();
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..size {
        bytes.push(rng.gen());
        if i % 1000 == 0 {
            progress.set_progress(i as f64/size as f64);
        }
    }

    progress.set_progress(1.0);
    progress.set_title(&format!("Generated pad {}.", pad_id).to_string());

    let output_path = format!("{}/pad{}.pad", PAD_DIR, pad_id).to_string();
    let mut file = File::create(&output_path).expect("[ ERROR ] Failed to create file");
    file.write_all(&bytes).expect("[ ERROR ] Failed to write to file");
    progress.complete();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        3 => generate_pads(&args[1], &args[2]),
        _ => usage()
    }
}
