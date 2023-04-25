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
// function that splits the output.txt one-liner into 5 links per file, since the URLs are separated by a single space only
pub fn split() {
    // Read the input from output.txx
    let input = File::open("output.txt").unwrap();

    // Split the input by whitespace and collect into a vector
    let words: Vec<String> = BufReader::new(input)
        .split(b' ')
        .map(|w| String::from_utf8(w.unwrap()).unwrap())
        .collect();

    println!("{:?}", words);

    // Open a file for writing output
    let output = File::create("outputnew.txt").unwrap();

    // Iterate over the words in chunks of 5
    for chunk in words.chunks(5) {
        // Join the chunk with spaces and write to the file
        let line = chunk.join(" ") + "\n";
        BufWriter::new(output.try_clone().unwrap())
            .write_all(line.as_bytes())
            .unwrap();
    }
}
