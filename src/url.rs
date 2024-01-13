extern crate rusqlite;
use rand::{self, Rng};
use rocket::response::Redirect;
use std::borrow::Cow;

use rusqlite::{Connection, Result};

const SHORT_URL_LEN: usize = 6;

#[derive(Debug, Default)]
pub struct ShortUrl<'a>(Cow<'a, str>);

impl ShortUrl<'_> {
    pub fn new(size: usize) -> ShortUrl<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        ShortUrl(Cow::Owned(id))
    }
    pub fn to_string(&self) -> String {
        format!("{}", &self.0.to_string())
    }

    pub fn from_string(&mut self, short_url: String) -> &ShortUrl {
        self.0 = std::borrow::Cow::Borrowed(short_url.as_str());
        self
    }
}

#[derive(Debug, Default)]
struct Url<'a> {
    orig_url: String,
    short_url: ShortUrl<'a>,
}

pub(crate) fn retrieve(short_url: String) -> Result<String> {
    let conn = Connection::open("aqlink.db")?;

    let mut stmt = conn.prepare("SELECT orig_url FROM urls WHERE short_url=:short_url LIMIT 1")?;
    let urls_iter = stmt.query_map(&[(":short_url", short_url.as_str())], |row| {
        Ok(Url {
            orig_url: row.get(0)?,
            short_url: ShortUrl::default(),
        })
    })?;

    for u in urls_iter {
        return Ok(u.unwrap().orig_url);
    }
    Ok("foo".to_string())
}

pub(crate) fn create_tables() -> Result<()> {
    let conn = Connection::open("aqlink.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
        orig_url text primary key,
        short_url text)",
        (),
    )?;

    Ok(())
}

pub(crate) fn new(orig_url: String) -> Result<String> {
    let short_url = ShortUrl::new(SHORT_URL_LEN);
    let url = Url {
        orig_url: orig_url.clone(),
        short_url,
    };

    let conn = Connection::open("aqlink.db")?;

    let mut stmt = conn.prepare("SELECT short_url FROM urls WHERE orig_url=:orig_url LIMIT 1")?;
    let urls_iter = stmt.query_map(&[(":orig_url", orig_url.as_str())], |row| {
        Ok(Url {
            short_url: ShortUrl::default(),
            orig_url: "".to_string(),
        })
    })?;

    for u in urls_iter {
        return Ok(u.unwrap().orig_url);
    }

    conn.execute(
        "INSERT INTO urls (orig_url, short_url) VALUES (?1, ?2)",
        (&url.orig_url, &url.short_url.to_string()),
    )?;

    Ok(url.short_url.to_string())
}
