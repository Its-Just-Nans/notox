use notox::{notox_full, parse_args};

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let (options, paths_to_check) = match parse_args(&args) {
        Ok((options, paths_to_check)) => (options, paths_to_check),
        Err(code) => std::process::exit(code),
    };
    let result_code = notox_full(&options, paths_to_check);
    std::process::exit(result_code);
}
