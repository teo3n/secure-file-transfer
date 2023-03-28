use openssl::rsa::{Padding, Rsa};
use openssl::symm::{decrypt, encrypt, Cipher};
use rand::RngCore;
use std::cell::RefCell;
use std::io::{Read, Write};
use rand::rngs::OsRng;
use std::net::TcpStream;

use crate::consts::IV;

pub struct Session {
    pub session_key: Vec<u8>,
    pub stream: RefCell<TcpStream>,
    pub cipher: Cipher,
}

impl Session {
    pub fn decrypt(&self, encrypted_message: &Vec<u8>) -> String {
        let message = decrypt(self.cipher, &self.session_key, Some(IV), encrypted_message).unwrap();
        String::from_utf8_lossy(&message).to_string()
    }

    pub fn receive_message(&self, len: usize) -> Vec<u8> {
        // Receive an encrypted response message from the server, decrypt it with the session key
        let mut encrypted_message = vec![0; len];
        self.stream.borrow_mut().read_exact(&mut encrypted_message).unwrap();

        encrypted_message
    }

    pub fn transmit(&self, bytes: &[u8]) {
        let encrypted_message = encrypt(self.cipher, &self.session_key, Some(IV), bytes).unwrap();

        // write the length buffer
        let len_buf = (encrypted_message.len() as u32).to_le_bytes();
        self.stream.borrow_mut().write_all(&len_buf).unwrap();

        // write the actual data buffer
        self.stream.borrow_mut().write_all(&encrypted_message).unwrap();
        self.stream.borrow_mut().flush().unwrap();
    }

    pub fn receive(&self) -> String {
        let lenbuf = self.receive_message(4);
        let recv_datalen = u32::from_le_bytes(lenbuf.try_into().unwrap());

        self.decrypt(&self.receive_message(recv_datalen as usize))
    }

    fn gen_session_key() -> [u8; 256] {
        let mut rng = OsRng;
        let mut key = [0u8; 256];
        rng.fill_bytes(&mut key[..32]);
        key
    }

    pub fn establish_connection(target: &str) -> Self {
        let mut stream = TcpStream::connect(target).unwrap();
        println!("connected");
    
        // Receive the server's public key
        // let mut public_key = Vec::new();
        let mut public_key = [0u8; 294];
        stream.read_exact(&mut public_key).unwrap();
        println!("public key read with len {}", public_key.len());
    
        let rsa_key = Rsa::public_key_from_der(&public_key).unwrap();
    
        // Generate a session key, encrypt it with the server's public key, and send it to the server.
        // The actual session key is 32 bytes long and the rest is padding
        let session_key = Session::gen_session_key();
    
        let mut encrypted_session_key = [0u8; 256];
        rsa_key.public_encrypt(&session_key, &mut encrypted_session_key, Padding::NONE).unwrap();
        stream.write_all(&encrypted_session_key).unwrap();
    
        let session_key = &session_key[0..32];
        let cipher = Cipher::aes_256_cbc();

        println!("connection succesful");
    
        Session {
            session_key: session_key.to_owned(),
            stream: RefCell::new(stream),
            cipher,
        }
    }
}