#[cfg(test)]
mod tests {
    use std::{os::unix::fs::PermissionsExt, path::PathBuf};

    #[test]
    fn test_parse_args_clean_directory() {
        let dir = "src".to_string();
        let vec_args = ["notox".to_string(), dir.clone(), "-j".to_string()];
        let res = notox::parse_args(&vec_args);
        let (options, vect) = res.ok().unwrap();
        let res_path = notox::notox(&options, &vect);
        let number = std::fs::read_dir(dir)
            .ok()
            .unwrap()
            .filter(|entry| entry.is_ok())
            .count();
        assert_eq!(
            options,
            notox::OptionalFields {
                options: notox::OptionsFields { dry_run: true },
                verbosity: notox::VerbosityFields {
                    verbose: false,
                    json: true,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        assert_eq!(res_path.len(), number + 1);
    }

    #[test]
    fn test_check_similars_void() {
        let res = notox::check_similar(None, &mut String::new(), false);
        assert!(!res);
    }

    fn setup(dir: &String) {
        let directory_path = PathBuf::from(dir);
        std::fs::remove_dir_all(&directory_path).unwrap_or(());
        std::fs::create_dir(&directory_path).unwrap();
        std::fs::File::create(directory_path.join("test 1.txt")).unwrap();
        std::fs::File::create(directory_path.join("test_2.txt")).unwrap();
        let mut second_level = dir.clone();
        second_level.push_str(" second");
        let second_level = &directory_path.join(&second_level);
        std::fs::create_dir(second_level).unwrap();
        std::fs::File::create(second_level.join("test 3.txt")).unwrap();
        std::fs::File::create(second_level.join("test_4.txt")).unwrap();
        std::fs::File::create(second_level.join("test_5.txt")).unwrap();

        // permissions file
        let mut perms = std::fs::metadata(second_level.join("test_5.txt"))
            .unwrap()
            .permissions();
        perms.set_mode(0o000);
        std::fs::set_permissions(second_level.join("test_5.txt"), perms).unwrap();
        // permissions firectory
        let mut perms = std::fs::metadata(second_level).unwrap().permissions();
        perms.set_mode(0o000);
        std::fs::set_permissions(second_level, perms).unwrap();
    }

    fn cleanup(dir: &String) {
        let directory_path = PathBuf::from(&dir);
        std::fs::remove_file(directory_path.join("test_1.txt")).unwrap();
        std::fs::remove_file(directory_path.join("test_2.txt")).unwrap();
        let mut second_level = dir.clone();
        second_level.push_str("_second");
        let second_level = &directory_path.join(&second_level);
        let mut perms = std::fs::metadata(second_level).unwrap().permissions();
        perms.set_mode(0o777);
        std::fs::set_permissions(second_level, perms).unwrap();
        std::fs::remove_file(second_level.join("test 3.txt")).unwrap();
        std::fs::remove_file(second_level.join("test_4.txt")).unwrap();

        // test 5
        let mut perms = std::fs::metadata(second_level.join("test_5.txt"))
            .unwrap()
            .permissions();
        perms.set_mode(0o777);
        std::fs::set_permissions(second_level.join("test_5.txt"), perms).unwrap();
        std::fs::remove_file(second_level.join("test_5.txt")).unwrap();

        // other
        std::fs::remove_dir(second_level).unwrap();
        std::fs::remove_dir_all(&directory_path).unwrap();
    }

    #[test]
    fn test_parse_args_clean_directory_recursive() {
        let dir = "test_folder".to_string();
        setup(&dir);

        let vec_args = [
            "notox".to_string(),
            dir.clone(),
            "-j".to_string(),
            "-d".to_string(),
        ];
        let res = notox::parse_args(&vec_args);
        let (options, vect) = res.ok().unwrap();
        let res_path = notox::notox(&options, &vect);

        assert_eq!(
            options,
            notox::OptionalFields {
                options: notox::OptionsFields { dry_run: false },
                verbosity: notox::VerbosityFields {
                    verbose: false,
                    json: true,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        assert_eq!(res_path.len(), 5);
        cleanup(&dir);
    }

    #[test]
    fn test_parse_args_clean_directory_recursive_verbose() {
        let dir = "test_folder_verbose".to_string();
        setup(&dir);

        let vec_args = ["notox".to_string(), dir.clone(), "-d".to_string()];
        let res = notox::parse_args(&vec_args);
        let (options, vect) = res.ok().unwrap();
        let res_path = notox::notox(&options, &vect);

        assert_eq!(
            options,
            notox::OptionalFields {
                options: notox::OptionsFields { dry_run: false },
                verbosity: notox::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        assert_eq!(res_path.len(), 5);
        cleanup(&dir);
    }
}
