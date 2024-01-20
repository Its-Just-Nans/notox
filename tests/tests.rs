#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    const TESTS_FIELDS_NOT_DRY_RUN: notox::OptionnalFields = notox::OptionnalFields {
        dry_run: false,
        verbose: false,
        json: notox::JsonFields {
            json: false,
            json_pretty: false,
            json_error: false,
        },
    };

    #[test]
    fn test_parse_args() {
        let args = vec![
            (
                vec!["notox".to_owned(), "README.md".to_owned()],
                notox::OptionnalFields {
                    dry_run: true,
                    verbose: true,
                    json: notox::JsonFields {
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "README.md".to_owned(), "-d".to_owned()],
                notox::OptionnalFields {
                    dry_run: false,
                    verbose: true,
                    json: notox::JsonFields {
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
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
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
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
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
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
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
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
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
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
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
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "-v".to_owned()],
                notox::OptionnalFields {
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "--version".to_owned()],
                notox::OptionnalFields {
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "-q".to_owned()],
                notox::OptionnalFields {
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["notox".to_owned(), "--quiet".to_owned()],
                notox::OptionnalFields {
                    dry_run: false,
                    verbose: false,
                    json: notox::JsonFields {
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
        ];
        print!("ARGS: {:?}\n", args);
        for one_test in args.iter() {
            let res = notox::parse_args(one_test.0.clone());
            if let Ok(ok_res) = res {
                assert_eq!(ok_res.0, one_test.1);
            } else {
                assert_eq!(res.err().unwrap(), 1)
            }
        }
    }

    #[test]
    fn no_rename() {
        let res = notox::notox(
            &TESTS_FIELDS_NOT_DRY_RUN,
            vec![PathBuf::from("my_file").into()],
        );
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].path, PathBuf::from("my_file"));
        assert_eq!(res[0].modified, None);
    }

    #[test]
    fn rename() {
        let path = PathBuf::from("my?..file");
        let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path.clone().into()]);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].path, path);
        assert_eq!(res[0].modified, Some(PathBuf::from("my_..file")));
    }
    #[test]
    fn rename_ascii() {
        let paths = vec![
            ("my file.ext", "my_file.ext"),
            ("my\"file.ext", "my_file.ext"),
            ("my#file.ext", "my_file.ext"),
            ("my$file.ext", "my_file.ext"),
            ("my%file.ext", "my_file.ext"),
            ("my&file.ext", "my_file.ext"),
            ("my'file.ext", "my_file.ext"),
            ("my(file.ext", "my_file.ext"),
            ("my)file.ext", "my_file.ext"),
            ("my*file.ext", "my_file.ext"),
            // ("my-file.ext", "my_file.ext"),
            // ("my.file.ext", "my_file.ext"),
            // ("my/file.ext", "my_file.ext"),
            ("my:file.ext", "my_file.ext"),
            ("my;file.ext", "my_file.ext"),
            ("my<file.ext", "my_file.ext"),
            ("my=file.ext", "my_file.ext"),
            ("my>file.ext", "my_file.ext"),
            ("my?file.ext", "my_file.ext"),
            ("my@file.ext", "my_file.ext"),
            // ("myAfile.ext", "my_file.ext"),
            // ("myZfile.ext", "my_file.ext"),
            ("my[file.ext", "my_file.ext"),
            (r"my\file.ext", "my_file.ext"),
            ("my]file.ext", "my_file.ext"),
            ("my^file.ext", "my_file.ext"),
            // ("my_file.ext", "my_file.ext"),
            ("my`file.ext", "my_file.ext"),
            // ("myafile.ext", "my_file.ext"),
            // ("myzfile.ext", "my_file.ext"),
            ("my{file.ext", "my_file.ext"),
            ("my|file.ext", "my_file.ext"),
            ("my}file.ext", "my_file.ext"),
            ("my~file.ext", "my_file.ext"),
            ("my\u{007F}file.ext", "my_file.ext"),
        ];
        for one_test in paths.iter() {
            let path_to_test = PathBuf::from(one_test.0);
            let result_to_test = PathBuf::from(one_test.1);
            print!("Testing: {:?} -> {:?}\n", path_to_test, result_to_test);
            let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path_to_test.clone().into()]);
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].path, path_to_test.to_owned());
            assert_eq!(res[0].modified, Some(result_to_test));
        }
    }
    #[test]
    fn ascii_should_not_rename() {
        let paths = vec![
            "my-file.ext",
            "my.file.ext",
            "my/file.ext",
            "myAfile.ext",
            "myZfile.ext",
            "my_file.ext",
            "myafile.ext",
            "myzfile.ext",
        ];
        for one_test in paths.iter() {
            let path_to_test = PathBuf::from(one_test);
            print!("Testing: {:?}\n", path_to_test);
            let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path_to_test.clone().into()]);
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].path, path_to_test.to_owned());
            assert_eq!(res[0].modified, None);
        }
    }
    #[test]
    fn non_ascci_rename() {
        let allow_no_change: Vec<i32> = vec![95]
            .into_iter()
            .chain((45..=47).collect::<Vec<_>>())
            .chain((48..=57).collect::<Vec<_>>()) // numbers
            .chain((65..=90).collect::<Vec<_>>()) // uppercase
            .chain((97..=122).collect::<Vec<_>>()) // lowercase
            .collect();
        let allow_but_change: Vec<i32> = vec![140, 156, 7545, 7549, 8211, 42792, 42793]
            .into_iter()
            .chain((192..=700).collect::<Vec<_>>()) // TODO refine
            .chain((768..=879).collect::<Vec<_>>()) // diacritics  \u{0300} to \u{036F}
            .chain((6832..=6911).collect::<Vec<_>>()) // diacritics  \u{1AB0} to \u{1AFF}
            .chain((7616..=7679).collect::<Vec<_>>()) // diacritics  \u{1DC0} to \u{1DFF}
            .chain((7680..=8000).collect::<Vec<_>>()) // TODO refine
            .chain((8580..=(8580 + 26)).collect::<Vec<_>>()) // TODO refine
            .chain((9398..=(9398 + 26)).collect::<Vec<_>>()) // TODO refine
            .chain((9425..=(9425 + 26)).collect::<Vec<_>>()) // TODO refine
            .chain((11360..=11400).collect::<Vec<_>>()) // TODO refine
            .chain((42802..=43000).collect::<Vec<_>>()) // TODO refine
            .chain((65313..=(65313 + 26)).collect::<Vec<_>>()) // TODO refine
            .chain((65345..=(65345 + 26)).collect::<Vec<_>>()) // TODO refine
            .collect();
        let max_unicode_point = 0x10FFFF;
        for index in 0..(max_unicode_point + 1) {
            let correct_path;
            let unicode_point = index;
            let options = std::char::from_u32(unicode_point as u32);
            if options.is_none() {
                print!("Invalid: {:?}\n", index);
                continue;
            }
            let options = options.unwrap();
            let path_to_test = PathBuf::from(format!("my{}file.ext", options));
            if allow_no_change.contains(&index) {
                correct_path = None;
            } else if allow_but_change.contains(&index) {
                print!(
                    "Passing: {:?} ({:?}) ({})\n",
                    path_to_test,
                    index,
                    options.escape_unicode()
                );
                continue;
            } else {
                correct_path = Some(PathBuf::from("my_file.ext"));
            }
            print!(
                "Testing: {:?} ({:?}) ({})\n",
                path_to_test,
                index,
                options.escape_unicode()
            );
            let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path_to_test.clone().into()]);
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].path, path_to_test.to_owned());
            assert_eq!(res[0].modified, correct_path);
        }
    }
}
