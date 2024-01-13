#[macro_use]
extern crate rocket;
mod url;

use rocket::response::Redirect;
use rusqlite::Error;

const HOST_PREFIX: &str = "http://192.168.6.151:8192/";

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/new", data = "<orig_url>")]
fn new(orig_url: String) -> String {
    HOST_PREFIX.to_string() + url::new(orig_url).unwrap().as_str()
}

#[get("/<short_url>")]
fn retrieve(short_url: String) {
    Redirect::to(url::retrieve(short_url).unwrap());
}

#[get("/error")]
fn uh_oh() -> String {
    format!("{}", "500")
}

#[launch]
fn rocket() -> _ {
    let f = url::create_tables();
    match f {
        Ok(_s) => _s,
        Err(e) => {
            rocket::build().mount("/", routes![uh_oh]);
            Redirect::to("/error");
        }
    };
    rocket::build().mount("/", routes![hello, retrieve, new, uh_oh])
}
