extern crate rusqlite;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Url {
    id: u32,
    orig_url: String,
    short_url: String,
}

fn create() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
        orig_url text,
        short_url text,
        uid unsigned big int
    )",
        (),
    )?;
    let url = Url {
        id: 1,
        orig_url: "https://lifehacker.com/some/path",
        short_url: "https://u.aql.ink/1q2w3e4r",
    };

    conn.execute(
        "INSERT INTO urls (orig_url, short_url) VALUES (?1, ?2)",
        (&url.orig_url, &url.short_url),
    )?;
}
