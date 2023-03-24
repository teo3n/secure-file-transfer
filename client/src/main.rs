use openssl::rsa::{Padding, Rsa};
use openssl::symm::{decrypt, Cipher};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("connected");

    // Receive the server's public key
    // let mut public_key = Vec::new();
    let mut public_key = [0u8; 294];
    stream.read_exact(&mut public_key).unwrap();
    println!("public key read with len {}", public_key.len());

    let rsa_key = Rsa::public_key_from_der(&public_key).unwrap();

    // Generate a session key, encrypt it with the server's public key, and send it to the server
    let mut session_key = [0u8; 256];
    session_key[0] = 5;
    session_key[10] = 15;
    session_key[20] = 25;


    // TODO: generate a random session key

    let mut encrypted_session_key = [0u8; 256];
    rsa_key.public_encrypt(&session_key, &mut encrypted_session_key, Padding::NONE).unwrap();
    stream.write_all(&encrypted_session_key).unwrap();

    let session_key = &session_key[0..32];

    // Receive an encrypted response message from the server, decrypt it with the session key, and print it
    let mut encrypted_message = Vec::new();
    stream.read_to_end(&mut encrypted_message).unwrap();
    println!("message read");

    let cipher = Cipher::aes_256_cbc();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let message = decrypt(cipher, &session_key, Some(iv), &encrypted_message).unwrap();

    println!("{}", String::from_utf8_lossy(&message));
}
