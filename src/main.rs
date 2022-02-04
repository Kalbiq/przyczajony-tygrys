use config::Config;
use encoder::{decode, encode};
use image::{io::Reader as ImageReader, DynamicImage, EncodableLayout, GenericImageView};
use std::{
    env,
    fs::{self, File},
    io::Write,
};

use crate::helper::print_help;

mod config;
mod encoder;
mod helper;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::from_args(&args) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Config validation failed!");
            eprintln!("Error: {}", e);
            std::process::exit(0);
        }
    };

    if config.help {
        print_help();
        std::process::exit(0);
    }

    let mut img = load_image(&config.org_image_path.unwrap());

    if config.decode {
        let file_bytes = decode(&img);

        let mut file = File::create(config.save_path.unwrap()).unwrap();

        file.write(file_bytes.as_bytes()).unwrap();
    } else {
        let file_bytes = fs::read_to_string(&config.file_to_hide_path.unwrap())
            .unwrap()
            .into_bytes();

        if file_bytes.len() > u32::MAX as usize {
            eprintln!("The file is too big. Allowed size is under 4.2 GB.");
            std::process::exit(1);
        }

        // file_bytes.len() is most likely < than max u32 number
        // it would be around 4 GB of data which is plenty for this application
        // thus we add 4 to encode the length of the hidden file
        if img.height() * img.width() < file_bytes.len() as u32 + 4 {
            eprintln!("The file is too big for this image!");
            eprintln!(
                "File size: {} | Img capacity: {} (bytes)",
                file_bytes.len(),
                img.height() * img.width()
            );
            std::process::exit(1);
        }

        encode(&file_bytes, &mut img);

        img.save(&config.save_path.unwrap()).unwrap();
    }

    std::process::exit(0);
}

fn load_image(path: &String) -> DynamicImage {
    let image = ImageReader::open(path).unwrap().decode().unwrap();

    image
}
