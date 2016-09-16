// 网易云音乐相关
use num::bigint::BigInt;
use num::pow::pow;

use std::str;
use std::iter::repeat;
use std::io::Read;

// use std::time::{Duration, SystemTime};

use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use rand::{OsRng, Rng};

use serde_json;

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



impl NetEaseMusicInfo {
    pub fn get_music_info(music_id: u64) -> Vec<NetEaseMusicInfo> {
        let message = format!("{{\"ids\": [{}], \"br\": 32000}}", music_id);

        let encrypted_data = aes_encrypt(message.as_bytes(), NONCE.as_bytes()).expect("aes failed");

        let params = base64::encode(&encrypted_data);

        // let random_key = "cc09f2ec1dc8ded1".as_bytes();
        let random_key = create_random_key(16);
        
        let encrypted_data = aes_encrypt(params.as_bytes(), &random_key).unwrap();
        let params = base64::encode(&encrypted_data);

        // let begin = SystemTime::now();
        println!("random key: {:?}", random_key);
        let sec_key = rsa_encrypt(&random_key);
        // println!("interval: {:?}", SystemTime::now().duration_since(begin).unwrap());

        let client = Client::new();
        println!("params: {} encSeckey: {}", params, sec_key);
        
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("params", params.as_str())
            .append_pair("encSecKey", sec_key.as_str())
            .finish();

        // let mut headers = Headers::new();
        // headers.set_raw("content-type", vec![b"x-www-form-urlencoded".to_vec()]);
        println!("encoded: {}", encoded);
        let mut res = client.post(REQUEST_STR).header(ContentType(mime!(Application/WwwFormUrlEncoded))).body(encoded.as_str()).send().expect("post failed");

        let mut json = String::new();
        res.read_to_string(&mut json);

        // println!("response: {:?} json result: {}", res, json);
        #[derive(Debug, Deserialize)]
        struct TempData {
            data: Vec<NetEaseMusicInfo>,
        }

        let tmp_data: TempData = serde_json::from_str(json.as_str()).expect("json 解析失败");
        tmp_data.data
    }
    pub fn music_url(&self) -> &Option<String> {
        &self.url
    }
}

fn create_random_key(size: usize) -> Vec<u8> {
    let mut rng = OsRng::new().expect("Failed to get OS random generator");

    let mut random_key: Vec<u8> = repeat(0u8).take(16).collect();
    rng.fill_bytes(&mut random_key);
    let random_char = random_key
        .iter()
        .map(|n| format!("{:x}", n))
        .collect::<String>()
        .as_bytes()
        .iter()
        .take(size)
        .map(|&n| n)
        .collect::<Vec<u8>>();
    println!("random char: {:?}", random_char);
    random_char
}

fn rsa_encrypt(text: &[u8]) -> String {
    let inner_text = text
        .iter()
        .rev()
        .map(|&n| format!("{:x}", n))
        .collect::<String>();
    let inner = inner_text.as_bytes();
    
    println!("text: {} inner_text: {}", str::from_utf8(text).unwrap(), str::from_utf8(&inner).unwrap());

    let n1 = BigInt::parse_bytes(&inner, 16).expect("输入的文本解析失败");

    let n2 = usize::from_str_radix(PUB_KEY, 16).expect("PUB_KEY 解析错误");
    let n3 = BigInt::parse_bytes(MODULUS.as_bytes(), 16).expect("MODULUE 解析失败");
    
    let n = pow(n1, n2) % n3;

    println!("result: {:0256x}", n);

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
