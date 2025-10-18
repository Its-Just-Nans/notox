#[cfg(test)]
mod tests {
    use std::{collections::HashSet, path::PathBuf};

    #[cfg(feature = "serde")]
    use notox::JsonOutput;
    use notox::{NotoxArgs, Output};

    #[test]
    fn test_print_output() {
        let args = [
            NotoxArgs {
                dry_run: true,
                output: Output::Default,
            },
            NotoxArgs {
                dry_run: false,
                output: Output::Default,
            },
            NotoxArgs {
                dry_run: true,
                output: Output::Quiet,
            },
            NotoxArgs {
                dry_run: false,
                output: Output::Quiet,
            },
            #[cfg(feature = "serde")]
            NotoxArgs {
                dry_run: true,
                output: Output::JsonOutput {
                    json: JsonOutput::JsonDefault,
                    pretty: false,
                },
            },
            #[cfg(feature = "serde")]
            NotoxArgs {
                dry_run: false,
                output: Output::JsonOutput {
                    json: JsonOutput::JsonDefault,
                    pretty: false,
                },
            },
            #[cfg(feature = "serde")]
            NotoxArgs {
                dry_run: false,
                output: Output::JsonOutput {
                    json: JsonOutput::JsonDefault,
                    pretty: true,
                },
            },
            #[cfg(feature = "serde")]
            NotoxArgs {
                dry_run: true,
                output: Output::JsonOutput {
                    json: JsonOutput::JsonOnlyError,
                    pretty: false,
                },
            },
            #[cfg(feature = "serde")]
            NotoxArgs {
                dry_run: false,
                output: Output::JsonOutput {
                    json: JsonOutput::JsonOnlyError,
                    pretty: false,
                },
            },
            #[cfg(feature = "serde")]
            NotoxArgs {
                dry_run: false,
                output: Output::JsonOutput {
                    json: JsonOutput::JsonOnlyError,
                    pretty: true,
                },
            },
        ];
        for options in args.iter() {
            let paths_to_check = HashSet::from([PathBuf::from("README.md")]);
            let final_res = notox::notox(options, &paths_to_check);
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
        std::fs::remove_file(name1).unwrap();

        // remove read only
        let mut perms = std::fs::metadata(read_only).unwrap().permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            perms.set_mode(0o644);
        }
        #[cfg(not(unix))]
        {
            perms.set_readonly(false);
        }
        std::fs::set_permissions(read_only, perms).unwrap();
        // remove
        std::fs::remove_file(read_only).unwrap();
    }

    #[test]
    fn test_print_output_verbose_dry() {
        let options = NotoxArgs {
            dry_run: true,
            output: Output::Default,
        };
        let to_correct = PathBuf::from("tes t verbose dry.txt");
        let read_only = PathBuf::from("test_verbose_dry.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = HashSet::from([
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ]);
        let final_res = notox::notox(&options, &paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&to_correct, &read_only);
    }

    #[test]
    fn test_print_output_verbose_real() {
        let options = NotoxArgs {
            dry_run: false,
            output: Output::Default,
        };
        let to_correct = PathBuf::from("tes t verbose.txt");
        let read_only = PathBuf::from("test_verbose.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = HashSet::from([
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ]);
        let final_res = notox::notox(&options, &paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_verbose.txt"), &read_only);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_print_output_json_real() {
        let options = NotoxArgs {
            dry_run: false,
            output: Output::JsonOutput {
                json: JsonOutput::JsonDefault,
                pretty: false,
            },
        };
        let to_correct = PathBuf::from("tes t json.txt");
        let read_only = PathBuf::from("test_json.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = HashSet::from([
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ]);
        let final_res = notox::notox(&options, &paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_json.txt"), &read_only);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_print_output_json_error_real() {
        let options = NotoxArgs {
            dry_run: false,
            output: Output::JsonOutput {
                json: JsonOutput::JsonOnlyError,
                pretty: false,
            },
        };
        let to_correct = PathBuf::from("tes t json error.txt");
        let read_only = PathBuf::from("test_json_error.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = HashSet::from([
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ]);
        let final_res = notox::notox(&options, &paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_json_error.txt"), &read_only)
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_print_output_json_error_dry() {
        let options = NotoxArgs {
            dry_run: true,
            output: Output::JsonOutput {
                json: JsonOutput::JsonOnlyError,
                pretty: false,
            },
        };
        let to_correct = PathBuf::from("tes t json error dry.txt");
        let read_only = PathBuf::from("test_json_error_dry.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = HashSet::from([
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ]);
        let final_res = notox::notox(&options, &paths_to_check);
        notox::print_output(&options, final_res).unwrap();

        // cleanup
        cleanup(&to_correct, &read_only)
    }
}
