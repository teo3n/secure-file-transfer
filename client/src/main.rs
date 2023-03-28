extern crate rand;

pub mod session;
pub mod consts;

use crate::session::Session;


fn main() {
    let session = Session::establish_connection("127.0.0.1:8080");
    println!("{:?}", session.receive());
    session.transmit("dimwits gonna bitchnigga".as_bytes());
}
