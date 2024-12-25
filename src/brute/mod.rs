use colored::Colorize;
use std::io::{read_to_string, Read};

pub fn enumerate<T: Read>(f: T) {
    println!("\n{}", "Brute forcing with dns.txt".red().underline());

    let domains: Vec<String> = read_to_string(f)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect();
    if let Some(l) = domains.into_iter().next() {
        println!("{}", l);
    }
}
