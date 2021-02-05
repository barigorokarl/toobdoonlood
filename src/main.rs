#![feature(proc_macro_hygiene, decl_macro)]

mod index;
mod download;

#[macro_use]
extern crate rocket;
extern crate bincode;
extern crate job_scheduler;

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use rocket_contrib::templates::Template;
use crate::index::static_rocket_route_info_for_index;
use crate::download::static_rocket_route_info_for_toob_dl;
use crate::index::static_rocket_route_info_for_download;
use std::thread::spawn;
use std::fs;

fn main() {

    spawn(|| {
        let mut sched = JobScheduler::new();
        sched.add(Job::new("* * * * * *".parse().unwrap(), || {
            fs::read_dir("./dl")
                .expect("something to read")
                .for_each(|v| {
                    if v.is_ok() {
                        let d = v.expect("DirEntry");
                        let m = d.metadata().expect("no metadata");
                        let oldness = m.modified().expect("modified time").elapsed().expect("elapsed");
                        let p = d.path();
                        let path = p.to_str().expect("path");
                        if m.is_dir() && oldness > Duration::new(7200, 0) {
                            println!("remove {} because it is {} seconds old", path, oldness.as_secs().to_string());
                            fs::remove_dir_all(p.as_path()).expect("deletion");
                        }
                    }
                });
        }));
        loop {
            sched.tick();
            std::thread::sleep(Duration::from_secs(600));
        }
    });

    rocket::ignite()
        .mount("/toob-dl", routes![index, toob_dl, download])
        .mount("/toob-dl/static", rocket_contrib::serve::StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();

}
