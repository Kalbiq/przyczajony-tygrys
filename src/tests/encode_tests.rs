use std::fs;

use image::GenericImageView;
use serial_test::serial;

use crate::config::Config;
use crate::encoder::encode;
use crate::load_image;

use super::{CLEAN_IMG, ENCODED_IMG, FILE_TO_HIDE};

fn conf_setup() -> Config {
    let args = vec!["-i", CLEAN_IMG, "-f", FILE_TO_HIDE, "-o", ENCODED_IMG]
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

/// encode test without any checks
#[test]
#[serial]
fn pure_encode_test() {
    let conf = conf_setup();

    let mut img = load_image(&conf.org_image_path.unwrap());

    let file_bytes = fs::read_to_string(&conf.file_to_hide_path.unwrap())
        .unwrap()
        .into_bytes();

    encode(&file_bytes, &mut img);

    img.save(&conf.save_path.as_ref().unwrap()).unwrap();

    drop(img);

    let img = load_image(&conf.save_path.as_ref().unwrap());

    for i in 0..file_bytes.len() {
        let x = i as u32 % img.width();
        let y = i as u32 / img.width();

        let px = img.get_pixel(x, y);

        assert_eq!(file_bytes[i], px[3], "pixel: {} {}", x, y);
    }

    match fs::remove_file(&conf.save_path.unwrap()) {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to remove the encoded file: {}", e)
        }
    }
}

#[test]
#[serial]
fn normal_encode() {
    let conf = conf_setup();

    crate::run(conf);

    let img = load_image(&ENCODED_IMG.to_string());
    let file_bytes = fs::read_to_string(FILE_TO_HIDE).unwrap().into_bytes();

    for i in 0..file_bytes.len() {
        let x = i as u32 % img.width();
        let y = i as u32 / img.width();

        let px = img.get_pixel(x, y);

        assert_eq!(file_bytes[i], px[3], "pixel: {} {}", x, y);
    }

    match fs::remove_file(ENCODED_IMG) {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to remove the encoded file: {}", e)
        }
    }
}
