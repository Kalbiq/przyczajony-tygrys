use super::{CLEAN_IMG, FILE_TO_HIDE};
use crate::config::Config;

const FORMAT_ERR: &str = "Wrong image format!";
const MISSING_FILES: &str = "Missing file paths!";
const FILE_DOESNT_EXIST: &str = "File doesn't exist!";

fn validate(conf: Result<Config, String>, message: &str, incorrect: bool) {
    match conf {
        Ok(_c) => {
            if incorrect {
                panic!("Config passed but shouldn't!");
            }
        }
        Err(s) => {
            assert_eq!(message, s, "Failed to load config: {}", s);
        }
    }
}

#[test]
fn correct_file_format_encode() {
    let args = vec!["-i", CLEAN_IMG, "-f", FILE_TO_HIDE, "-o", "encoded.png"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let conf = Config::from_args(&args);

    validate(conf, FORMAT_ERR, false);
}

#[test]
fn correct_file_format_decode() {
    let args = vec!["--decode", "-i", CLEAN_IMG, "-o", "output.txt"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let conf = Config::from_args(&args);

    validate(conf, FORMAT_ERR, false);
}

#[test]
fn incorrect_file_format_encode() {
    let args = vec![
        "-i",
        CLEAN_IMG.replace(".png", ".gnp").as_str(),
        "-f",
        FILE_TO_HIDE,
        "-o",
        "encoded.png",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let conf = Config::from_args(&args);

    validate(conf, FORMAT_ERR, true);
}

#[test]
fn incorrect_file_format_decode() {
    let args = vec![
        "--decode",
        "-i",
        CLEAN_IMG.replace(".png", ".gnp").as_str(),
        "-o",
        "output.txt",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let conf = Config::from_args(&args);

    validate(conf, FORMAT_ERR, true);
}

#[test]
fn correct_file_count_encode() {
    let args = vec!["-i", CLEAN_IMG, "-f", FILE_TO_HIDE, "-o", "encoded.png"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let conf = Config::from_args(&args);

    validate(conf, MISSING_FILES, false);
}

#[test]
fn correct_file_count_decode() {
    let args = vec!["--decode", "-i", CLEAN_IMG, "-o", "encoded.png"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let conf = Config::from_args(&args);

    validate(conf, MISSING_FILES, false);
}

#[test]
fn incorrect_file_count_encode() {
    let args = vec!["-i", CLEAN_IMG, "-o", "encoded.png"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let conf = Config::from_args(&args);

    validate(conf, MISSING_FILES, true);
}

#[test]
fn incorrect_file_count_decode() {
    let args = vec!["--decode", "-o", "encoded.png"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let conf = Config::from_args(&args);

    validate(conf, MISSING_FILES, true);
}
