#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

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

use std::sync::{Arc, Mutex};
use std::thread;

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
    let song_ids = song_id_arg.split(',').map(|s| s.to_owned()).collect::<Vec<String>>();

    let music_infos_collections = Arc::new(Mutex::new(Vec::new()));

    let mut children = vec![];

    for id in song_ids {
        let infos_collections = music_infos_collections.clone();
        children.push(thread::spawn(move || {
            let mut music_infos = match NetEaseMusicInfo::get_music_info(format!("{}", id).as_str()) {
                Ok(infos) => infos,
                Err(err) => {
                    info!("error: {}", err);
                    return;
                }
            };

            let music_info = music_infos.remove(0);
            
            match infos_collections.lock() {
                Ok(mut infos) => infos.push(music_info),
                Err(_) => {},
            };
        }));
    }

    for child in children {
        let _ = child.join();
    }
    debug!("duration: {:?}", SystemTime::now().duration_since(begin));

    let infos_coll = music_infos_collections.clone();
    let infos_result = infos_coll.lock();
    match infos_result {
        Ok(infos) => {
            for info in infos.iter() {
                println!("id: {}\turl: {:?}", info.id, info.url.clone().unwrap_or("没有版权信息！".to_string()));
            }            
        },
        Err(_) => {},
    }
}
