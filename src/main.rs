#![allow(dead_code)]
#![allow(unused_variables)]

mod test;
mod metadata;
mod parameters;
mod scan;

use crate::metadata::get_metadata;

use crate::parameters::get_parameters;
use crate::parameters::Parameters;

use scan::scan_csv;


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
