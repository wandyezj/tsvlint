/*
File to contain all test functions to show proof of concept functionality

These should eventually serve as pin tests.

*/

// global ignore dead code warnings, while developing test functions.
#![allow(dead_code)]
#![allow(unused_imports)]
#[cfg(test)]
#[test]
pub fn command_line_arguments_read() {
    // basic output
    println!("Hello, world!");

    // command line arguments
    for argument in std::env::args() {
        println!("->{}", argument);
    }
}

#[test]
pub fn command_line_arguments_individual() {
    // get individual command line arguments distinguished by position
    let args: Vec<String> = std::env::args().collect();
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
#[test]
pub fn read_file_data() {
    // data.txt
    /*
    Hello World!

    From Data
    */

    // read all data from a file
    let data = std::fs::read_to_string("data/test/data.txt");
    let value = data.unwrap();
    println!("{}", value);
}

// for the buffer.lines()
use std::io::prelude::*;

// data.tsv
/*
one	two	three
1	2	3
uno	dos	tres
hannah	dul	set
un	deux	trois
*/

#[test]
pub fn read_file_tsv() {
    // read a tsv file go through line by line value by value

    //https://doc.rust-lang.org/std/io/trait.BufRead.html

    // open the file
    let file_path = String::from("data/test/data.tsv");
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
struct SimpleJson {
    name: String,
}

// simple-data.json
/*
{
    "name":"MY NAME"
}
    */

#[test]
pub fn read_file_json_simple() {
    println!("read_file_json");
    let file_path = String::from("data/test/simple-data.json");
    let f = std::fs::File::open(file_path).unwrap();
    let v: SimpleJson = serde_json::from_reader(f).unwrap();

    // todo: error checking to ensure values are present
    let name = v.name;

    println!("name: [{}]", name);
}

#[derive(Serialize, Deserialize)]
struct ComplexJsonObject {
    name: String,
    // https://doc.rust-lang.org/std/option/
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ComplexJson {
    name: String,
    object: ComplexJsonObject,
    array: Vec<String>,
    object_array: Vec<ComplexJsonObject>,
}
#[test]
pub fn read_file_json_complex() {
    println!("read_file_json_complex");
    let file_path = String::from("data/test/data.json");
    let f = std::fs::File::open(file_path).unwrap();
    let v: ComplexJson = serde_json::from_reader(f).unwrap();

    // todo: error checking to ensure values are present
    let name = v.name;
    println!(".name [{}]", name);

    let object_name = v.object.name;

    let object_description = v.object.description;
    let object_description_is_none = object_description.is_some(); // .is_none();
    let object_description_string = object_description.unwrap_or(String::new()); // give a default value
    println!(".object.name [{}]", object_name);
    println!(
        ".object.description is_none [{}]",
        object_description_is_none
    );
    println!(".object.description [{}]", object_description_string);

    let array = v.array;
    println!(".array len [{}]", array.len());
    println!(".array join [{}]", array.join(", "));

    let object_array = v.object_array;
    println!(".object_array len [{}]", object_array.len());
}

#[test]
pub fn test_regex() {
    // https://docs.rs/regex/1.5.4/regex/
    let regex_data = String::from(r"^\d{4}-\d{2}-\d{2}$");
    let re = regex::Regex::new(regex_data.as_str()).unwrap();

    // expect true
    let test = String::from("0000-00-00");
    let matched = re.is_match(test.as_str());
    println!("matched: [{}] [{}]", test, matched);
    assert_eq!(matched, true);

    // expect false
    let test2 = String::from("0000-00-0x");
    let matched2 = re.is_match(test2.as_str());
    println!("matched2: [{}] [{}]", test2, matched2);
    assert_eq!(matched2, false);
}
