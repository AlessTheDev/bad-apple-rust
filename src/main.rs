use core::time;
use rand::prelude::SliceRandom;
use screenshots::Screen;
use std::{
    fs::{self, File},
    io::Read,
    thread,
    time::Instant,
};

use image::GenericImageView;

type ImageData = Vec<String>;

fn main() {
    let files = fs::read_dir("frames").unwrap();

    let mut frame_offset = 0; 

    let mut frame: usize = 0; // Frame count
    for file in files {
        if frame < frame_offset { frame += 1; continue; }

        let file_path_buf = file.unwrap().path();
        let file_path = file_path_buf.to_str().unwrap();

        let frame_data = get_image_data(file_path, 2);
        write_image_data(frame_data, "result.rs"); // Write data to file

        // Wait 200 ms so visual studio renders colors
        thread::sleep(time::Duration::from_millis(200));

        take_screenshot(frame);
        println!("{:?} rendered!", file_path);

        frame += 1;
    }
}

/// Takes a screenshot of the main screen with a width of 1500 and 1800 
/// param `frame_n` the frame number
fn take_screenshot(frame_n: usize) {
    let screens = Screen::all().unwrap();

    let screen = screens[0];

    let mut image = screen.capture().unwrap();
    image = screen.capture_area(0, 0, 1500, 1080).unwrap();

    thread::spawn(move || {
        image
        .save(format!("rendered/frame{}.png", convert_to_glob_pattern(frame_n, 4)))
        .unwrap();
    });
    
}

/// Converts a number to glob pattern (ex 23 -> 0023)
/// num_len the new number length (ex. 5 -> 00000)
fn convert_to_glob_pattern(number: usize, num_len: usize) -> String {
    let mut num_string = number.to_string();
    loop {
        if num_string.len() >= num_len{
            return num_string;
        }
        num_string = format!("0{}", num_string);
    };
}

/// Returns a vector of strings containing all the keywords foud in keywords.txt 
fn get_keywords() -> Vec<String> {
    let mut keys_file = File::open("keywords.txt").expect("Can't open the file");

    let mut content = String::new();

    keys_file
        .read_to_string(&mut content)
        .expect("Couldn't read the file");

    content.split(' ').map(|s| s.to_string()).collect()
}

/// Writes the image in the file using the keywords
fn write_image_data(data: ImageData, out_file: &str) {
    // read rust keywords form "keywords.txt"
    let keys: Vec<String> = get_keywords();
    let max_key_len = 11;

    let mut to_write = String::new();

    for line in data {
        let mut count: usize = 0;
        for char in line.chars().into_iter() {
            if char == '0' {
                if count > 1 {
                    to_write += "-";
                    to_write += get_random_keyword(keys.clone(), count - 1).as_str();
                    count = 0;
                }
                to_write += " ";
            } else {
                if count == max_key_len {
                    to_write += "-";
                    to_write += get_random_keyword(keys.clone(), count - 1).as_str();
                    count = 0;
                }
                count += 1;
            }
        }
        to_write += "\n";
    }

    let _ = fs::write(out_file, to_write);
}

/// Retuns a random keyword given a length 
/// it can concatenate multiple keywords to reach the required length
fn get_random_keyword(keywords: Vec<String>, len: usize) -> String {
    if len == 1 {
        return "-".to_string();
    }
    if len == 0 {
        return "".to_string();
    }

    let mut choosen: String;
    loop {
        choosen = keywords
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        if choosen.len() == len {
            return choosen;
        }

        if choosen.len() < len {
            choosen += "-";
            choosen += get_random_keyword(keywords, len - choosen.len()).as_str();
            break;
        }
    }

    choosen
}

/// Returns a vector of Strings containing row data
/// 1 for white pixels and 0 for black ones
/// the scale determines how many pixels get skipped
fn get_image_data(path: &str, scale: u32) -> Vec<String> {
    let image = image::open(path).unwrap();

    let (w, h) = image.dimensions();

    let mut rows: Vec<String> = vec![];
    let mut row_data = String::new();
    for y in 0..h {
        for x in 0..w {
            if y % (scale * 2) == 0 && x % scale == 0 {
                // Skip some pixels
                let pixel = image.get_pixel(x, y);
                if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
                    // It's a white pixel
                    row_data += "1";
                } else {
                    row_data += "0";
                }
            }
        }
        if y % (scale * 2) == 0 {
            // New row
            rows.push(row_data.clone());
            row_data = String::new();
        }
    }

    rows
}
