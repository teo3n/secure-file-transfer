use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

use crate::commtypes::{FileInfo, FileEntry};

pub fn get_files_in_folder(folder_path: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let folder = fs::read_dir(folder_path).unwrap();

    for file in folder {
        let file_path = file.unwrap().path();
        if file_path.is_file() {
            files.push(file_path);
        } else if file_path.is_dir() {
            let sub_files = get_files_in_folder(file_path.to_str().unwrap());
            files.extend(sub_files);
        }
    }

    files
}

pub fn files_to_serializeable(files: &Vec<PathBuf>) -> serde_json::Value {
    let finfo = FileInfo {
        files: files
            .into_iter()
            .enumerate()
            .map(|(idx, fname)| FileEntry {
                index: idx,
                path: fname.to_string_lossy().to_string(),
            })
            .collect::<Vec<_>>(),
    };

    serde_json::to_value(&finfo).unwrap()
}

pub fn file_to_buffer(file: &str) -> Vec<u8> {
    let mut file_handle = File::open(file).unwrap();
    let mut buffer = Vec::new();

    file_handle.read_to_end(&mut buffer).unwrap();
    buffer
}
