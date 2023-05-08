extern crate rand;

use std::{cell::RefCell, net::{TcpListener, TcpStream}};

use crate::{
    commtypes::FileRequest,
    consts::{FILE_DB_PATH, AUTH_PWD, AUTH_FAILURE, AUTH_SUCCESS},
    fileio::{file_to_buffer, files_to_serializeable, get_files_in_folder},
    session::Session,
};

pub mod commtypes;
pub mod consts;
pub mod fileio;
pub mod session;


fn handler(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("incoming stream from {:?}", stream.local_addr()?);

    let session = Session::establish_connection(RefCell::new(stream))?;

    // authenticate
    let auth = session.receive_string()?;
    if auth.trim() != AUTH_PWD {
        session.transmit(AUTH_FAILURE.as_bytes())?;
        return Err("authentication failed".into());
    } else {
        session.transmit(AUTH_SUCCESS.as_bytes())?;
    }

    let files = get_files_in_folder(FILE_DB_PATH);
    session.transmit(files_to_serializeable(&files)?.to_string().as_bytes())?;

    let file_rqs = session.receive_string()?;

    let file_request = FileRequest::deserialize(&file_rqs)?;
    println!("file {} requested, transmitting", file_request.file);

    // verify the file is valid
    if !files.iter().map(|f| f.to_string_lossy()).collect::<String>().contains(&file_request.file) {
        return Err("invalid file path in request!".into());
    }

    let buffer = file_to_buffer(&file_request.file)?;
    session.transmit(&buffer)?;
    println!("{} bytes sent", buffer.len());

    Ok(())
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handler(stream) {
                    println!("{}", err);
                    println!("continuing...");
                }
            }

            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
