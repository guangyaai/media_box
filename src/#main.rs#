extern crate ring;
extern crate crypto;
extern crate rand;
extern crate base64;
extern crate num;

use num::bigint::{BigInt, BigUint, ToBigInt};
use num::pow::pow;

mod music;

use std::str;
use std::str::Chars;
use std::iter::repeat;

use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use rand::{OsRng, Rng};
use ring::aead;
use music::netease;

const IV: &'static str = "0102030405060708";
const NOUNCE: &'static str = "0CoJUm6Qyw8W8jud";
const SEC_KEY: &'static str = "08ba71e3f6739995";
const PUBKEY: &'static str = "010001";
const MODULUS: &'static str = "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";

fn rsa_encrypt(text: &str) -> String {
    let text = text.chars().rev().map(|c| format!("{:x}", c as u8)).collect::<String>();
    println!("text: {}", text);

    let n1 = BigInt::parse_bytes(text.as_bytes(), 16).unwrap();
    let n2 = usize::from_str_radix(PUBKEY, 16).unwrap();
    let n3 = BigInt::parse_bytes(MODULUS.as_bytes(), 16).unwrap();
    println!("n1: {}, n2: {}, n3: {}", n1, n2, n3);
    
    let n = pow(n1, n2) % n3;
    println!("n: {}", n);
    format!("{:0256x}", n)
}

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize128, key, iv, blockmodes::PkcsPadding);

    let mut final_result: Vec<u8> = vec![];
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 2048];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn main() {
    println!("Hello, world! {}", netease::music_url("abcde"));

    
    // println!("sealing key: {:?}", sealing_key);

    // let sealing_key: aead::SealingKey = aead::SealingKey::new(&aead::AES_128_GCM, IV.as_bytes()).unwrap();

    // let mut v1 = aes_encrypt(r#"{"ids": [11212], "br": 32000}"#).into_bytes();
    // println!("v1: {:?}", v1);
    // let mut v1 = vec![0; 200];

    // let s_alg = sealing_key.algorithm();
    // println!("{} {}", s_alg.key_len(), s_alg.nonce_len());
    // println!("max over: {} {}", sealing_key.algorithm().max_overhead_len(), aead::MAX_OVERHEAD_LEN);
    
    // let result = aead::seal_in_place(&sealing_key, NOUNCE.as_bytes(), &mut v1, aead::MAX_OVERHEAD_LEN, b"as").unwrap();
    // println!("{:?} {}", result, aead::MAX_OVERHEAD_LEN);

    // println!("result: {:?}", &v1[..result]);

    // println!("length: {}", "jIreCfXUS16Mh1s%2BmUuz1ndex8NSLnK6ozE3TLk71c9eMT3TsVQGU2nKcUbmMXRAOEHYKjKAe4lASsqaGncbDwk1QDhSCg8F1S0S%2BZAF3XiTA3sLQrLKt%2B7Kc0XW31eaVBml9Z%2B81pUaKFP%2BIRn%2B2rmmRnxBU%2BXxW%2BkL1Kd6cPM3U0Exhobjqp81jyakNZYF".chars().count());
    let message = "abcdefg";
    let message = r#"{"ids": [], "br": 32000}."#;

    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    
    let mut rng = OsRng::new().expect("Failed to get OS random generator");
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    println!("text: {} nounce: {}", message, NOUNCE);
    let encrypted_data = encrypt(message.as_bytes(), NOUNCE.as_bytes(), IV.as_bytes()).unwrap();

    let result = base64::encode(&encrypted_data);
    println!("encrypted: {:?}", result);
    
    let encrypted_data = encrypt(result.as_bytes(), "cc09f2ec1dc8ded1".as_bytes(), IV.as_bytes()).unwrap();

    println!("encrypted: {:?}", base64::encode(&encrypted_data));

    println!("rsa: {}", rsa_encrypt("cc09f2ec1dc8ded1"));
}
