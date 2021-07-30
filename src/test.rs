// global ignore dead code warnings, while developing test functions.
#![allow(dead_code)]



pub fn command_line_arguments_read() {
    // basic output
    println!("Hello, world!");

    // command line arguments
    for argument in std::env::args() {
        println!("->{}", argument);
    }
}


pub fn command_line_arguments_individual() {
    // get individual command line arguments distinguished by position
    let args:Vec<String> = std::env::args().collect();
    let total_arguments = args.len();

    if total_arguments == 3 {
        println!("correct number of arguments [{}]", total_arguments);

        let zero = &args[0];
        println!("zero [{}]", zero);

        let one = &args[1];
        println!("one [{}]", one);

        let two = &args[2];
        println!("two [{}]", two);

    } else {
        println!("usage: [metadata.json] [data.csv]");
    }


}

pub fn read_file_data() {
    // read all data from a file
    let data = std::fs::read_to_string("data/data.txt");
    let value = data.unwrap();
    println!("{}", value);
}

// for the buffer.lines()
use std::io::prelude::*;

pub fn read_file_tsv() {
    // read a tsv file go through line by line value by value

    //https://doc.rust-lang.org/std/io/trait.BufRead.html

    // open the file
    let file_path = String::from("data/data.tsv");
    println!("{}", file_path);
    let f = std::fs::File::open(file_path).unwrap();

    // go through each line
    let buffer = std::io::BufReader::new(f);
    for line in buffer.lines() {
        let data = line.unwrap();
        print!("{}", data);

        print!(" = ");

        // go through each value in each line
        let values = data.split('\t');
        for value in values {
            print!("[{}]", value);

            // todo: check each value
        }

        println!();
    }
}

// https://github.com/serde-rs/json
// https://docs.serde.rs/serde_json/index.html
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Test {
    name: String,
}

pub fn read_file_json() {
    println!("read_file_json");
    let file_path = String::from("data/data.json");
    let f = std::fs::File::open(file_path).unwrap();
    let v: Test = serde_json::from_reader(f).unwrap();

    // todo: error checking to ensure values are present
    let name = v.name;

    println!("name: [{}]", name);
}

pub fn test_regex() {
    // https://docs.rs/regex/1.5.4/regex/
    let regex_data = String::from(r"^\d{4}-\d{2}-\d{2}$");
    let re = regex::Regex::new(regex_data.as_str()).unwrap();

    // expect true
    let test = String::from("0000-00-00");
    let matched = re.is_match(test.as_str());
    println!("matched: [{}] [{}]", test, matched);

    // expect false
    let test2 = String::from("0000-00-0x");
    let matched2 = re.is_match(test2.as_str());
    println!("matched2: [{}] [{}]", test2, matched2);
}

