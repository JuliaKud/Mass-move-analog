use std::path::{PathBuf};
use glob::glob;
use regex::Regex;
use std::fs;

fn get_files_by_pattern(pattern: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(entries) = glob(pattern) {
        for entry in entries {
            if let Ok(path) = entry {
                paths.push(path);
            }
        }
    }

    paths
}

fn is_file_name_invalid(path: &str) -> bool {
    let components: Vec<&str> = path.split('/').collect();

    if let Some(file_name) = components.last() {
        return !file_name.contains('*');
    }

    false
}

fn get_new_file_path(path: &str, source_pattern: &str, target_pattern: &str) -> PathBuf {
    let re_source = Regex::new(r"\*").unwrap();
    let source_pattern_re = re_source.replace_all(source_pattern, |caps: &regex::Captures| {
        format!("(.{})", &caps[0])
    });

    let source_regex = Regex::new(&source_pattern_re).unwrap();

    let captures = source_regex.captures(path).unwrap();

    let re_target = Regex::new(r"#(\d+)").unwrap();

    let result = re_target.replace_all(target_pattern, |caps: &regex::Captures<'_>| {
        if let Some(num_match) = caps.get(1) {
            if let Ok(num) = num_match.as_str().parse::<usize>() {
                if let Some(replacement) = captures.get(num) {
                    return replacement.as_str().to_string();
                }
            }
        }
        caps[0].to_string()
    }).into_owned();

    PathBuf::from(result)
}

fn move_file(source_path: PathBuf, target_path: PathBuf) {
    fs::rename(&source_path, &target_path).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: mmv <source_pattern> <target_pattern>");
        return;
    }

    let source_pattern = args[1].as_str();
    let target_pattern = args[2].as_str();

    if is_file_name_invalid(source_pattern) {
        println!("The * character can only be contained in the file name");
    }

    let source_files_paths = get_files_by_pattern(source_pattern);
    for source_path in source_files_paths {
        let target_path = get_new_file_path(source_path.to_str().unwrap(), source_pattern, target_pattern);
        println!("{} -> {}", source_path.to_str().unwrap(), target_path.to_str().unwrap());
        move_file(source_path.clone(), target_path.clone());
    }
}
