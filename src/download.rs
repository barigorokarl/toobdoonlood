use rocket::http::{ContentType, Status};
use rocket::request::Form;
use url::Url;
use rocket::{Response, State};
use rocket::http::hyper::header::{ContentDisposition, DispositionType, DispositionParam, Charset, Location};
use std::io::{Cursor, BufReader, Read};
use std::fs::File;
use std::process::Command;
use crate::{Download, Db};
use rocket::response::Redirect;
use std::error::Error;
use anyhow::Result;

#[derive(FromForm, Debug)]
pub struct ToobDl {
    url: String,
    playlist: bool,
}

fn download_from_toob(id: &str) -> Result<Download> {
    // youtube-dl -s --get-filename -o'./dl/%(title)s.%(ext)s' --extract-audio --audio-format mp3 AllqigkmTOg

    let output = Command::new("youtube-dl")
        .arg("--print-json")
        .arg("-o")
        .arg("dl/%(id)s.%(ext)s")
        .arg("--extract-audio")
        .arg("--audio-quality=3")
        .arg(id)
        .output()?;

    let out = String::from_utf8(output.stdout)?;

    println!("{}", out);

    if output.status.success() {
        let dl = serde_json::from_str(&out)?;
        Ok(dl)
    } else {
        let oute = String::from_utf8(output.stderr)?;
        Err(anyhow::Error::msg(format!("stat{} {} {}", output.status,  out.clone(), oute.clone())))
    }
}

fn download_pl_from_toob(id: &str) -> Result<Vec<Download>> {
    // youtube-dl -s --get-filename -o'./dl/%(title)s.%(ext)s' --extract-audio --audio-format mp3 AllqigkmTOg

    let output = Command::new("youtube-dl")
        .arg("--print-json")
        .arg("-o")
        .arg("dl/%(id)s.%(ext)s")
        .arg("--extract-audio")
        .arg("--audio-quality=3")
        .arg(id)
        .output()?;

    let out = String::from_utf8(output.stdout)?;

    if output.status.success() {
        let out_str = String::from(&out);
        let mut res :Vec<Download> = Vec::new();
        let mut i = 0;
        out_str.lines().for_each(|x| {
            res.push(serde_json::from_str(&x).expect("json is parseable"));
            i += 1;
        });
        Ok(res)
    } else {
        let oute = String::from_utf8(output.stderr)?;
        Err(anyhow::Error::msg(format!("stat{} {} {}", output.status,  out.clone(), oute.clone())))
    }
}

fn get_id_from_url(url: &Url) -> Option<String> {
    match url.query() {
        Some(q) => {
            match q.split('&').find(|x| String::from(*x).starts_with("v=")) {
                Some(e) => Some(e.replace("v=", "")),
                _ => None
            }
        }
        _ => None
    }
}


fn get_pid_from_url(url: &Url) -> Option<String> {
    match url.query() {
        Some(q) => {
            match q.split('&').find(|x| String::from(*x).starts_with("list=")) {
                Some(e) => Some(e.replace("list=", "")),
                _ => None
            }
        }
        _ => None
    }
}


#[cfg(test)]
mod tests {
    use url::Url;

    #[test]
    fn get_id_from_url() {
        let u = Url::parse("https://www.youtube.com/watch?v=xXUT3XDnK5E").expect("url is parseable");
        assert_eq!(super::get_id_from_url(&u), Some(String::from("xXUT3XDnK5E")));
    }
    #[test]
    fn get_pid_from_url() {
        let u = Url::parse("https://www.youtube.com/watch?v=xXUT3XDnK5E&list=bla").expect("url is parseable");
        assert_eq!(super::get_pid_from_url(&u), Some(String::from("bla")));
    }
}

#[post("/toob", data = "<form>")]
pub fn toob_dl(form: Form<ToobDl>, db: State<Db>) -> Redirect {

    if form.playlist {
        download_playlist(&form, db);
    } else {
        download_one(&form, db);
    }

    Redirect::to("/toob-dl/")
}



fn download_playlist(form: &Form<ToobDl>, db: State<Db>) {
    let mut pid: String = String::from("");

    let id = get_pid_from_url(&Url::parse(&form.url).expect("parseable"));
    if id.is_some() {
        pid = id.clone().unwrap();
        let _dl = download_pl_from_toob(&id.unwrap());
        if _dl.is_ok() {
            for x in _dl.unwrap() {
                db.save(&x);
            }
            db.flush();
        }
    }
}

fn download_one(form: &Form<ToobDl>, db: State<Db>) {
    let mut fid: String = String::from("");

    let id = get_id_from_url(&Url::parse(&form.url).expect("parseable"));
    if id.is_some() {
        fid = id.clone().unwrap();
        let dl = download_from_toob(&id.unwrap());
        if dl.is_ok() {
            db.save(&dl.unwrap());
            db.flush();
        }
    }
}