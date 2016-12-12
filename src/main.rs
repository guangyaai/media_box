#![feature(proc_macro)]

extern crate crypto;
extern crate rand;
extern crate base64;
extern crate num as big_num;
extern crate ramp;

#[macro_use]
extern crate serde_derive;
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

use std::io;
use std::num;

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
                    info!("error: {:?}", err);
                    return;
                }
            };

            let music_info = music_infos.remove(0);
            
            match infos_collections.lock() {
                Ok(mut infos) => infos.push(music_info),
                Err(_) => debug!("没有获取锁！"),
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
        Err(_) => debug!("主线程获取锁失败！"),
    }
}

#[derive(Debug)]
pub enum MediaBoxError {
    ParseNum(num::ParseIntError),
    ParseRamp(ramp::int::ParseIntError),
    Io(io::Error),
    Network(hyper::Error),
    Json(serde_json::Error),
    BigInt(String),
}

impl From<num::ParseIntError> for MediaBoxError {
    fn from(err: num::ParseIntError) -> MediaBoxError {
        MediaBoxError::ParseNum(err)
    }
}
impl From<ramp::int::ParseIntError> for MediaBoxError {
    fn from(err: ramp::int::ParseIntError) -> MediaBoxError {
        MediaBoxError::ParseRamp(err)
    }
}
impl From<io::Error> for MediaBoxError {
    fn from(err: io::Error) -> MediaBoxError {
        MediaBoxError::Io(err)
    }
}
impl From<hyper::Error> for MediaBoxError {
    fn from(err: hyper::Error) -> MediaBoxError {
        MediaBoxError::Network(err)
    }
}
impl From<serde_json::Error> for MediaBoxError {
    fn from(err: serde_json::Error) -> MediaBoxError {
        MediaBoxError::Json(err)
    }
}
impl From<String> for MediaBoxError {
    fn from(err: String) -> MediaBoxError {
        MediaBoxError::BigInt(err)
    }
}
