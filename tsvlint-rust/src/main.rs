//use std::env;
use std::io::prelude::*;


fn read_arguments() {

    // basic output
    println!("Hello, world!");

    // command line arguments
    for argument in std::env::args() {
        println!("->{}", argument);
    }
}

fn read_file_data() {
    // read all data from a file
    let data = std::fs::read_to_string("data/data.txt");
    let value = data.unwrap();
    println!("{}", value);
}

fn read_file_tsv() {
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
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Test {
    name: String,
}

fn read_file_json() {
    println!("read_file_json");
    let file_path = String::from("data/data.json");
    let f = std::fs::File::open(file_path).unwrap();
    let v: Test = serde_json::from_reader(f).unwrap();

    // todo: error checking to ensure values are present
    let name = v.name;

    println!("name: [{}]", name);
}

fn main() {

    read_file_json();

}
