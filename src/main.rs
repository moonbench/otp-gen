extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::fs;
use std::fs::{File};
use std::io::{Write};

const SHEET_DIR: &str = "sheets";

fn title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(&format!(" (v{}) ", env!("CARGO_PKG_VERSION")).to_string());
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn usage() {
    println!("{}", title());
    println!("Usage: {} <number of sheets> <size in bytes>", env!("CARGO_PKG_NAME"));
}

fn generate_sheets(sheet_count: &str, sheet_size: &str) {
    let sheet_count: i32 = sheet_count.parse().expect("[ ERROR ] Invalid number of sheets");
    let sheet_size: i32 = sheet_size.parse().expect("[ ERROR ] Invalid sheet size");
    fs::create_dir_all(SHEET_DIR).expect(&format!("[ ERROR ] Unable to make directory {}", SHEET_DIR).to_string());

    println!("[ INFO ] Generating {} sheets with {} bytes each", sheet_count, sheet_size);

    for sheet_number in 0..sheet_count {
        generate_sheet(sheet_number, sheet_size);
    }
}

fn generate_sheet(sheet_id: i32, size: i32) {
    println!("[ INFO ] Generating sheet {}", sheet_id);

    let mut rng = thread_rng();
    let mut bytes: Vec<u8> = Vec::new();
    for _ in 0..size {
        bytes.push(rng.gen());
    }

    let output_path = format!("{}/sheet{}.pad", SHEET_DIR, sheet_id).to_string();
    let mut file = File::create(&output_path).expect("[ ERROR ] Failed to create file");
    file.write_all(&bytes).expect("[ ERROR ] Failed to write to file");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        3 => generate_sheets(&args[1], &args[2]),
        _ => usage()
    }
}
