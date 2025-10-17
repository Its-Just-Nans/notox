#[cfg(test)]
mod tests {
    use std::{collections::HashSet, path::PathBuf};

    use notox::PathChange;
    const TESTS_FIELDS_NOT_DRY_RUN: notox::NotoxArgs = notox::NotoxArgs {
        dry_run: false,
        verbose: false,
        json_pretty: false,
        json_output: Some(notox::JsonOutput::Default),
    };

    #[test]
    fn no_rename() {
        let paths = HashSet::from([PathBuf::from("my_file")]);
        let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, &paths);
        assert_eq!(res.len(), 1);
        let correct_path = PathBuf::from("my_file");
        match &res[0] {
            PathChange::Unchanged { path } => {
                assert_eq!(path, &correct_path);
            }
            _ => panic!("Expected Unchanged"),
        }
    }

    #[test]
    fn rename() {
        let base_path = PathBuf::from("my?..file");
        let paths = HashSet::from([base_path.clone()]);
        let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, &paths);
        assert_eq!(res.len(), 1);
        match &res[0] {
            PathChange::ErrorRename {
                path,
                modified,
                error,
            } => {
                assert_eq!(path, &base_path);
                assert_eq!(modified, &PathBuf::from("my_..file"));
                assert_eq!(error, "No such file or directory (os error 2)");
            }
            _ => {
                println!("Result: {:?}", res[0]);
                panic!("Expected Renamed")
            }
        }
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
            println!("Testing: {:?} -> {:?}", path_to_test, result_to_test);
            let paths = HashSet::from([path_to_test.clone()]);
            let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, &paths);
            assert_eq!(res.len(), 1);
            match &res[0] {
                PathChange::ErrorRename {
                    path,
                    modified,
                    error,
                } => {
                    assert_eq!(path, &path_to_test);
                    assert_eq!(modified, &result_to_test);
                    assert_eq!(error, "No such file or directory (os error 2)")
                }
                _ => {
                    println!("Result: {:?}", res[0]);
                    panic!("Expected Changed");
                }
            }
        }
    }

    #[test]
    fn ascii_should_not_rename() {
        let paths = [
            "my-file.ext",
            "my.file.ext",
            "my/file.ext",
            "my0file.ext",
            "my9file.ext",
            "myAfile.ext",
            "myZfile.ext",
            "my_file.ext",
            "myafile.ext",
            "myzfile.ext",
        ];
        for one_test in paths.iter() {
            let path_to_test = PathBuf::from(one_test);
            println!("Testing: {:?}", path_to_test);
            let res = notox::notox(
                &TESTS_FIELDS_NOT_DRY_RUN,
                &HashSet::from([path_to_test.clone()]),
            );
            assert_eq!(res.len(), 1);
            match &res[0] {
                PathChange::Unchanged { path } => {
                    assert_eq!(path, &path_to_test);
                }
                _ => panic!("Expected Unchanged"),
            }
        }
    }

    fn is_allowed_but_changed(current_char: char) -> (bool, String) {
        let mut acc: String = String::new();
        notox::check_similar(Some(current_char), &mut acc, false);
        if acc == "_" {
            return (false, acc);
        }
        (true, acc)
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
        let diacritics: Vec<i32> = vec![]
            .into_iter()
            .chain((768..=879).collect::<Vec<_>>()) // diacritics  \u{0300} to \u{036F}
            .chain((6832..=6911).collect::<Vec<_>>()) // diacritics  \u{1AB0} to \u{1AFF}
            .chain((7616..=7679).collect::<Vec<_>>()) // diacritics  \u{1DC0} to \u{1DFF}
            .collect();
        let max_unicode_point = 0x10FFFF;
        for index in 0..(max_unicode_point + 1) {
            let correct_path;
            let unicode_point = index;
            let optional_current = std::char::from_u32(unicode_point as u32);
            let current_char = match optional_current {
                Some(c) => c,
                None => {
                    println!("Invalid: {:?}", index);
                    continue;
                }
            };
            let path_to_test = PathBuf::from(format!("my{}file.ext", current_char));
            let (boo, acc) = is_allowed_but_changed(current_char);
            if allow_no_change.contains(&index) {
                correct_path = PathBuf::from("UNCHANGED");
            } else if diacritics.contains(&index) {
                correct_path = PathBuf::from("myfile.ext");
            } else if boo {
                // here format
                let corrected = format!("my{}file.ext", acc);
                let corrected = PathBuf::from(corrected);
                println!(
                    "Changed: {:?} ({:?}) ({}) -> {:?}",
                    path_to_test,
                    index,
                    current_char.escape_unicode(),
                    corrected
                );
                correct_path = corrected;
            } else {
                correct_path = PathBuf::from("my_file.ext");
            }
            println!(
                "Testing: {:?} ({:?}) ({})",
                path_to_test,
                index,
                current_char.escape_unicode()
            );
            let paths = HashSet::from([path_to_test.clone()]);
            let res = notox::notox(&TESTS_FIELDS_NOT_DRY_RUN, &paths);
            assert_eq!(res.len(), 1);
            match &res[0] {
                PathChange::ErrorRename {
                    path,
                    modified,
                    error,
                } => {
                    assert_eq!(path, &path_to_test);
                    assert_eq!(modified, &correct_path);
                    if index == 0 {
                        assert_eq!(error, "file name contained an unexpected NUL byte");
                    } else {
                        assert_eq!(error, "No such file or directory (os error 2)");
                    }
                }
                PathChange::Unchanged { path } => {
                    assert_eq!(path, &path_to_test);
                    assert_eq!(correct_path, PathBuf::from("UNCHANGED"));
                }
                _ => {
                    println!("Result: {:?}", res[0]);
                    panic!("Expected Changed or Unchanged")
                }
            }
        }
    }

    #[test]
    fn test_grapheme_four_conversion() {
        // 4 bytes grapheme
        let four_bytes = '💣'; // https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8
        assert_eq!(four_bytes.len_utf8(), 4);
        let mut four_bytes_str = String::new();
        four_bytes_str.push(four_bytes);
        assert_eq!(four_bytes_str.chars().count(), 1);
        assert_eq!(four_bytes_str.chars().next().unwrap(), four_bytes);
        let four_as_bytes = four_bytes_str.as_bytes();
        let four_bytes_u32 = notox::convert_four_to_u32(
            four_as_bytes[0],
            four_as_bytes[1],
            four_as_bytes[2],
            four_as_bytes[3],
        );
        assert_eq!(four_bytes_u32, four_bytes as u32);
        let curr_char: Option<char> = std::char::from_u32(four_bytes_u32);
        assert_eq!(curr_char, Some(four_bytes));
    }

    #[test]
    fn test_grapheme_three_conversion() {
        // 3 bytes grapheme
        let three_bytes = 'ℝ'; // https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8
        assert_eq!(three_bytes.len_utf8(), 3);
        let mut three_bytes_str = String::new();
        three_bytes_str.push(three_bytes);
        assert_eq!(three_bytes_str.chars().count(), 1);
        assert_eq!(three_bytes_str.chars().next().unwrap(), three_bytes);
        let three_as_bytes = three_bytes_str.as_bytes();
        let three_bytes_u32 =
            notox::convert_three_to_u32(three_as_bytes[0], three_as_bytes[1], three_as_bytes[2]);
        assert_eq!(three_bytes_u32, three_bytes as u32);
        let curr_char: Option<char> = std::char::from_u32(three_bytes_u32);
        assert_eq!(curr_char, Some(three_bytes));
    }

    #[test]
    fn test_grapheme_two_conversion() {
        // 2 bytes grapheme
        let two_bytes = 'ß'; // https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8
        assert_eq!(two_bytes.len_utf8(), 2);
        let mut two_bytes_str = String::new();
        two_bytes_str.push(two_bytes);
        assert_eq!(two_bytes_str.chars().count(), 1);
        assert_eq!(two_bytes_str.chars().next().unwrap(), two_bytes);
        let two_as_bytes = two_bytes_str.as_bytes();
        let two_bytes_u32 = notox::convert_two_to_u32(two_as_bytes[0], two_as_bytes[1]);
        assert_eq!(two_bytes_u32, two_bytes as u32);
        let curr_char: Option<char> = std::char::from_u32(two_bytes_u32);
        assert_eq!(curr_char, Some(two_bytes));
    }

    #[test]
    fn test_grapheme_one_conversion() {
        // 1 bytes grapheme
        let one_bytes = 'A'; // https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8
        assert_eq!(one_bytes.len_utf8(), 1);
        let mut one_bytes_str = String::new();
        one_bytes_str.push(one_bytes);
        assert_eq!(one_bytes_str.chars().count(), 1);
        assert_eq!(one_bytes_str.chars().next().unwrap(), one_bytes);
        let one_as_bytes = one_bytes_str.as_bytes();
        let one_bytes_u32 = one_as_bytes[0] as u32;
        assert_eq!(one_bytes_u32, one_bytes as u32);
        let curr_char: Option<char> = std::char::from_u32(one_bytes_u32);
        assert_eq!(curr_char, Some(one_bytes));
    }
}
