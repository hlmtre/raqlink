extern crate rusqlite;
use crate::Upload;
use rand::{self, Rng};
use rocket::form::Form;
use rocket::fs::TempFile;
use std::borrow::Cow;

use rusqlite::{Connection, Result};

const SAVE_LOCATION: &str = "./images/";
const SHORT_URL_LEN: usize = 6;

#[derive(Debug, Default)]
pub struct ShortUrl<'a>(Cow<'a, str>);

fn gen_random_string(size: usize) -> String {
    const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut id = String::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        id.push(BASE62[rng.gen::<usize>() % 62] as char);
    }
    id
}

impl ShortUrl<'_> {
    pub fn new(size: usize) -> ShortUrl<'static> {
        ShortUrl(Cow::Owned(gen_random_string(size)))
    }
    pub fn to_string(&self) -> String {
        self.0.to_string().to_string()
    }

    /*
    pub fn from_string(f: String) -> Result<Self> {
        let mut s = ShortUrl::new(SHORT_URL_LEN);
        s.0 = f.into();
        Ok(s)
    }
    */
}

#[derive(Debug, Default)]
struct Url<'a> {
    orig_url: String,
    short_url: ShortUrl<'a>,
}

#[derive(Debug, Default)]
struct Img {
    uuid: String,
    filetype: String,
    data: Option<Vec<u8>>,
}

pub(crate) fn retrieve(short_url: String) -> Result<String> {
    let conn = Connection::open("aqlink-testing.db")?;

    let mut stmt = conn.prepare("SELECT orig_url FROM urls WHERE short_url=:short_url LIMIT 1")?;
    let urls_iter = stmt.query_map(&[(":short_url", short_url.as_str())], |row| {
        Ok(Url {
            orig_url: row.get(0)?,
            short_url: ShortUrl::default(),
        })
    })?;

    for u in urls_iter {
        // we should return here whenever we have a url
        return Ok(u.unwrap().orig_url);
    }
    Ok("https://letmegooglethat.com/?q=404".to_string())
}

pub(crate) fn create_tables() -> Result<()> {
    let conn = Connection::open("aqlink-testing.db")?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS urls (
        orig_url text primary key,
        short_url text);
              CREATE TABLE IF NOT EXISTS imgs (
        id INTEGER PRIMARY KEY,
        uuid TEXT,
        filetype TEXT,
        img BLOB
        )",
    )?;

    Ok(())
}

pub(crate) async fn new_img(mut form: Form<Upload<'_>>) -> Result<String> {
    eprintln!("image: {:?}", form.image);
    /*
    println!("ni = {:?}", ni);
    */
    let id = gen_random_string(SHORT_URL_LEN);
    let filename = String::from(SAVE_LOCATION) + &id;
    let ctype = form.image.content_type();

    let i = read_file_to_bytes(form.image.path().unwrap().to_str().unwrap()).unwrap();
    eprintln!("i: {:?}", i);

    let ni = Img {
        data: Some(i),
        uuid: gen_random_string(SHORT_URL_LEN),
        filetype: ctype.unwrap().to_string(),
    };

    let conn = Connection::open("aqlink-testing.db")?;
    conn.execute(
        "INSERT INTO imgs (img, uuid, filetype) VALUES (?1, ?2, ?3)",
        (&ni.data, &ni.uuid, &ni.filetype),
    )?;
    let _ = form.image.move_copy_to(filename.clone()).await;
    Ok(filename)
}

fn read_file_to_bytes(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let byte_content = std::fs::read(path)?;
    Ok(byte_content)
}

pub(crate) fn new(orig_url: String) -> Result<String> {
    let short_url = ShortUrl::new(SHORT_URL_LEN);
    let url = Url {
        orig_url: orig_url.clone(),
        short_url,
    };

    let conn = Connection::open("aqlink-testing.db")?;

    let mut stmt = conn.prepare("SELECT short_url FROM urls WHERE orig_url=:orig_url LIMIT 1")?;
    let urls_iter = stmt.query_map(&[(":orig_url", orig_url.as_str())], |row| {
        Ok(Url {
            short_url: ShortUrl::default(),
            orig_url: row.get(0)?,
        })
    })?;

    // if the orig_url already exists in the db, return just that short_url
    for u in urls_iter {
        return Ok(u.unwrap().orig_url);
    }

    conn.execute(
        "INSERT INTO urls (orig_url, short_url) VALUES (?1, ?2)",
        (&url.orig_url, &url.short_url.to_string()),
    )?;

    Ok(url.short_url.to_string())
}
