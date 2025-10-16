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
            notox::NotoxArgs {
                dry_run: true,
                verbose: false,
                json_pretty: false,
                json_output: Some(notox::JsonOutput::Default),
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

        cleanup(dir);
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
        let text_1 = directory_path.join("test_1.txt");
        if text_1.exists() {
            std::fs::remove_file(text_1).unwrap();
        }
        let text_2 = directory_path.join("test_2.txt");
        if text_2.exists() {
            std::fs::remove_file(text_2).unwrap();
        }
        let mut second_level = dir.clone();
        second_level.push_str("_second");
        let second_level = &directory_path.join(&second_level);
        if second_level.exists() && second_level.is_dir() {
            let mut perms = std::fs::metadata(second_level).unwrap().permissions();
            perms.set_mode(0o777);
            std::fs::set_permissions(second_level, perms).unwrap();
        }
        let text_3 = second_level.join("test 3.txt");
        if text_3.exists() {
            std::fs::remove_file(text_3).unwrap();
        }
        let text_4 = second_level.join("test_4.txt");
        if text_4.exists() {
            std::fs::remove_file(text_4).unwrap();
        }

        let text_5 = second_level.join("test_5.txt");
        if text_5.exists() {
            let mut perms = std::fs::metadata(&text_5).unwrap().permissions();
            perms.set_mode(0o777);
            std::fs::set_permissions(&text_5, perms).unwrap();
            std::fs::remove_file(text_5).unwrap();
        }
        // test 5

        // other
        if second_level.exists() && second_level.is_dir() {
            std::fs::remove_dir(second_level).unwrap();
        }
        if directory_path.exists() && directory_path.is_dir() {
            std::fs::remove_dir(&directory_path).unwrap();
        }
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
            notox::NotoxArgs {
                dry_run: false,
                verbose: false,
                json_pretty: false,
                json_output: Some(notox::JsonOutput::Default),
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
            notox::NotoxArgs {
                dry_run: false,
                verbose: true,
                json_pretty: false,
                json_output: None,
            }
        );
        assert_eq!(res_path.len(), 5);
        cleanup(&dir);
    }
}
