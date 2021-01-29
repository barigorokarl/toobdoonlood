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

fn main() {

    rocket::ignite()
        .mount("/toob-dl", routes![index, toob_dl, download])
        .mount("/toob-dl/static", rocket_contrib::serve::StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}
