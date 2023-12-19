use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};
#[derive(Debug)]
struct OptionnalFields {
    dry_run: bool,
    verbose: bool,
}

#[derive(Debug)]
struct CustomSingleResult {
    path: PathBuf,
    modified: Option<PathBuf>,
    error: Option<String>,
}

#[derive(Debug)]
struct CustomResult {
    res: Vec<CustomSingleResult>,
}

fn clean_name(path: &OsStr, _options: &OptionnalFields) -> Result<OsString, String> {
    if let Some(e) = path.to_str() {
        let cleaned_name = e.replace(" ", "_");
        return Ok(OsString::from(cleaned_name));
    }
    return Err("Cannot clean name".to_string());
}

fn clean_path(file_path: &PathBuf, options: &OptionnalFields) -> CustomSingleResult {
    let file_name = file_path.file_name();
    if file_name.is_none() {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: None,
            error: Some("No file name".to_string()),
        };
    }
    let file_name = file_name.unwrap();
    let cleaned_name = clean_name(file_name, &options);
    if cleaned_name.is_err() {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: None,
            error: Some(cleaned_name.err().unwrap()),
        };
    }
    let cleaned_name = cleaned_name.unwrap();
    let cleaned_path = file_path.with_file_name(cleaned_name.clone());
    if options.dry_run {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: if &cleaned_name != &file_name {
                Some(cleaned_path)
            } else {
                None
            },
            error: Some("--dry-run enabled".to_string()),
        };
    }
    let is_renamed = std::fs::rename(file_path, &cleaned_path);
    if let Err(is_renamed) = is_renamed {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: None,
            error: Some(is_renamed.to_string()),
        };
    }
    return CustomSingleResult {
        path: file_path.to_path_buf(),
        modified: if &cleaned_name != &file_name {
            Some(cleaned_path)
        } else {
            None
        },
        error: None,
    };
}

fn is_directory(path: &PathBuf) -> bool {
    if let Ok(metadata) = std::fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}

fn clean_directory(dir_path: &PathBuf, options: &OptionnalFields) -> CustomResult {
    let mut res: CustomResult = CustomResult { res: Vec::new() };
    let res_dir = clean_path(dir_path, &options);
    res.res.push(res_dir);
    if let Ok(entries) = std::fs::read_dir(&dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if is_directory(&file_path) {
                    let e = clean_directory(&file_path, &options);
                    res.res.extend(e.res);
                } else {
                    let e = clean_path(&file_path, &options);
                    res.res.push(e);
                }
            } else {
                res.res.push(CustomSingleResult {
                    path: dir_path.clone(),
                    modified: None,
                    error: Some("Error while reading directory".to_string()),
                });
            }
        }
    } else {
        res.res.push(CustomSingleResult {
            path: PathBuf::from(dir_path),
            modified: None,
            error: Some("Error while reading directory".to_string()),
        });
    }
    return res;
}

fn clean(path: PathBuf, options: &OptionnalFields) -> CustomResult {
    if is_directory(&path) {
        return clean_directory(&path, &options);
    } else {
        let res = clean_path(&path, &options);
        return CustomResult {
            res: Vec::from([res]),
        };
    }
}

fn get_path_of_dir(dir_path: &str) -> Vec<PathBuf> {
    let mut path_to_check: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                path_to_check.push(file_path)
            }
        }
    }
    return path_to_check;
}

fn show_version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    println!("notox {} by {}", &VERSION, &AUTHORS)
}

fn main() -> Result<(), std::io::Error> {
    // let args: Vec<String> = std::env::args().collect();
    let args: Vec<String> = Vec::from(["notox".to_string(), "-n".to_string(), "*".to_string()]);
    if args.len() == 1 {
        println!("You need to provide at least one path");
        std::process::exit(1);
    }

    let mut dry_run = false;
    let mut verbose = true;
    let mut path_to_check: Vec<PathBuf> = Vec::new();
    for one_arg in &args[1..] {
        if one_arg == "-n" {
            dry_run = true;
        } else if one_arg == "--dry-run" {
            dry_run = true;
        } else if one_arg == "-v" {
            show_version()
        } else if one_arg == "--version" {
            show_version()
        } else if one_arg == "-q" {
            verbose = false;
        } else if one_arg == "*" {
            let paths = get_path_of_dir(".");
            path_to_check.extend(paths);
        } else {
            path_to_check.push(PathBuf::from(one_arg))
        }
    }
    if path_to_check.len() == 0 {
        if verbose {
            println!("You need to provide at least one path !");
        }
        std::process::exit(1);
    }
    let options = &OptionnalFields {
        dry_run: dry_run,
        verbose: verbose,
    };
    if verbose {
        println!("Running with options: {:?}", &options);
    }
    let mut final_res = CustomResult { res: Vec::new() };
    for one_path in path_to_check {
        let one_res = clean(one_path, &options);
        final_res.res.extend(one_res.res);
    }
    if options.verbose {
        for one_res in final_res.res {
            if one_res.modified.is_some() {
                println!("{:?}", one_res);
            }
        }
    }
    Ok(())
}
