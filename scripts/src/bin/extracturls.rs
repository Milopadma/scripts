// given an input.txt, extract all URL occurences in the text file and put it into a new file called output.txt
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    // extracturls();
    split();
}

pub fn extracturls() {
    let input = File::open("input.txt").unwrap();
    let output = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(output);
    let reader = BufReader::new(input);
    // the regex that can capture links such as https://www.youtube.com/watch?v=nYYkRaU0xh8 and https://youtu.be/rgLQWutNxKc
    let re =
        Regex::new(r"https?://(?:www\.)?(?:youtube\.com/watch\?v=|youtu\.be/)([a-zA-Z0-9_-]{11})")
            .unwrap();
    println!("Extracting URLs from input.txt...");
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{:?}", line);
        for cap in re.captures_iter(&line) {
            writer.write_all(cap[0].as_bytes()).unwrap();
            writer.write_all(b" ").unwrap();
        }
    }
}
// function that splits the output.txt into 5 links per file
pub fn split() {
    let input = File::open("output.txt").unwrap();
    let reader = BufReader::new(input);
    let mut count = 0;
    let mut file_count = 0;
    let mut file = File::create(format!("output{}.txt", file_count)).unwrap();
    let mut writer = BufWriter::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        writer.write_all(line.as_bytes()).unwrap();
        writer.write_all(b" ").unwrap();
        count += 1;
        if count == 5 {
            file_count += 1;
            file = File::create(format!("output{}.txt", file_count)).unwrap();
            writer = BufWriter::new(file);
            count = 0;
        }
    }
}
