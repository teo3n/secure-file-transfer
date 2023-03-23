use openssl::rsa::{Padding, Rsa};
use openssl::symm::{encrypt, Cipher};
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Start listening for incoming connections
    for stream in listener.incoming() {
        println!("incoming stream from");
        match stream {
            Ok(mut stream) => {
                println!("stream ok");

                // Generate a public-private key pair
                let rsa_key = Rsa::generate(2048).unwrap();
                println!("keys generated");

                // Get the public key as a byte slice
                let public_key = rsa_key.public_key_to_der().unwrap();
                println!("public key with length {}", public_key.len());

                // Send the public key to the client
                stream.write_all(&public_key).unwrap();
                println!("public key sent");

                // Read the encrypted session key from the client
                let mut encrypted_session_key = [0u8; 256];
                stream.read_exact(&mut encrypted_session_key).unwrap();
                println!("session key read");

                // Decrypt the session key with the private key
                let mut decrypted_session_key = [0u8; 32];
                rsa_key
                    .private_decrypt(
                        &encrypted_session_key,
                        &mut decrypted_session_key,
                        Padding::PKCS1,
                    )
                    .unwrap();

                println!("session key decrypted");

                // Use the decrypted session key to encrypt and send a response message to the client
                let message = "This is a secure message.".as_bytes();
                let cipher = Cipher::aes_256_cbc();

                let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
                let encrypted_message = encrypt(cipher, &decrypted_session_key, Some(iv), message).unwrap();
                stream.write_all(&encrypted_message).unwrap();

                println!("message written to remote");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
