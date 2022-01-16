extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::fs;
use std::fs::{File};
use std::io::{Write};
use termprogress::prelude::*;
use clap::{Arg, App};

const PAD_DIR: &str = "pads";

fn generate_pads(pad_count: &str, pad_size: &str) {
    let pad_count: i32 = pad_count.parse().expect("Invalid number of pads");
    let pad_size: i32 = pad_size.parse().expect("Invalid pad size");

    create_pad_directory();

    println!("Generating {} one-time pads with {} bytes each", pad_count, pad_size);
    for pad_number in 0..pad_count {
        generate_pad(pad_number, pad_size);
    }
}

fn generate_pad(pad_id: i32, size: i32) {
    let mut progress = Bar::default();
    progress.set_title(&format!("Generating {}/{}.pad ...", PAD_DIR, pad_id).to_string());

    let mut rng = thread_rng();
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..size {
        bytes.push(rng.gen());
        if i % 1000 == 0 {
            // Periodically update the status bar
            progress.set_progress(i as f64/size as f64);
        }
    }

    write_bytes_to_file(bytes, pad_id);
    progress.set_progress(1.0);
    progress.set_title(&format!("Generated {}/{}.pad", PAD_DIR, pad_id).to_string());
    progress.complete();
}

fn create_pad_directory() {
    fs::create_dir_all(PAD_DIR).expect(&format!("Unable to make directory {}", PAD_DIR).to_string());
}

fn write_bytes_to_file(bytes: Vec<u8>, pad_id: i32) {
    let output_path = format!("{}/{}.pad", PAD_DIR, pad_id).to_string();
    let mut file = File::create(&output_path).expect("Failed to create file");
    file.write_all(&bytes).expect("Failed to write to file");
}

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .arg(Arg::with_name("number")
                    .help("The number of pads to generate")
                    .long("number")
                    .short("n")
                    .takes_value(true)
                    .default_value("1"))
                .arg(Arg::with_name("size")
                    .help("The size of the pads (in bytes)")
                    .long("size")
                    .short("s")
                    .takes_value(true)
                    .required(true))
                .get_matches();

    generate_pads(
        args.value_of("number").unwrap(),
        args.value_of("size").unwrap());
}
