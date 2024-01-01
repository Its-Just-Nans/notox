use std::{
    ffi::{OsStr, OsString},
    fs::DirEntry,
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
struct JsonFields {
    json: bool,
    json_pretty: bool,
    json_error: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]

struct OptionnalFields {
    dry_run: bool,
    verbose: bool,
    json: JsonFields,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
struct CustomSingleResult {
    path: PathBuf,
    modified: Option<PathBuf>,
    error: Option<String>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
struct CustomResult {
    res: Vec<CustomSingleResult>,
}

struct ResultArgsParse {
    fields: OptionnalFields,
    paths: Vec<PathBuf>,
}

fn clean_name(path: &OsStr, _options: &OptionnalFields) -> OsString {
    // for each byte of the path if it's not ascii, replace it with _
    let mut new_name = String::new();
    let mut vec_grapheme = Vec::with_capacity(4);
    for byte in path.as_bytes().to_owned() {
        if vec_grapheme.len() == 0 && byte < 128 {
            match byte {
                0..=44 => {
                    new_name.push('_');
                }
                46 => {
                    new_name.push('.');
                }
                47 => {
                    new_name.push('_');
                }
                58..=64 => {
                    new_name.push('_');
                }
                91..=96 => {
                    new_name.push('_');
                }
                123..=127 => {
                    new_name.push('_');
                }
                _ => new_name.push(byte as char),
            }
        } else {
            vec_grapheme.push(byte);
            let first_byte = vec_grapheme[0];
            if first_byte >= 192 && first_byte < 240 && vec_grapheme.len() == 2 {
                // two bytes grapheme
                let vec_to_string =
                    String::from_utf8(vec_grapheme.clone()).unwrap_or("".to_string());
                match vec_to_string.as_str() {
                    "À" | "Á" | "Â" | "Ã" | "Ä" | "Å" => {
                        new_name.push('A');
                    }
                    "Æ" => {
                        new_name.push('A');
                        new_name.push('E');
                    }
                    "Ç" => {
                        new_name.push('C');
                    }
                    "É" | "È" | "Ê" | "Ë" => {
                        new_name.push('E');
                    }
                    "Ì" | "Í" | "Î" | "Ï" => {
                        new_name.push('I');
                    }
                    "Ð" => {
                        new_name.push('D');
                    }
                    "Ñ" => {
                        new_name.push('N');
                    }
                    "Ò" | "Ó" | "Ô" | "Õ" | "Ö" => {
                        new_name.push('O');
                    }
                    "×" => {
                        new_name.push('x');
                    }
                    "Ù" | "Ú" | "Û" | "Ü" => {
                        new_name.push('U');
                    }
                    "Ý" => {
                        new_name.push('Y');
                    }
                    "ß" => {
                        new_name.push('b');
                    }
                    "à" | "á" | "â" | "ä" | "ã" | "å" => {
                        new_name.push('a');
                    }
                    "æ" => {
                        new_name.push('a');
                        new_name.push('e');
                    }
                    "ç" => {
                        new_name.push('c');
                    }
                    "é" | "è" | "ê" | "ë" => {
                        new_name.push('e');
                    }
                    "ì" | "í" | "î" | "ï" => {
                        new_name.push('i');
                    }
                    "ñ" => {
                        new_name.push('n');
                    }
                    "ð" | "ò" | "ó" | "ô" | "õ" | "ö" => {
                        new_name.push('o');
                    }
                    "ù" | "ú" | "û" | "ü" => {
                        new_name.push('u');
                    }
                    "ý" | "ÿ" => {
                        new_name.push('y');
                    }
                    _ => {
                        new_name.push('_');
                    }
                }
                vec_grapheme.clear();
            } else if first_byte >= 224 && first_byte < 240 && vec_grapheme.len() == 3 {
                // three bytes grapheme
                new_name.push('_');
                vec_grapheme.clear();
            } else if first_byte >= 240 && vec_grapheme.len() == 4 {
                // four bytes grapheme
                new_name.push('_');
                vec_grapheme.clear();
            }
        }
    }
    return OsString::from(new_name);
}

fn clean_path(file_path: &PathBuf, options: &OptionnalFields) -> CustomSingleResult {
    let file_name = file_path.file_name();
    if file_name.is_none() {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: None,
            error: None,
        };
    }
    let file_name = file_name.unwrap();
    let cleaned_name = clean_name(file_name, &options);
    if &cleaned_name == file_name {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: None,
            error: None,
        };
    }
    let cleaned_path = file_path.with_file_name(cleaned_name);
    if options.dry_run {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: Some(cleaned_path),
            error: Some("dry-run".to_string()),
        };
    }
    let is_renamed = std::fs::rename(file_path, &cleaned_path);
    if let Err(rename_error) = is_renamed {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: Some(cleaned_path),
            error: Some(rename_error.to_string()),
        };
    }
    return CustomSingleResult {
        path: file_path.to_path_buf(),
        modified: Some(cleaned_path),
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

fn is_directory_entry(entry: &DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        metadata.is_dir()
    } else {
        false
    }
}

fn clean_directory(dir_path: &PathBuf, options: &OptionnalFields) -> CustomResult {
    let mut dir_path = dir_path.clone();
    let mut res: CustomResult = CustomResult { res: Vec::new() };
    let res_dir = clean_path(&dir_path, &options);
    if res_dir.modified.is_some() && !options.dry_run && res_dir.error.is_none() {
        if let Some(ref modified) = res_dir.modified {
            dir_path = modified.clone();
        }
    }
    res.res.push(res_dir);
    if let Ok(entries) = std::fs::read_dir(&dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if is_directory_entry(&entry) {
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
                    error: Some("Entry error".to_string()),
                });
            }
        }
    } else {
        res.res.push(CustomSingleResult {
            path: dir_path,
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

fn parse_args() -> ResultArgsParse {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("You need to provide at least one path");
        std::process::exit(1);
    }

    let mut dry_run = true;
    let mut verbose = true;
    let mut json = false;
    let mut json_pretty = false;
    let mut json_error = false;
    let mut path_to_check: Vec<PathBuf> = Vec::new();
    for one_arg in &args[1..] {
        if one_arg == "-d" || one_arg == "--do" {
            dry_run = false;
        } else if one_arg == "-v" || one_arg == "--version" {
            show_version();
            std::process::exit(0);
        } else if one_arg == "-p" || one_arg == "--json-pretty" {
            json = true;
            json_pretty = true;
            verbose = false;
        } else if one_arg == "-e" || one_arg == "--json-error" {
            json = true;
            json_error = true;
            verbose = false;
        } else if one_arg == "-j" || one_arg == "--json" {
            json = true;
            verbose = false;
        } else if one_arg == "-q" || one_arg == "--quiet" {
            verbose = false;
        } else if one_arg == "*" {
            let paths = get_path_of_dir(".");
            path_to_check.extend(paths);
        } else {
            if std::fs::metadata(one_arg).is_ok() {
                path_to_check.push(PathBuf::from(one_arg))
            } else {
                if verbose {
                    println!("Cannot find path: {}", one_arg);
                }
            }
        }
    }
    if path_to_check.len() == 0 {
        if verbose {
            println!("You need to provide at least one path !");
        } else if json {
            println!(r#"{{"error": "You need to provide at least one path"}}"#);
        }
        std::process::exit(1);
    }
    return ResultArgsParse {
        fields: OptionnalFields {
            dry_run,
            verbose,
            json: JsonFields {
                json,
                json_pretty,
                json_error,
            },
        },
        paths: path_to_check,
    };
}

fn print_output(options: &OptionnalFields, final_res: CustomResult) {
    if options.verbose {
        let len = final_res.res.len();
        for one_res in final_res.res {
            if one_res.modified.is_some() && one_res.error.is_some() {
                println!(
                    "{:?} -> {:?} : {}",
                    one_res.path,
                    one_res.modified.unwrap(),
                    one_res.error.unwrap()
                );
            } else if one_res.error.is_some() {
                println!("{:?} : {}", one_res.path, one_res.error.unwrap());
            } else if one_res.modified.is_some() {
                println!("{:?} -> {:?}", one_res.path, one_res.modified.unwrap());
            }
        }
        println!("{} files checked", len);
    } else if options.json.json {
        #[cfg(feature = "serde")]
        {
            let vec_to_json = if options.json.json_error {
                let mut vec_to_json: Vec<CustomSingleResult> = Vec::new();
                for one_res in final_res.res {
                    if one_res.error.is_some() {
                        vec_to_json.push(one_res);
                    }
                }
                vec_to_json
            } else {
                final_res.res
            };
            let json_string = if options.json.json_pretty {
                serde_json::to_string_pretty(&vec_to_json)
            } else {
                serde_json::to_string(&vec_to_json)
            };
            if let Ok(stringed) = json_string {
                println!("{}", stringed);
            } else {
                println!(r#"{{"error": "Cannot serialize result"}}"#);
                std::process::exit(1);
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let ResultArgsParse {
        fields: options,
        paths: path_to_check,
    } = parse_args();
    if options.verbose {
        println!("Running with options: {:?}", &options);
    }
    let mut final_res = CustomResult { res: Vec::new() };
    for one_path in path_to_check {
        let one_res = clean(one_path, &options);
        final_res.res.extend(one_res.res);
    }
    print_output(&options, final_res);
    Ok(())
}
