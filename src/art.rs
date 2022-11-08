use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Art {
    pub ascii: Vec<String>,
    pub caption: String,
}


pub fn load_ascii_art(art: &str) -> Vec<String> {
    let file = File::open(art).expect("Error opening file!");
    let reader = BufReader::new(file);
    
    // for line in reader.lines() {
    //     println!("{}", line?);
    // }
    reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>()
}

impl Art {
    pub fn new(path: &str, caption: String) -> Self {
        let ascii = load_ascii_art(path);
        Art {
            ascii,
            caption,
        }
    }
}