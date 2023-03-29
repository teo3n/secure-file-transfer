use openssl::rsa::{Rsa, Padding};
use openssl::symm::{decrypt, encrypt, Cipher};
use rand::RngCore;
use rand::rngs::OsRng;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::consts::{IV_LEN, SESSION_KEY_FULL_LEN, SESSION_KEY_LEN};

pub struct Session {
    pub session_key: Vec<u8>,
    pub stream: RefCell<TcpStream>,
    pub cipher: Cipher,
}

impl Session {
    pub fn decrypt(&self, encrypted_message: &Vec<u8>) -> String {
        let iv: [u8; IV_LEN] = encrypted_message[..IV_LEN].try_into().unwrap();
        let message = decrypt(self.cipher, &self.session_key, Some(&iv), &encrypted_message[IV_LEN..]).unwrap();
        String::from_utf8_lossy(&message).to_string()
    }

    pub fn receive_message(&self, len: usize) -> Vec<u8> {
        // Receive an encrypted response message from the server, decrypt it with the session key
        let mut encrypted_message = vec![0; len];
        self.stream.borrow_mut().read_exact(&mut encrypted_message).unwrap();

        encrypted_message
    }

    fn gen_iv(&self) -> [u8; IV_LEN] {
        let mut rng = OsRng;
        let mut iv = [0u8; IV_LEN];
        rng.fill_bytes(&mut iv[..]);
        iv
    }

    pub fn transmit(&self, bytes: &[u8]) {
        let iv = self.gen_iv();
        let encrypted_message = encrypt(self.cipher, &self.session_key, Some(&iv), bytes).unwrap();
        let mut msg_with_iv = TryInto::<Vec<u8>>::try_into(iv).unwrap();
        msg_with_iv.extend(encrypted_message);

        // write the length buffer
        let len_buf = (msg_with_iv.len() as u32).to_le_bytes();
        self.stream.borrow_mut().write_all(&len_buf).unwrap();

        // write the actual data buffer
        self.stream.borrow_mut().write_all(&msg_with_iv).unwrap();
        self.stream.borrow_mut().flush().unwrap();
    }

    pub fn receive(&self) -> String {
        let lenbuf = self.receive_message(4);
        let recv_datalen = u32::from_le_bytes(lenbuf.try_into().unwrap());

        self.decrypt(&self.receive_message(recv_datalen as usize))
    }

    pub fn establish_connection(stream: RefCell<TcpStream>) -> Self {
        // Generate a public-private key pair
        let rsa_key = Rsa::generate(2048).unwrap();
        println!("keys generated");

        // Get the public key as a byte slice
        let public_key = rsa_key.public_key_to_der().unwrap();
        println!("public key with length {}", public_key.len());

        // Send the public key to the client
        stream.borrow_mut().write_all(&public_key).unwrap();
        println!("public key sent");

        // Read the encrypted session key from the client
        let mut encrypted_session_key = [0u8; SESSION_KEY_FULL_LEN];
        stream
            .borrow_mut()
            .read_exact(&mut encrypted_session_key)
            .unwrap();
        println!("session key read");

        // Decrypt the session key with the private key
        let mut decrypted_session_key = [0u8; SESSION_KEY_FULL_LEN];
        rsa_key
            .private_decrypt(
                &encrypted_session_key,
                &mut decrypted_session_key,
                Padding::NONE,
            )
            .unwrap();

        let session_key = &decrypted_session_key[..SESSION_KEY_LEN];
        println!("session key decrypted");

        let cipher = Cipher::aes_256_cbc();

        println!("connection succesful");

        Session {
            session_key: session_key.to_owned(),
            stream: stream,
            cipher,
        }
    }
}
