// 网易云音乐相关


const MODULUS: &'static str = "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";
const NONCE: &'static str = "0CoJUm6Qyw8W8jud";
const PUB_KEY: &'static str = "010001";

fn aes_encrypt(text: &str, sec_key: &str) -> String {
    let pad = 16 - text.chars().count();

    let mut new_text = text.to_string();

    let tail = format!("{}", pad);

    for _ in 0..pad {
        new_text += tail.as_str();
    }
    
    new_text
}

fn rsa_encrypt(text: &str, pub_key: &str, modulus: &str) -> String {
    let text: String = text.chars().rev().collect();
    text
}

pub fn music_url(name: &str) -> String {
    rsa_encrypt(name, PUB_KEY, MODULUS)
}
