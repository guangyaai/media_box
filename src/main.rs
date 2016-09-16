#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate ring;
extern crate crypto;
extern crate rand;
extern crate base64;
extern crate num;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;

#[macro_use]
extern crate mime;

#[macro_use]
extern crate log;
extern crate env_logger;

mod music;

use music::netease::NetEaseMusicInfo;

fn main() {
    env_logger::init().unwrap();
    
    let music_infos = NetEaseMusicInfo::get_music_info(22817125);

    match music_infos {
        Ok(real_music_infos) => {
            let head_music = real_music_infos.first();
            if let Some(first_music) = head_music {
                let music_url = first_music.music_url();
                match music_url {
                    Some(url) => println!("music url: {}", url),
                    None => println!("没有音乐版权信息!"),
                }
            } else {
                println!("无法获取音乐信息!")
            }                
        },
        Err(error) => println!("err: {}", error),
    }
}
