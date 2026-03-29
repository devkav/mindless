use std::{fs, path::{PathBuf}};
use zlib_rs::{DeflateConfig, InflateConfig, compress_bound, compress_slice, decompress_slice};

use crate::util::constants::MINDLESS_DIR_NAME;

pub fn decompress_blob(blob_path: String) -> String {
    // TODO: Prob needs a rewrite/TODO to fix error handling

    let config = InflateConfig::default();

    let bytes: Vec<u8> = fs::read(blob_path).expect("Something went wrong while opening blob");
    let input: &[u8] = &bytes;
    let mut header_buf = vec![0u8; 32];

    let (decompressed_header, _) = decompress_slice(&mut header_buf, input, config);
    let mut header_bytes: Vec<u8> = Vec::new();
    let mut i = 0;
    let mut null_char_found: bool = false;

    while i < decompressed_header.len() && !null_char_found {
        let byte: u8 = decompressed_header[i];

        if byte == 0 {
            null_char_found = true;
            continue;
        }

        header_bytes.push(byte);
        i += 1;
    }

    let header_str = str::from_utf8(&header_bytes).expect("Something went wrong while parsing decompressed string");
    let header_tokens: Vec<&str> = header_str.split(" ").collect();

    assert!(header_tokens.len() == 2);
    let blob_size = header_tokens[1].parse::<usize>();
    assert!(blob_size.is_ok());

    let total_blob_size = blob_size.unwrap() + decompressed_header.len();

    let mut buf = vec![0u8; total_blob_size];
    let (decompressed_blob, _) = decompress_slice(&mut buf, input, config);
    let output_result = str::from_utf8(decompressed_blob)
        .expect("Something went wrong while parsing blob")
        .to_string();

    return output_result;
}

pub fn compress_file(file_path: &PathBuf, mindless_root: &PathBuf) {
    let input = fs::read(file_path);

    match input {
        Ok(file_bytes) => {
            let mut compressed_buf = vec![0u8; compress_bound(file_bytes.len())];
            let header = format!("blob {}\0", file_bytes.len());
            let mut blob = header.into_bytes();
            blob.extend(&file_bytes);

            let (compressed, _) = compress_slice(&mut compressed_buf, &blob, DeflateConfig::default());

            // TODO: Add error handling
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            // TODO: Better organization and naming for the files
            let output_path = mindless_root.join(MINDLESS_DIR_NAME).join(file_name);

            fs::write(output_path, &compressed).expect("error");
        },
        Err(_e) => {
            // TODO
        }
    }
}
