use std::io::{BufRead, BufReader, Read};
use std::fs::File;

pub fn load_ascii_art(art: &str) -> Vec<String> {
    let mut file = File::open(art).expect("Error opening file!");
    let reader = BufReader::new(file);
    
    // for line in reader.lines() {
    //     println!("{}", line?);
    // }
    reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>()
}