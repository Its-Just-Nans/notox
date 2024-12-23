#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_args() {
        let args = vec![
            (
                vec!["notox".to_owned(), "README.md".to_owned()],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: true },
                    verbosity: notox::VerbosityFields {
                        verbose: true,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "README.md".to_owned(), "-d".to_owned()],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: false },
                    verbosity: notox::VerbosityFields {
                        verbose: true,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "notox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-j".to_owned(),
                ],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: false },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "notox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json".to_owned(),
                ],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: false },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "notox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-e".to_owned(),
                ],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: false },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: true,
                    },
                },
            ),
            (
                vec![
                    "notox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json-error".to_owned(),
                ],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: false },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: true,
                    },
                },
            ),
            (
                vec![
                    "notox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-p".to_owned(),
                ],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: false },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "notox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json-pretty".to_owned(),
                ],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: false },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "-v".to_owned()],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: true },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "--version".to_owned()],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: true },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "-q".to_owned()],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: true },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "--quiet".to_owned()],
                notox::OptionnalFields {
                    options: notox::OptionsFields { dry_run: true },
                    verbosity: notox::VerbosityFields {
                        verbose: false,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
        ];
        println!("ARGS: {:?}", args);
        for one_test in args.iter() {
            let res = notox::parse_args(&one_test.0.clone());
            if let Ok(ok_res) = res {
                assert_eq!(ok_res.0, one_test.1);
            } else {
                assert_eq!(res.err().unwrap(), 1)
            }
        }
    }

    #[test]
    fn test_parse_args_no_file_found() {
        let vec_args = [
            "notox".to_owned(),
            "README.md".to_owned(),
            "README".to_owned(),
        ];
        let res = notox::parse_args(&vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            notox::OptionnalFields {
                options: notox::OptionsFields { dry_run: true },
                verbosity: notox::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        assert_eq!(vect.len(), 1);
    }

    #[test]
    fn test_parse_args_star() {
        let vec_args = ["notox".to_owned(), "*".to_owned()];
        let res = notox::parse_args(&vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            notox::OptionnalFields {
                options: notox::OptionsFields { dry_run: true },
                verbosity: notox::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        let number = std::fs::read_dir(".")
            .ok()
            .unwrap()
            .filter(|entry| entry.is_ok())
            .count();
        assert_eq!(vect.len(), number);
    }
}
