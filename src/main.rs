#![feature(proc_macro_hygiene, decl_macro)]

mod index;
mod download;

#[macro_use]
extern crate rocket;
extern crate bincode;

use rocket_contrib::templates::Template;
use crate::index::static_rocket_route_info_for_index;
use crate::download::static_rocket_route_info_for_toob_dl;
use crate::index::static_rocket_route_info_for_download;


use std::sync::atomic::AtomicU8;
use sled::IVec;
use std::sync::atomic::Ordering;
use serde::{Serialize,Deserialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    #[serde(rename = "_filename")]
    filename: String,
    fulltitle: String,
    ext: String,
}

pub struct Db {
    pub sled: sled::Db,
    id: AtomicU8,
}

impl Db {
    pub fn save(&self, dl: &Download) {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let bytes = bincode::serialize(dl).expect("serialize");
        self.sled.insert(&[id], bytes);
    }

    pub fn flush(&self) {
        self.sled.flush();
    }

    pub fn delete(&self, id: &str) {

    }

    pub fn load(&self) -> Vec<Download> {
        let start: &[u8] = &[0];
        let end: &[u8] = &[self.id.load(Ordering::Relaxed)+1];
        let r = self.sled.range(start..end);
        r.filter(|x| x.is_ok()).map(|x| {
            let e: Download = bincode::deserialize(&x.unwrap().1).expect("deserialize");
            e
        }).collect()
    }
}

fn main() {
    let db = Db{sled: sled::open("dl_index").expect("open"), id: AtomicU8::new(0) };
    let last = db.sled.last();

    match last {
        Ok(last) => match last {
            Some(l) => {
                let lst_id: IVec = l.0;
                let i = lst_id.to_vec();
                db.id.fetch_add(i[0], Ordering::Relaxed);
            }
            _ => {},
        },
        Err(_) => {}
    }

    rocket::ignite()
        .mount("/toob-dl", routes![index, toob_dl, download])
        .manage(db)
        .attach(Template::fairing())
        .launch();
}
