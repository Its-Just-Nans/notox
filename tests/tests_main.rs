#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
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
    fn test_main_json() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md").arg("Cargo.toml").arg("-j");
        cmd.assert().success();
        let stdout = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
        let idx = stdout.find("README.md").unwrap();
        println!("Index: {}", idx);
        let (name1, name2) = if idx == 10 {
            ("README.md", "Cargo.toml")
        } else {
            ("Cargo.toml", "README.md")
        };
        let result = format!(
            r#"[{{"path":"{}","modified":null,"error":null}},{{"path":"{}","modified":null,"error":null}}]
"#,
            name1, name2
        );
        assert_eq!(stdout, result);
    }

    #[test]
    fn test_main_json_pretty() {
        let mut cmd = Command::cargo_bin("notox").unwrap();

        cmd.arg("README.md").arg("Cargo.toml").arg("-p");
        cmd.assert().success();
        let stdout = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
        let idx = stdout.find("README.md").unwrap();
        println!("Index: {}", idx);
        let (name1, name2) = if idx == 19 {
            ("README.md", "Cargo.toml")
        } else {
            ("Cargo.toml", "README.md")
        };
        let result = format!(
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
        assert_eq!(stdout, result);
    }
}
