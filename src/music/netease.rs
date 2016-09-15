// 网易云音乐相关
use num::bigint::{BigInt, BigUint, ToBigInt};
use num::pow::pow;

use std::str;
use std::str::Chars;
use std::iter::repeat;
use std::io::{self, Read};

use std::time::{Duration, SystemTime};

use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use rand::{OsRng, Rng};

use serde_json::{self, Value};

use base64;

use hyper::Client;
use hyper::header::ContentType;

use url::form_urlencoded;

const MODULUS: &'static str = "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";
const NONCE: &'static str = "0CoJUm6Qyw8W8jud";
const PUB_KEY: &'static str = "010001";
const IV: &'static str = "0102030405060708";

const REQUEST_STR: &'static str = "http://music.163.com/weapi/song/enhance/player/url?csrf_token=";

#[derive(Debug, Deserialize)]
#[serde(rename = "data")]
pub struct NetEaseMusicInfo {
    id: u64,
    url: Option<String>,
    br: u32,
    md5: Option<String>,
    // music_type: super::MusicType,
}

#[derive(Debug, Deserialize)]
struct TempData {
    data: Vec<NetEaseMusicInfo>,
}

impl NetEaseMusicInfo {
    pub fn get_music_info(music_id: u64) -> Vec<NetEaseMusicInfo> {
        let message = format!("{{\"ids\": [{}], \"br\": 32000}}", music_id);

        let mut rng = OsRng::new().expect("Failed to get OS random generator");

        let encrypted_data = aes_encrypt(message.as_bytes(), NONCE.as_bytes()).expect("aes failed");

        let params = base64::encode(&encrypted_data);

        let random_key = "cc09f2ec1dc8ded1";
        
        let encrypted_data = aes_encrypt(params.as_bytes(), random_key.as_bytes()).unwrap();
        let params = base64::encode(&encrypted_data);

        let begin = SystemTime::now();
        let sec_key = rsa_encrypt(random_key);
        // println!("interval: {:?}", SystemTime::now().duration_since(begin).unwrap());

        let client = Client::new();
        
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("params", params.as_str())
            .append_pair("encSecKey", sec_key.as_str())
            .finish();

        // let mut headers = Headers::new();
        // headers.set_raw("content-type", vec![b"x-www-form-urlencoded".to_vec()]);
        let mut res = client.post(REQUEST_STR).header(ContentType(mime!(Application/WwwFormUrlEncoded))).body(encoded.as_str()).send().expect("post failed");

        let mut json = String::new();
        res.read_to_string(&mut json);

        // println!("response: {:?} json result: {}", res, json);

        let tmp_data: TempData = serde_json::from_str(json.as_str()).unwrap();
        tmp_data.data
    }
    pub fn music_url(&self) -> &Option<String> {
        &self.url
    }
}

fn rsa_encrypt(text: &str) -> String {
    let text = text.chars().rev().map(|c| format!("{:x}", c as u8)).collect::<String>();

    let n1 = BigInt::parse_bytes(text.as_bytes(), 16).unwrap();
    let n2 = usize::from_str_radix(PUB_KEY, 16).unwrap();
    let n3 = BigInt::parse_bytes(MODULUS.as_bytes(), 16).unwrap();
    
    let n = pow(n1, n2) % n3;

    format!("{:0256x}", n)
}

fn aes_encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize128, key, IV.as_bytes(), blockmodes::PkcsPadding);

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
