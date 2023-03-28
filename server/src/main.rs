use std::{net::TcpListener, cell::RefCell};

use crate::session::Session;

pub mod session;
pub mod consts;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Start listening for incoming connections
    for stream in listener.incoming() {
        println!("incoming stream from");
        match stream {
            Ok(stream) => {
                println!("stream ok");

                let session = Session::establish_connection(RefCell::new(stream));
                session.transmit("this is a message".as_bytes());
                println!("{:?}", session.receive());

            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
