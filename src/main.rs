extern crate reqwest;

use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let video_id = &args[1].trim();
    let res = request(&video_id).expect("couldn't request video ID!");

    println!("Got response: {}", res);
}

fn request(video_id: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://youtube.com/get_video_info?video_id={}", video_id);
    println!("Requesting: {}", url);
    Ok(reqwest::blocking::get(&url)?.text()?)
}
