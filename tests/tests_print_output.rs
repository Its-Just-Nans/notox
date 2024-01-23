#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_parse_args() {
        let args = vec![
            notox::OptionnalFields {
                dry_run: true,
                verbose: true,
                json: notox::JsonFields {
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            },
            notox::OptionnalFields {
                dry_run: true,
                verbose: false,
                json: notox::JsonFields {
                    json: true,
                    json_pretty: false,
                    json_error: false,
                },
            },
            notox::OptionnalFields {
                dry_run: true,
                verbose: false,
                json: notox::JsonFields {
                    json: true,
                    json_pretty: false,
                    json_error: true,
                },
            },
            notox::OptionnalFields {
                dry_run: true,
                verbose: false,
                json: notox::JsonFields {
                    json: true,
                    json_pretty: true,
                    json_error: false,
                },
            },
            notox::OptionnalFields {
                dry_run: true,
                verbose: false,
                json: notox::JsonFields {
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            },
        ];
        for options in args.iter() {
            let paths_to_check = vec![PathBuf::from("README.md")];
            let final_res = notox::notox(options, paths_to_check);
            notox::print_output(options, final_res).unwrap();
        }
    }
}
