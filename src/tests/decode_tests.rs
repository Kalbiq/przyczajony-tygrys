use std::fs;

use crate::{config::Config, encoder::decode, load_image};

use super::{DECODED_FILE, FILE_TO_HIDE, IMG_TO_DECODE};

fn conf_setup() -> Config {
    let args = vec!["--decode", "-i", IMG_TO_DECODE, "-o", DECODED_FILE]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let conf = match Config::from_args(&args) {
        Ok(c) => c,
        Err(s) => {
            panic!("Failed to load config: {}", s)
        }
    };

    conf
}

#[test]
fn pure_decode_test() {
    let conf = conf_setup();

    let img = load_image(&conf.org_image_path.unwrap());

    let file_bytes = fs::read_to_string(FILE_TO_HIDE).unwrap().into_bytes();

    let decoded_bytes = decode(&img);

    if decoded_bytes.len() != file_bytes.len() {
        panic!(
            "Wrong length | org = {}, decoded = {}",
            file_bytes.len(),
            decoded_bytes.len()
        );
    }

    assert_eq!(file_bytes, decoded_bytes);
}

#[test]
fn normal_decode() {
    let conf = conf_setup();

    crate::run(conf);

    let file_bytes = fs::read_to_string(FILE_TO_HIDE).unwrap().into_bytes();
    let decoded_bytes = fs::read_to_string(DECODED_FILE).unwrap().into_bytes();

    if decoded_bytes.len() != file_bytes.len() {
        panic!(
            "Wrong length | org = {}, decoded = {}",
            file_bytes.len(),
            decoded_bytes.len()
        );
    }

    assert_eq!(file_bytes, decoded_bytes);

    match fs::remove_file(DECODED_FILE) {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to remove the decoded file: {}", e)
        }
    }
}
