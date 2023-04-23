use std::{fs::File, io::Write};

pub fn write_buffer_to_file(file_path: &str, buffer: &[u8]) {
    let mut file_handle = File::create(file_path).unwrap();
    file_handle.write_all(buffer).unwrap();
}
