use crate::metadata::Metadata;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub trait Log {
    fn log(&mut self, message: &String);
}

pub struct ScanReporter {
    /**
     * print when logged
     */
    print_log: bool,
    messages: Vec<String>,
}

impl ScanReporter {
    pub fn new(print_log: bool) -> ScanReporter {
        ScanReporter {
            messages: Vec::new(),
            print_log
        }
    }

    pub fn print(&self) {
        for message in self.messages.iter() {
            println!("{}",message);
            
        }
    }
}

impl Log for ScanReporter {
    fn log(&mut self, message: &String) {
        self.messages.push(message.clone());
        if self.print_log {
            println!("{}", message);
        }
    }
}

pub fn scan_csv(metadata: Metadata, data_file_path: String, report: &mut dyn Log) {
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

    let mut lines = buffer.lines();

    // handle header
    // skip header line if present
    if metadata.header.present {
        let line = lines.next();
        let data = line.unwrap().unwrap();
        report.log(&"header".to_string());
        report.log(&data);
        //println!("header");
        //println!("{}", data);

        // TODO: validate that specified field header lines up with actual header value
    }

    for line in lines {
        line_number += 1;

        let data = line.unwrap();

        // go through each value in each line
        let values_iterator = data.split('\t');
        let mut values: Vec<String> = Vec::new();
        for value in values_iterator {
            values.push(value.to_string());
        }

        // ensure line has correct number of values according to metadata
        let values_count = values.len();
        if values_count != expected_value_count {
            // TODO: collect information for log
            let message = &format!(
                "error: line [{}] has incorrect number of values [{}], expected [{}]",
                line_number, values_count, expected_value_count
            );
            report.log(message);
            //println!("{}", message);
            continue;
        }

        // value
        for field_number in 0..values_count {
            let value = values.get(field_number).unwrap();
            let re = fields_regex.get(field_number).unwrap();
            let mut is_match = true;
            if re.is_some() {
                is_match = re.unwrap().is_match(value);
            }

            if !is_match {
                //TODO: collect information for log
                let message = &format!(
                    "error: (value does not match rule) line [{line_number}] field [{field_number}] [{field_name}] rule [{field_rule}] value [{value}]",
                    line_number = line_number + 1, /* line numbers start at 1*/
                    field_number = field_number,
                    field_name = fields_name.get(field_number).unwrap(),
                    value = value,
                    field_rule = fields_rule.get(field_number).unwrap(),
                );
                report.log(message);
                //println!("{message}", message=message)
            }
        }
    }
}
