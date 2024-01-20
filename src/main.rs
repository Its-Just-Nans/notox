use notox::{notox, parse_args, print_output};

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = parse_args(args);
    if let Err(code) = parsed_args {
        std::process::exit(code);
    } else if let Ok((options, paths_to_check)) = parsed_args {
        let final_res = notox(&options, paths_to_check);
        if let Err(code) = print_output(&options, final_res) {
            std::process::exit(code);
        }
    }
    Ok(())
}
