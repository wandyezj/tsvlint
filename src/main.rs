#![allow(dead_code)]

mod test;



use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MetadataField {
    name: Option<String>,
    description: Option<String>,
    rule: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct MetadataRule {
    name: Option<String>,
    regex: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    description: Option<String>,
    fields: Vec<MetadataField>,
    rules: std::collections::HashMap<String, String>
}


struct Parameters {
    program_path: String,
    data_file_path: String,
    metadata_file_path: String,
}
use std;

fn get_parameters() -> (bool, Vec<String>, Parameters) {
    let args:Vec<String> = std::env::args().collect();
    let total_arguments = args.len();

    if total_arguments != 3 {
        let parameters = Parameters {
            program_path: String::from(""),
            data_file_path: String::from(""),
            metadata_file_path: String::from("")
        };
        return (false, args, parameters);
    }

    // parse arguments
    let zero = &args[0];
    let one = &args[1];
    let two = &args[2];

    let parameters = Parameters {
        program_path: String::from(zero),
        data_file_path: String::from(one),
        metadata_file_path: String::from(two)
    };

    return (true, args, parameters);
}


fn main() {

    let (success, args, parameters) = get_parameters();
    println!("{}", args.join(" "));
    if !success {
        println!("usage: [metadata.json] [data.csv]");
        return;
    }
    
}
