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

mod music;

use music::netease::{self, NetEaseMusicInfo};

fn main() {
    let music_infos = NetEaseMusicInfo::get_music_info(22817125);
    let music_info = music_infos.first().unwrap();
    println!("music: {:?}", music_info.music_url());
}
