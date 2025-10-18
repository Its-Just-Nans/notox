use notox::Notox;

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let exit_code = Notox::run_main_from_args(&args);
    std::process::exit(exit_code);
}
