extern crate time;
use rocket_contrib::templates::Template;
use serde::{Serialize};
use rocket::{Response};
use std::fs::{File};
use std::io::{BufReader, Read, Cursor};
use rocket::http::{ContentType, Cookie, Cookies, SameSite, Status};
use rocket::http::hyper::header::{ContentDisposition, DispositionType, DispositionParam, Charset, Location};
use std::fs;
use itertools::Itertools;
use uuid::Uuid;
use time::Duration;

#[derive(Serialize)]
struct IndexGet {
    existing: Vec<String>
}

#[get("/")]
pub fn index(mut cookies: Cookies) -> Template {

    let sid:String = match cookies.get_private("session") {
        Some(c) => c.value().to_string(),
        None => {
            let id = Uuid::new_v4().to_string();
            let c = Cookie::build("session", id.clone())
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .max_age(Duration::hours(2))
                .finish();
            cookies.add_private(c);
            id
        }
    };

    let the_dir = format!("./dl/{}", sid);

    fs::create_dir_all(the_dir.clone()).expect("create directory");

    let d = dir_contents(the_dir);
    let context = IndexGet { existing: d};
    Template::render("index", &context)
}

fn dir_contents(the_dir: String) -> Vec<String> {
    let d = fs::read_dir(the_dir)
        .expect("something to read")
        .filter_map(|v| v.ok())
        .sorted_by(|a, b| Ord::cmp(
            &a.metadata().expect("a metadata").modified().expect("modified time").elapsed().expect("system time").as_secs(),
            &b.metadata().expect("a metadata").modified().expect("modified time").elapsed().expect("system time").as_secs()
        ))
        .map(|x| x.file_name().to_str().expect("string").to_owned())
        .collect::<Vec<String>>();
    d
}

#[get("/dl/<id>")]
pub fn download<'a>(id: String, mut cookies: Cookies) -> Response<'a> {

    let sid:String = match cookies.get_private("session") {
        Some(c) => c.value().to_string(),
        None => "nope".to_string()
    };

    if sid == "nope" {
        let er = not_found_response();
        return er;
    }

    let the_dir = format!("./dl/{}", sid);
    let filepath :String;
    if id == "latest" {
        match dir_contents(the_dir.clone()).first() {
            Some(e) => {
                println!("{}", e.to_string());
                filepath = format!("{}/{}", &the_dir, e);
            },
            None => {
                let er = not_found_response();
                return er;
            }
        }
    } else {
        filepath = format!("{}/{}", the_dir, id);
    }
    let f = File::open(&filepath).expect("open file");
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    let _b = reader.read_to_end(&mut buf).expect("read file");
    let cursor = Cursor::new(buf);

    let r = Response::build()
        .header(ContentType::Binary)
        .header(Location("/toob-dl/".to_string()))
        .header_adjoin(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(
                Charset::Iso_8859_1,
                None,
                format!("{}", id).into(),
            )],
        })
        .sized_body(cursor)
        .finalize();
    r
}

fn not_found_response() -> Response<'static> {
    let er = Response::build()
        .status(Status::NotFound)
        .finalize();
    er
}