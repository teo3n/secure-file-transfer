use openssl::pkey::Private;
use openssl::pkey::Public;
use openssl::rsa::{Padding, Rsa};
use openssl::ssl::HandshakeError;
use openssl::ssl::{SslAcceptor, SslStream};
use openssl::ssl::{SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let acceptor = build_ssl_acceptor();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let ssl_stream = acceptor.accept(stream).unwrap();
                handle_client(ssl_stream).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn build_ssl_acceptor() -> SslAcceptor {
    let mut acceptor = SslAcceptor::mozilla_modern(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file("server.key", SslFiletype::PEM)
        .unwrap();
    acceptor.set_certificate_chain_file("server.crt").unwrap();
    acceptor.build()
}

fn handle_client(mut ssl_stream: SslStream<TcpStream>) -> std::io::Result<()> {
    let mut buf = [0; 1024];
    let mut session_key = [0; 32];

    // Step 1: client starts the session by connecting to the server

    println!(
        "New client connected: {:?}",
        ssl_stream.get_ref().peer_addr().unwrap()
    );

    // Step 2: server generates a public-private key pair and responds with its public key
    let rsa = Rsa::generate(2048).unwrap();
    let pub_key = rsa.public_key_to_pem().unwrap();

    ssl_stream.write_all(pub_key.as_slice()).unwrap();

    // Step 3: client generates and encrypts a session-key with the public key and sends to the server
    ssl_stream.read(&mut buf).unwrap();

    let decrypted_len = rsa
        .private_decrypt(&buf, &mut session_key, Padding::PKCS1)
        .unwrap();
    let session_key = &session_key[..decrypted_len];

    // Step 4: server decrypts the session-key using its private key

    println!("Session key received: {:?}", session_key);

    Ok(())
}
