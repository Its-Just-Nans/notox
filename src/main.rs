use notox::{parse_args, Notox};

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let exit_code = match parse_args(&args) {
        Ok((options, paths)) => Notox::new(options).run_and_print(&paths),
        Err(code) => code,
    };
    std::process::exit(exit_code);
}
