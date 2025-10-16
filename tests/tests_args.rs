#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_args() {
        let args = vec![
            (
                vec!["notox".to_string(), "README.md".to_string()],
                notox::NotoxArgs {
                    dry_run: true,
                    json_pretty: false,
                    verbose: true,
                    json_output: None,
                },
            ),
            (
                vec![
                    "notox".to_string(),
                    "README.md".to_string(),
                    "-d".to_string(),
                ],
                notox::NotoxArgs {
                    dry_run: false,
                    verbose: true,
                    json_pretty: false,
                    json_output: None,
                },
            ),
            (
                vec![
                    "notox".to_string(),
                    "README.md".to_string(),
                    "-d".to_string(),
                    "-j".to_string(),
                ],
                notox::NotoxArgs {
                    dry_run: false,
                    verbose: false,
                    json_pretty: false,
                    json_output: Some(notox::JsonOutput::Default),
                },
            ),
            (
                vec![
                    "notox".to_string(),
                    "README.md".to_string(),
                    "-d".to_string(),
                    "--json".to_string(),
                ],
                notox::NotoxArgs {
                    dry_run: false,
                    verbose: false,
                    json_pretty: false,
                    json_output: Some(notox::JsonOutput::Default),
                },
            ),
            (
                vec![
                    "notox".to_string(),
                    "README.md".to_string(),
                    "-d".to_string(),
                    "-e".to_string(),
                ],
                notox::NotoxArgs {
                    dry_run: false,
                    verbose: false,
                    json_pretty: false,
                    json_output: Some(notox::JsonOutput::OnlyError),
                },
            ),
            (
                vec![
                    "notox".to_string(),
                    "README.md".to_string(),
                    "-d".to_string(),
                    "--json-error".to_string(),
                ],
                notox::NotoxArgs {
                    dry_run: false,
                    verbose: false,
                    json_pretty: false,
                    json_output: Some(notox::JsonOutput::OnlyError),
                },
            ),
            (
                vec![
                    "notox".to_string(),
                    "README.md".to_string(),
                    "-d".to_string(),
                    "-p".to_string(),
                ],
                notox::NotoxArgs {
                    dry_run: false,
                    verbose: false,
                    json_pretty: true,
                    json_output: Some(notox::JsonOutput::Default),
                },
            ),
            (
                vec![
                    "notox".to_string(),
                    "README.md".to_string(),
                    "-d".to_string(),
                    "--json-pretty".to_string(),
                ],
                notox::NotoxArgs {
                    dry_run: false,
                    verbose: false,
                    json_pretty: true,
                    json_output: Some(notox::JsonOutput::Default),
                },
            ),
            (
                vec!["notox".to_string(), "-v".to_string()],
                notox::NotoxArgs {
                    dry_run: true,
                    verbose: false,
                    json_pretty: true,
                    json_output: None,
                },
            ),
            (
                vec!["notox".to_string(), "--version".to_string()],
                notox::NotoxArgs {
                    dry_run: true,
                    verbose: false,
                    json_pretty: true,
                    json_output: None,
                },
            ),
            (
                vec!["notox".to_string(), "-q".to_string()],
                notox::NotoxArgs {
                    dry_run: true,
                    verbose: false,
                    json_pretty: false,
                    json_output: None,
                },
            ),
            (
                vec!["notox".to_string(), "--quiet".to_string()],
                notox::NotoxArgs {
                    dry_run: true,
                    verbose: false,
                    json_pretty: false,
                    json_output: None,
                },
            ),
        ];
        println!("ARGS: {:?}", args);
        for one_test in args.iter() {
            let res = notox::parse_args(&one_test.0.clone());
            if let Ok(ok_res) = res {
                assert_eq!(ok_res.0, one_test.1, "Args: {:?}", one_test.0);
            } else {
                println!("Error parsing args: {:?} {:?}", res, one_test.0);
                assert_eq!(res.err().unwrap(), 1);
            }
        }
    }

    #[test]
    fn test_parse_args_no_file_found() {
        let vec_args = [
            "notox".to_string(),
            "README.md".to_string(),
            "README".to_string(),
        ];
        let res = notox::parse_args(&vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            notox::NotoxArgs {
                dry_run: true,
                verbose: true,
                json_pretty: false,
                json_output: None,
            }
        );
        assert_eq!(vect.len(), 1);
    }

    #[test]
    fn test_parse_args_star() {
        let vec_args = ["notox".to_string(), "*".to_string()];
        let res = notox::parse_args(&vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            notox::NotoxArgs {
                dry_run: true,
                verbose: true,
                json_pretty: false,
                json_output: None,
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
