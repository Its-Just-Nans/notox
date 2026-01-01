#[cfg(test)]
mod tests {
    // Add methods on commands
    use assert_cmd::prelude::*;
    // Used for writing assertions
    use predicates::prelude::*;
    use std::process::Command; // Run programs

    #[test]
    fn test_main_wrong_path() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("READ ME.md");
        cmd.assert()
            .stdout(predicate::str::contains("annot find path"));
    }

    #[test]
    fn test_main_normal() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md");
        cmd.assert()
            .stdout(predicate::str::contains("1 file checked"));
    }

    #[test]
    fn test_main_normal_wrong_path() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README"); // invalid path
        cmd.assert()
            .code(1)
            .stdout(predicate::str::contains("Cannot find path: README"));
    }

    #[test]
    fn test_main_normal_one_wrong_path() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md");
        cmd.arg("README"); // invalid path
        cmd.assert()
            .code(0)
            .stdout(predicate::str::contains("1 file checked"));
    }

    #[test]
    fn test_main_quiet() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md").arg("-q");
        cmd.assert().stdout(predicate::str::is_empty());
    }

    #[test]
    fn test_main_version() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md").arg("-v");
        cmd.assert()
            .failure()
            .stdout(predicate::str::contains("notox"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_main_json() {
        use serde_json::{json, Value};
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md").arg("Cargo.toml").arg("-j");
        cmd.assert().success();
        let stdout = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
        let idx = stdout.find("README.md").unwrap();
        println!("Index: {}", idx);
        let (json_result, (name1, name2)) = if idx == 10 {
            let json_val = json!([
                {
                    "path": "README.md",
                    "modified": null,
                    "error": null,
                },
                {
                    "path": "Cargo.toml",
                    "modified": null,
                    "error": null,
                }
            ]);
            (json_val, ("README.md", "Cargo.toml"))
        } else {
            let json_val = json!([
                {
                    "path": "Cargo.toml",
                    "modified": null,
                    "error": null,
                },
                {
                    "path": "README.md",
                    "modified": null,
                    "error": null,
                }
            ]);
            (json_val, ("Cargo.toml", "README.md"))
        };
        let str_json = format!(
            r#"[{{"path":"{}","modified":null,"error":null}},{{"path":"{}","modified":null,"error":null}}]
"#,
            name1, name2
        );
        assert_eq!(stdout, str_json);
        let json_deserialized: Value = serde_json::from_str(&stdout).unwrap();
        assert_eq!(json_deserialized, json_result);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_main_json_pretty() {
        use serde_json::{json, Value};
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md").arg("Cargo.toml").arg("-p");
        cmd.assert().success();
        let stdout = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
        let idx = stdout.find("README.md").unwrap();
        println!("Index: {}", idx);
        let (json_result, (name1, name2)) = if idx == 19 {
            let json_val = json!([
                {
                    "path": "README.md",
                    "modified": null,
                    "error": null,
                },
                {
                    "path": "Cargo.toml",
                    "modified": null,
                    "error": null,
                }
            ]);
            (json_val, ("README.md", "Cargo.toml"))
        } else {
            let json_val = json!([
                {
                    "path": "Cargo.toml",
                    "modified": null,
                    "error": null,
                },
                {
                    "path": "README.md",
                    "modified": null,
                    "error": null,
                }
            ]);
            (json_val, ("Cargo.toml", "README.md"))
        };
        let str_json = format!(
            r#"[
  {{
    "path": "{}",
    "modified": null,
    "error": null
  }},
  {{
    "path": "{}",
    "modified": null,
    "error": null
  }}
]
"#,
            name1, name2
        );
        assert_eq!(stdout, str_json);
        let json_deserialized: Value = serde_json::from_str(&stdout).unwrap();
        assert_eq!(json_deserialized, json_result);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_main_serialize() {
        use std::{collections::HashSet, path::PathBuf};

        use notox::{JsonOutput, Notox, NotoxArgs, NotoxOutput, PathChange};

        let notox_args = NotoxArgs {
            dry_run: true,
            output: NotoxOutput::JsonOutput {
                json: JsonOutput::JsonDefault,
                pretty: true,
            },
        };
        let path_to_check: HashSet<PathBuf> =
            HashSet::from(["README.md".into(), "Cargo.toml".into()]);
        let result_lib = Notox::new(notox_args).run(&path_to_check);
        let result_lib: HashSet<PathChange> = result_lib.into_iter().collect();

        let mut cmd = Command::cargo_bin("notox").unwrap();
        cmd.arg("README.md").arg("Cargo.toml").arg("-p");
        cmd.assert().success();
        let stdout = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
        let result_bin: Vec<PathChange> = serde_json::from_str(&stdout).unwrap();
        let result_bin: HashSet<PathChange> = result_bin.into_iter().collect();

        assert_eq!(result_lib, result_bin);
    }
}
