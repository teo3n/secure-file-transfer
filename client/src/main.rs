extern crate rand;

pub mod commtypes;
pub mod consts;
pub mod fileio;
pub mod session;

use std::path::PathBuf;

use crate::{
    commtypes::{FileInfo, FileRequest},
    fileio::write_buffer_to_file,
    session::Session,
};


fn main() {
    let session = Session::establish_connection("127.0.0.1:8080");
    let recv_files = FileInfo::deserialize(&session.receive_string());

    println!("files available:");
    recv_files.files.clone().into_iter().for_each(|fentry| {
        println!("   {}: {}", fentry.index, fentry.path);
    });

    println!("select file: ");
    let mut file_input = String::new();
    std::io::stdin().read_line(&mut file_input).unwrap();
    let file_index: usize = file_input.as_str().trim().parse().unwrap();
    println!(
        "file {} selected, downloading...",
        recv_files.files[file_index].path
    );

    session.transmit(
        serde_json::to_value(FileRequest {
            file: recv_files.files[file_index].path.to_owned(),
        })
        .unwrap()
        .to_string()
        .as_bytes(),
    );

    let file_recv = session.receive_bytes();
    println!("{} bytes received, saving...", file_recv.len());

    let write_path = PathBuf::from(recv_files.files[file_index].path.to_owned());
    let write_file_path = write_path.file_name().unwrap().to_str().unwrap();
    write_buffer_to_file(write_file_path, &file_recv);

    println!("file written to {}", write_file_path);
}
