extern crate rand;

pub mod commtypes;
pub mod consts;
pub mod fileio;
pub mod session;

use std::{path::PathBuf, io::Write};

use crate::{
    commtypes::{FileInfo, FileRequest},
    fileio::write_buffer_to_file,
    session::Session, consts::AUTH_SUCCESS,
};


fn main() {
    let session = Session::establish_connection("127.0.0.1:8080");

    // authenticate
    let mut passwd = String::new();

    print!("password: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut passwd).unwrap();
    session.transmit(passwd.as_bytes());

    let auth_status = session.receive_string();
    if auth_status == AUTH_SUCCESS {
        println!("authentication succesfull");
    } else {
        println!("authentication failed!");
        return;
    }

    let recv_files = FileInfo::deserialize(&session.receive_string());

    println!("files available:");
    recv_files.files.clone().into_iter().for_each(|fentry| {
        println!("   {}: {}", fentry.index, fentry.path);
    });

    print!("select file: ");
    std::io::stdout().flush().unwrap();
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
