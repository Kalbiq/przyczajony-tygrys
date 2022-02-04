use image::{DynamicImage, GenericImage, GenericImageView};

pub fn encode(file_bytes: &Vec<u8>, img: &mut DynamicImage) {
    for i in 0..((img.width() * img.height()) - 4) as usize {
        let xy = (i as u32 % img.width(), i as u32 / img.width());

        // image::image::GenericImageView::get_pixel_mut is not yet implemented?
        // it panics with "not implemented"
        // so we need to use get_pixel and then put_pixel

        let mut px = img.get_pixel(xy.0, xy.1);
        px[3] = file_bytes[i % file_bytes.len()];

        img.put_pixel(xy.0, xy.1, px);
    }

    println!("Encoded: {} bytes", file_bytes.len());

    encode_file_len(img, file_bytes.len() as u32);
}

pub fn decode(img: &DynamicImage) -> Vec<u8> {
    let file_len = decode_file_len(img);

    let mut file_bytes: Vec<u8> = Vec::new();

    for i in 0..file_len {
        let xy = (i as u32 % img.width(), i as u32 / img.width());

        let px = img.get_pixel(xy.0, xy.1);

        file_bytes.push(px[3]);
    }

    println!("Decoded: {} bytes", file_len);

    file_bytes
}

/// Writes files length as u32 at the end of the img
fn encode_file_len(img: &mut DynamicImage, file_len: u32) {
    let len = file_len.to_be_bytes();

    for i in 0..4 {
        let x = i as u32 + img.width() - 4;

        let mut px = img.get_pixel(x, img.height() - 1);

        px[3] = len[i];

        img.put_pixel(x, img.height() - 1, px);
    }
}

fn decode_file_len(img: &DynamicImage) -> u32 {
    let mut len: u32 = 0;

    for i in 0..4 {
        let x = i as u32 + img.width() - 4;

        let px = img.get_pixel(x, img.height() - 1);

        // bit shift first because we don't want to do it at the end
        // when i = 0 => len = 0 so "len << 8" doesn't change anything
        len = len << 8;
        len = len + px[3] as u32;
    }

    len
}
