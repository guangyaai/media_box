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

use std::env;
use std::time::SystemTime;

use music::netease::NetEaseMusicInfo;

fn main() {
    env_logger::init().unwrap();

    let begin = SystemTime::now();

    let mut args = env::args();

    let song_id_arg = match args.nth(1) {
        Some(id) => id,
        None => {
            println!("请输入需要查询的音乐id！");
            return;
        }
    };
    let song_ids = song_id_arg.split(',').collect::<Vec<&str>>();

    let mut music_infos_collections: Vec<NetEaseMusicInfo> = vec![];

    for id in song_ids {
        let mut music_infos = match NetEaseMusicInfo::get_music_info(id) {
            Ok(infos) => infos,
            Err(err) => {
                info!("error: {}", err);
                continue;
            }
        };

        let music_info = music_infos.remove(0);

        music_infos_collections.push(music_info);
    }
    debug!("duration: {:?}", SystemTime::now().duration_since(begin));

    for info in music_infos_collections {
        println!("id: {}\turl: {}", info.id, info.url.unwrap_or("没有版权信息!".to_string()));
    }
}
