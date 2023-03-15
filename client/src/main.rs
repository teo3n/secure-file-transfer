use openssl::pkey::Public;
use openssl::rsa::{Padding, Rsa};
use openssl::ssl::HandshakeError;
use openssl::ssl::{SslConnector, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut ssl_connector = build_ssl_connector();
    let stream = ssl_connector.connect("127.0.0.1:8888", ).unwrap();

    handle_server(stream).unwrap();
}

fn build_ssl_connector() -> SslConnector {
    let mut connector = SslConnector::builder(SslMethod::tls()).unwrap();
    connector.set_ca_file("server.crt").unwrap();
    connector.build()
}

fn handle_server(mut ssl_stream: openssl::ssl::SslStream<TcpStream>) -> std::io::Result<()> {
    let mut buf = [0; 1024];
    let mut session_key = [0; 32];

    // Step 1: client starts the session by connecting to the server

    println!(
        "Connected to server: {:?}",
        ssl_stream.get_ref().peer_addr().unwrap()
    );

    // Step 2: server generates a public-private key pair and responds with its public key
    ssl_stream.read(&mut buf).unwrap();
    let pub_key = &buf[..];

    // Step 3: client generates and encrypts a session-key with the public key and sends to the server
    let rsa = Rsa::generate(2048).unwrap();
    let encrypted_len = rsa
        .public_encrypt(b"Hello, server!", &mut session_key, Padding::PKCS1)
        .unwrap();
    let session_key = &session_key[..encrypted_len];

    ssl_stream.write_all(session_key).unwrap();

    // Step 4: server decrypts the session-key using its private key

    Ok(())
}
