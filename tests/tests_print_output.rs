#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_print_output() {
        let args = vec![
            notox::OptionnalFields {
                dry_run: false,
                verbose: true,
                json: notox::JsonFields {
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            },
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

    fn setup(name1: &PathBuf, name2: &PathBuf) {
        // create
        let to_correct = PathBuf::from(&name1);
        std::fs::File::create(&to_correct).unwrap();

        // set read only
        let read_only = PathBuf::from(&name2);
        std::fs::File::create(&read_only).unwrap();
        let mut perms = std::fs::metadata(&read_only).unwrap().permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(&read_only, perms).unwrap();
    }

    fn cleanup(name1: &PathBuf, read_only: &PathBuf) {
        // remove
        std::fs::remove_file(&name1).unwrap();

        // remove read only
        let mut perms = std::fs::metadata(&read_only).unwrap().permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(&read_only, perms).unwrap();
        // remove
        std::fs::remove_file(&read_only).unwrap();
    }

    #[test]
    fn test_print_output_verbose_dry() {
        let options = notox::OptionnalFields {
            dry_run: true,
            verbose: true,
            json: notox::JsonFields {
                json: false,
                json_pretty: false,
                json_error: false,
            },
        };
        let to_correct = PathBuf::from("tes t verbose dry.txt");
        let read_only = PathBuf::from("test_verbose_dry.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = notox::notox(&options, paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&to_correct, &read_only);
    }

    #[test]
    fn test_print_output_verbose_real() {
        let options = notox::OptionnalFields {
            dry_run: false,
            verbose: true,
            json: notox::JsonFields {
                json: false,
                json_pretty: false,
                json_error: false,
            },
        };
        let to_correct = PathBuf::from("tes t verbose.txt");
        let read_only = PathBuf::from("test_verbose.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = notox::notox(&options, paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_verbose.txt"), &read_only);
    }

    #[test]
    fn test_print_output_json_real() {
        let options = notox::OptionnalFields {
            dry_run: false,
            verbose: false,
            json: notox::JsonFields {
                json: true,
                json_pretty: false,
                json_error: false,
            },
        };
        let to_correct = PathBuf::from("tes t json.txt");
        let read_only = PathBuf::from("test_json.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = notox::notox(&options, paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_json.txt"), &read_only);
    }

    #[test]
    fn test_print_output_json_error_real() {
        let options = notox::OptionnalFields {
            dry_run: false,
            verbose: false,
            json: notox::JsonFields {
                json: true,
                json_pretty: false,
                json_error: true,
            },
        };
        let to_correct = PathBuf::from("tes t json error.txt");
        let read_only = PathBuf::from("test_json_error.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = notox::notox(&options, paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_json_error.txt"), &read_only)
    }

    #[test]
    fn test_print_output_json_error_dry() {
        let options = notox::OptionnalFields {
            dry_run: true,
            verbose: false,
            json: notox::JsonFields {
                json: true,
                json_pretty: false,
                json_error: true,
            },
        };
        let to_correct = PathBuf::from("tes t json error dry.txt");
        let read_only = PathBuf::from("test_json_error_dry.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = notox::notox(&options, paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&to_correct, &read_only)
    }
}
