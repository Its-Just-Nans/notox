use notox::{notox_full, parse_args};

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let exit_code = match parse_args(&args) {
        Ok((options, paths)) => notox_full(&options, paths),
        Err(code) => code,
    };
    std::process::exit(exit_code);
}
