extern crate rand;

use std::{net::TcpListener, cell::RefCell};

use crate::session::Session;

pub mod session;
pub mod consts;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("incoming stream from {:?}", stream.local_addr().unwrap());
                println!("\nstream ok");

                let session = Session::establish_connection(RefCell::new(stream));
                session.transmit("message from the server".as_bytes());
                println!("{:?}", session.receive());

            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
