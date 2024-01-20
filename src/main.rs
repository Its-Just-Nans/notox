use notox::{notox, parse_args, print_output};

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = parse_args(args);
    if let Ok((options, paths_to_check)) = parsed_args {
        let final_res = notox(&options, paths_to_check);
        print_output(&options, final_res);
    }
    Ok(())
}
