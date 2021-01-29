
use rocket::request::Form;
use url::Url;
use std::process::Command;
use rocket::response::Redirect;
use regex::Regex;
use rocket::http::{Cookies, Cookie, SameSite};
use uuid::Uuid;
use time::Duration;

#[derive(FromForm, Debug)]
pub struct ToobDl {
    url: String,
    playlist: bool,
}

#[post("/toob", data = "<form>")]
pub fn toob_dl(form: Form<ToobDl>, mut cookies: Cookies) -> Redirect {
    let mut playlist: String;

    let sid:String = match cookies.get_private("session") {
        Some(c) => c.value().to_string(),
        None => "nope".to_string()
    };
    let the_dir = format!("./dl/{}", sid);

    let u = Url::parse(&form.url);
    if sid != "nope" && u.is_ok() && u.unwrap().domain().unwrap_or("") == "www.youtube.com" {
        if form.playlist {
            playlist = format!("--yes-playlist");
        } else {
            playlist = format!("--no-playlist");
        }

        let o = Command::new("youtube-dl")
            .arg("--no-mtime")
            .arg(playlist)
            .arg("--extract-audio")
            .arg("--audio-quality=3")
            .arg("--audio-format=mp3")
            .arg("-o")
            .arg(format!("{}/%(title)s.mp3", the_dir))
            .arg(&form.url)
            .output().expect("output");

        println!("{}", String::from_utf8(o.stdout).unwrap());
    }

    Redirect::to("/toob-dl/")
}