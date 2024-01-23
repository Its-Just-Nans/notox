#[cfg(test)]
mod tests {
    use notox::OptionnalFields;

    #[test]
    fn test_parse_args_clean_directory() {
        let dir = "src".to_owned();
        let vec_args = vec!["notox".to_owned(), dir.clone(), "-j".to_owned()];
        let res = notox::parse_args(vec_args);
        let (options, vect) = res.ok().unwrap();
        let res_path = notox::notox(&options, vect);
        let number = std::fs::read_dir(dir)
            .ok()
            .unwrap()
            .filter(|entry| entry.is_ok())
            .count();
        assert_eq!(
            options,
            OptionnalFields {
                dry_run: true,
                verbose: false,
                json: notox::JsonFields {
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
        let res = notox::check_similar(vec![], &mut String::new(), false);
        assert_eq!(res, false);
    }
}
