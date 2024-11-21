extern crate rusqlite;
use crate::Upload;
use rand::{self, Rng};
use rocket::form::Form;
use std::fmt;
use std::{borrow::Cow, path::Path};

use rusqlite::{Connection, Result};

pub(crate) const SAVE_LOCATION: &str = "/images/";
pub(crate) const DATABASE_LOCATION: &str = "todo_db_here";
const SHORT_URL_LEN: usize = 6;

#[derive(Debug, Default)]
pub struct ShortUrl<'a>(Cow<'a, str>);

fn gen_random_string(size: usize) -> String {
    const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-;+=";
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
    /*
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    */
}

impl fmt::Display for ShortUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
struct Url<'a> {
    orig_url: String,
    short_url: ShortUrl<'a>,
}

#[derive(Debug, Default, Clone)]
struct Img {
    uuid: String,
    filetype: String,
    data: Option<Vec<u8>>,
}

pub(crate) fn retrieve(short_url: String) -> Result<String> {
    let conn = Connection::open(DATABASE_LOCATION)?;

    let mut stmt = conn.prepare("SELECT orig_url FROM urls WHERE short_url=:short_url LIMIT 1")?;
    let urls_iter = stmt.query_map(&[(":short_url", short_url.as_str())], |row| {
        Ok(Url {
            orig_url: row.get(0)?,
            short_url: ShortUrl::default(),
        })
    })?;

    #[allow(clippy::never_loop)]
    for u in urls_iter {
        // we should return here whenever we have a url
        return Ok(u.unwrap().orig_url);
    }
    Ok("https://letmegooglethat.com/?q=404".to_string())
}

pub(crate) fn retrieve_img(uuid: String) -> Result<String> {
    //eprintln!("got uuid {:?}", uuid);
    let uuid_as_path = Path::new(&uuid);
    // TODO clean this up
    let f_without_extension = uuid_as_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .clone();
    //eprintln!("f without extension: {:?}", f_without_extension);
    let conn = Connection::open(DATABASE_LOCATION)?;
    // TODO remove img binary instances/calls because we are storing images on the fs now
    let mut stmt = conn
        .prepare("SELECT uuid, filetype, img FROM imgs WHERE uuid=:f_without_extension LIMIT 1")?;
    let imgs_iter = stmt.query_map(
        &[(":f_without_extension", f_without_extension.as_str())],
        |row| {
            Ok(Img {
                uuid: row.get(0)?,
                filetype: row.get(1)?,
                data: row.get(2)?,
            })
        },
    )?;
    // more TODO here; figure out how to chain this or do it cleanly
    let cwd = std::env::current_dir().unwrap();
    let cwd2 = cwd.to_string_lossy() + SAVE_LOCATION;
    #[allow(clippy::never_loop)]
    for i in imgs_iter {
        let my_image = i.as_ref().unwrap().clone();
        let final_destination = cwd2.into_owned()
            + &my_image.uuid
            + file_extension(my_image.filetype.as_str()).as_str();
        //eprintln!("final_destination: {:?}", final_destination);
        return Ok(final_destination);
    }
    Ok("https://letmegooglethat.com/?q=404".to_string())
}

pub(crate) fn create_tables() -> Result<()> {
    let conn = Connection::open(DATABASE_LOCATION)?;

    // need execute_batch or it runs only the first statement
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
    let id = gen_random_string(SHORT_URL_LEN);
    let filename = String::from(SAVE_LOCATION) + &id;
    let ctype = form.image.content_type();

    let ni = Img {
        data: None,
        uuid: id,
        filetype: ctype.unwrap().to_string(),
    };

    let conn = Connection::open(DATABASE_LOCATION)?;
    conn.execute(
        "INSERT INTO imgs (img, uuid, filetype) VALUES (?1, ?2, ?3)",
        (&ni.data, &ni.uuid, &ni.filetype),
    )?;
    // more TODO this cleanly
    let cwd = std::env::current_dir().unwrap();
    let cwd2 = cwd.to_string_lossy();
    let final_destination = cwd2.into_owned() + &filename + file_extension(&ni.filetype).as_str();
    let _r = form.image.move_copy_to(final_destination.clone()).await;
    Ok(ni.uuid + &file_extension(&ni.filetype))
}

fn file_extension(c: &str) -> String {
    // TODO add more options here
    match c {
        "image/jpeg" => ".jpg".to_string(),
        "image/png" => ".png".to_string(),
        _ => ".png".to_string(),
    }
}

pub(crate) fn ensure_images_directory(target: &str) -> std::io::Result<()> {
    if !std::path::Path::new(&target).exists() {
        std::fs::create_dir(target)?;
    }
    Ok(())
}

pub(crate) fn new(orig_url: String) -> Result<String> {
    let short_url = ShortUrl::new(SHORT_URL_LEN);
    let url = Url {
        orig_url: orig_url.clone(),
        short_url,
    };

    let conn = Connection::open(DATABASE_LOCATION)?;

    let mut stmt = conn.prepare("SELECT short_url FROM urls WHERE orig_url=:orig_url LIMIT 1")?;
    let urls_iter = stmt.query_map(&[(":orig_url", orig_url.as_str())], |row| {
        Ok(Url {
            short_url: ShortUrl::default(),
            orig_url: row.get(0)?,
        })
    })?;

    // if the orig_url already exists in the db, return just that short_url
    #[allow(clippy::never_loop)]
    for u in urls_iter {
        return Ok(u.unwrap().orig_url);
    }

    conn.execute(
        "INSERT INTO urls (orig_url, short_url) VALUES (?1, ?2)",
        (&url.orig_url, &url.short_url.to_string()),
    )?;

    Ok(url.short_url.to_string())
}
