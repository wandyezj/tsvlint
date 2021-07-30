#![allow(dead_code)]
#![allow(unused_variables)]
use std;
mod test;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MetadataHeader {
    present: bool,
}

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
    header: MetadataHeader,
    fields: Vec<MetadataField>,
    rules: HashMap<String, MetadataRule>,
}

fn get_metadata(metadata_file_path: String) -> Metadata {
    let f = File::open(metadata_file_path).unwrap();
    let metadata: Metadata = serde_json::from_reader(f).unwrap();

    // Do any checks on the metadata
    // Report any invlid regex
    // report any field.rule that do not have a corresponding entry in rules or are not defined internally

    return metadata;
}

struct Parameters {
    program_path: String,
    data_file_path: String,
    metadata_file_path: String,
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
    let mut rules: HashMap<String, Regex> = HashMap::new();

    for (key, value) in metadata.rules.iter() {
        let expression = value.regex.as_ref();
        if expression.is_some() {
            let expression_string = expression.unwrap().as_str();
            // TODO: check will want error checking to know that all regex are valid, this should be done when the metadata is fetched
            let re = Regex::new(expression_string).unwrap();
            rules.insert(key.to_string(), re);
        }
    }

    // Order rules for each value based on entry
    let mut fields_regex: Vec<Option<&Regex>> = Vec::new();
    let mut fields_name: Vec<&String> = Vec::new();
    let name_default = "".to_string();
    let mut fields_rule: Vec<&String> = Vec::new();

    for field in metadata.fields.iter() {
        let rule = field.rule.as_ref();

        let field_name = field.name.as_ref().unwrap_or(&name_default);

        let mut rule_name = &name_default;
        let mut field_regex: Option<&Regex> = None;

        if rule.is_some() {
            rule_name = rule.unwrap();

            let re = rules.get(rule_name).unwrap();
            field_regex = Some(re);
        }

        fields_rule.push(rule_name);
        fields_name.push(field_name);
        fields_regex.push(field_regex);
    }

    // Scan through CSV

    let expected_value_count = fields_regex.len();

    // Open File
    let f = File::open(data_file_path).unwrap();

    // line
    let buffer = BufReader::new(f);
    let mut line_number = -1;
    for line in buffer.lines() {
        line_number += 1;

        let data = line.unwrap();

        // go through each value in each line
        let values_iterator = data.split('\t');
        let mut values: Vec<String> = Vec::new();
        for value in values_iterator {
            values.push(value.to_string());
        }

        // check for correct number of values
        let values_count = values.len();
        if values_count != expected_value_count {
            // TODO: collect information for log
            println!(
                "error: line [{}] has incorrect number of values [{}], expected [{}]",
                line_number, values_count, expected_value_count
            );
            continue;
        }

        for field_number in 0..values_count {
            let value = values.get(field_number).unwrap();
            let re = fields_regex.get(field_number).unwrap();
            let mut is_match = true;
            if re.is_some() {
                is_match = re.unwrap().is_match(value);
            }

            if !is_match {
                //TODO: collect information for log
                println!(
                    "error: line [{}] field [{}] [{}] value [{}] does not match field rule [{}]",
                    line_number + 1, /* line numbers start at 1*/
                    field_number,
                    fields_name.get(field_number).unwrap(),
                    value,
                    fields_rule.get(field_number).unwrap()
                );
            }
        }
    }

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

    let Parameters {
        metadata_file_path,
        data_file_path,
        ..
    } = parameters;
    println!("{}", metadata_file_path);

    let metadata = get_metadata(metadata_file_path);

    scan_csv(metadata, data_file_path);
}
