use std::{env, fs, path::Path};
use zlib_rs::{InflateConfig, decompress_slice};

pub fn decompress_blob(blob_path: String) -> String {
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

pub fn compress_blob(path_str: String) {
    let working_directory = env::current_dir().expect("Couldn't find working directory");
    let target_path = Path::new(&path_str);
    let absolute_target_path = working_directory.join(target_path);

    if absolute_target_path.is_dir() {
        println!("Adding all files in directory: {}", absolute_target_path.display());

        let children = absolute_target_path.read_dir().expect("Error searching directory");

        for path in children {
            let Ok(valid_path) = path else { continue };
            let current_path = valid_path.path();
            
            if current_path.is_dir() {
                compress_blob(current_path.display().to_string());
            }
        }
    } else {
        println!("Adding file: {}", absolute_target_path.display());
    }
}
