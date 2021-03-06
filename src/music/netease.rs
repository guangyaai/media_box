// 网易云音乐相关
extern crate ramp;

use big_num::bigint::BigInt;
use big_num::pow::pow;

use std::str;
use std::iter::repeat;
use std::io::Read;

use std::error::Error;

use std::time::SystemTime;

use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use rand::{OsRng, Rng};

use serde_json;

use base64;

use hyper::Client;
use hyper::header::ContentType;

use url::form_urlencoded;

use MediaBoxError;

const MODULUS: &'static str = "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";
const NONCE: &'static str = "0CoJUm6Qyw8W8jud";
const PUB_KEY: &'static str = "010001";
const IV: &'static str = "0102030405060708";

const REQUEST_STR: &'static str = "http://music.163.com/weapi/song/enhance/player/url?csrf_token=";

#[derive(Debug, Deserialize)]
#[serde(rename = "data")]
pub struct NetEaseMusicInfo {
    pub id: u64,
    pub url: Option<String>,
    br: u32,
    md5: Option<String>,
    // music_type: super::MusicType,
}

impl NetEaseMusicInfo {
    pub fn get_music_info(music_id: &str) -> Result<Vec<NetEaseMusicInfo>, MediaBoxError> {
        let message = format!("{{\"ids\": [{}], \"br\": 32000}}", music_id);

        let encrypted_data = aes_encrypt(message.as_bytes(), NONCE.as_bytes()).unwrap();

        let params = base64::encode(&encrypted_data);

        let random_key = "cc09f2ec1dc8ded1".as_bytes();
        // let random_key = create_random_key(16);
        
        let encrypted_data = aes_encrypt(params.as_bytes(), &random_key).unwrap();
        let params = base64::encode(&encrypted_data);

        let begin = SystemTime::now();
        // println!("random key: {:?}", random_key);
        // let sec_key = try!(rsa_encrypt(&random_key));
        let sec_key = "b00ecb5f666b22b0271ca83afa5b30e9483dafcc051d9b7819e1ae2d77f165826c27609c0a26c9c34a3b2495951c2983ca1c67d7bd2e2ff11950d9f2a67f496fbf1c73b89baa5adae68ea5a9d9a58b245c2f289aff501ad315469e709c20c3aa74b7317b92e022f196ec5af344ff5b93b5360125c4b85af86d1c4c94f3aa987b".to_string();
        info!("interval: {:?}", SystemTime::now().duration_since(begin).unwrap());

        let client = Client::new();
        debug!("params: {}\nencSeckey: {}", params, sec_key);
        
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("params", params.as_str())
            .append_pair("encSecKey", sec_key.as_str())
            .finish();

        let mut res = try!(client.post(REQUEST_STR).header(ContentType(mime!(Application/WwwFormUrlEncoded))).body(encoded.as_str()).send());

        let mut json = String::new();
        try!(res.read_to_string(&mut json));

        #[derive(Debug, Deserialize)]
        struct TempData {
            data: Vec<NetEaseMusicInfo>,
        }

        let tmp_data: TempData = try!(serde_json::from_str(json.as_str()));
        Ok(tmp_data.data)
    }
    pub fn music_url(&self) -> Option<String> {
        self.url.clone()
    }
}

fn create_random_key(size: usize) -> Vec<u8> {
    let mut rng = OsRng::new().expect("Failed to get OS random generator");

    let mut random_key: Vec<u8> = repeat(0u8).take(16).collect();
    rng.fill_bytes(&mut random_key);
    let random_char = random_key
        .iter()
        .flat_map(|n| format!("{:x}", n).into_bytes())
        .take(size)
        .collect::<Vec<u8>>();

    random_char
}

fn rsa_encrypt(text: &[u8]) -> Result<String, MediaBoxError> {
    let inner_text = text
        .iter()
        .rev()
        .map(|&n| format!("{:x}", n))
        .collect::<String>();
    let inner = inner_text.as_bytes();

    // let n1 = ramp::Int::from_str_radix(&inner_text, 16)?;
    let n1 = try!(BigInt::parse_bytes(&inner, 16).ok_or("n1 解析失败".to_string()));

    let n2 = try!(usize::from_str_radix(PUB_KEY, 16));
    let n3 = try!(BigInt::parse_bytes(MODULUS.as_bytes(), 16).ok_or("n3 解析失败".to_string()));
    // let n3 = ramp::Int::from_str_radix(MODULUS, 16)?;
    
    let n = pow(n1, n2) % n3;

    Ok(format!("{:0256x}", n))
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
