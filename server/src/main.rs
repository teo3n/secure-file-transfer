extern crate rand;

use std::{net::TcpListener, cell::RefCell};

use crate::{session::Session, fileio::{get_files_in_folder, files_to_serializeable, file_to_buffer}, consts::FILE_DB_PATH, commtypes::FileRequest};

pub mod session;
pub mod consts;
pub mod fileio;
pub mod commtypes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("incoming stream from {:?}", stream.local_addr().unwrap());
                println!("\nstream ok");

                let session = Session::establish_connection(RefCell::new(stream));

                let files = get_files_in_folder(FILE_DB_PATH);
                session.transmit(files_to_serializeable(&files).to_string().as_bytes());

                let file_request = FileRequest::deserialize(&session.receive_string());
                println!("file {} requested, transmitting", file_request.file);

                let buffer = file_to_buffer(&file_request.file);
                session.transmit(&buffer);
                println!("{} bytes sent", buffer.len());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
