use rocket_contrib::templates::Template;
use serde::{Serialize};
use crate::{Download, Db};
use rocket::{State, Response};
use rocket::response::status::NotFound;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use rocket::http::ContentType;
use rocket::http::hyper::header::{ContentDisposition, DispositionType, DispositionParam, Charset};

#[derive(Serialize)]
struct IndexGet {
    existing: Vec<Download>
}

#[get("/")]
pub fn index(db: State<Db>) -> Template {
    let context = IndexGet { existing: db.load()};
    Template::render("index", &context)
}

#[get("/dl/<id>")]
pub fn download(id: String, db: State<Db>) -> Response {
    let id = format!("dl/{}", id);
    let dl = db.load().into_iter().find(|x| x.filename.eq(&id)).unwrap();
    let cwd = std::env::current_dir().expect("current working directory");
    let f = File::open(dl.filename).expect("open file");
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    let _b = reader.read_to_end(&mut buf).expect("read file");
    let cursor = Cursor::new(buf);

    let r = Response::build()
        .header(ContentType::Binary)
        .header_adjoin(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(
                Charset::Iso_8859_1,
                None,
                format!("{}.{}", dl.fulltitle, dl.ext).into(),
            )],
        })
        .sized_body(cursor)
        .finalize();
    r
}