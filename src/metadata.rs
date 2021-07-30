use std::collections::HashMap;
use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetadataHeader {
    pub present: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataField {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rule: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataRule {
    pub name: Option<String>,
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: Option<String>,
    pub header: MetadataHeader,
    pub fields: Vec<MetadataField>,
    pub rules: HashMap<String, MetadataRule>,
}

pub fn get_metadata(metadata_file_path: String) -> Metadata {
    let f = File::open(metadata_file_path).unwrap();
    let metadata: Metadata = serde_json::from_reader(f).unwrap();

    // Do any checks on the metadata
    // Report any invlid regex
    // report any field.rule that do not have a corresponding entry in rules or are not defined internally

    return metadata;
}