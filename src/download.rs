use rocket::request::Form;
use url::Url;
use std::process::Command;
use rocket::response::Redirect;
use rocket::http::{Cookies};

#[derive(FromForm, Debug)]
pub struct ToobDl {
    url: String,
    playlist: bool,
    quality: i8,
}

#[post("/toob", data = "<form>")]
pub fn toob_dl(form: Form<ToobDl>, mut cookies: Cookies) -> Redirect {
    let playlist: String;

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

        let quality:i8;
        if form.quality == 10 {
            quality = 0;
        } else {
            quality = 10 - form.quality;
        }

        let o = Command::new("youtube-dl")
            .arg("--no-mtime")
            .arg(playlist)
            .arg("--extract-audio")
            .arg(format!("--audio-quality={}", quality))
            .arg("--audio-format=mp3")
            .arg("-o")
            .arg(format!("{}/%(title)s.mp3", the_dir))
            .arg(&form.url)
            .output().expect("output");

        println!("{}", String::from_utf8(o.stdout).unwrap());

        // if !form.playlist {
        //     return Redirect::to("/toob-dl/dl/latest");
        // }

    }

    Redirect::to("/toob-dl/")
}