#![allow(dead_code)]
#![allow(unused_variables)]
use std;
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
struct MetadataHeader {
    present: bool,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    description: Option<String>,
    header: MetadataHeader,
    fields: Vec<MetadataField>,
    rules: std::collections::HashMap<String, MetadataRule>,
}

struct Parameters {
    program_path: String,
    data_file_path: String,
    metadata_file_path: String,
}

fn get_metadata(metadata_file_path: String) -> Metadata {
    let f = std::fs::File::open(metadata_file_path).unwrap();
    let metadata: Metadata = serde_json::from_reader(f).unwrap();

    // Do any checks on the metadata

    return metadata;
}


fn get_parameters() -> (bool, Vec<String>, Parameters) {
    let args: Vec<String> = std::env::args().collect();
    let total_arguments = args.len();

    if total_arguments != 3 {
        let parameters = Parameters {
            program_path: String::from(""),
            data_file_path: String::from(""),
            metadata_file_path: String::from(""),
        };
        return (false, args, parameters);
    }

    // parse arguments
    let zero = &args[0];
    let one = &args[1];
    let two = &args[2];

    let parameters = Parameters {
        program_path: String::from(zero),
        
        metadata_file_path: String::from(one),
        data_file_path: String::from(two),
    };

    return (true, args, parameters);
}

fn scan_csv(metadata: Metadata, data_file_path: String) {

    // Initialize Regex

    // Scan through CSV

    // line

    // ensure line has correct number of values according to metadata

    // value

}


fn main() {
    let (success, args, parameters) = get_parameters();
    println!("{}", args.join(" "));
    if !success {
        println!("usage: [metadata.json] [data.csv]");
        return;
    }

    let Parameters {metadata_file_path, data_file_path, ..} = parameters;
    println!("{}", metadata_file_path);

    let metadata = get_metadata(metadata_file_path);

    scan_csv(metadata, data_file_path);
}


