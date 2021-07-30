pub struct Parameters {
    pub program_path: String,
    pub data_file_path: String,
    pub metadata_file_path: String,
}

pub fn get_parameters() -> (bool, Vec<String>, Parameters) {
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
