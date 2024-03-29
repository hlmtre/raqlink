#[macro_use]
extern crate rocket;
mod url;

use rocket::response::Redirect;

const HOST_PREFIX: &str = "https://u.aql.ink/";

#[post("/new", data = "<orig_url>")]
fn new(orig_url: String) -> String {
    HOST_PREFIX.to_string() + url::new(orig_url).unwrap().as_str()
}

/*
  IF YOU WANT IT TO REDIRECT *TO* SOMEWHERE, THE FUNCTION'S RETURN TYPE
  HAS TO BE A REDIRECT
*/
#[get("/<short_url>")]
fn retrieve(short_url: String) -> Redirect {
    Redirect::to(url::retrieve(short_url).unwrap())
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
        Err(_e) => {
            let _ = rocket::build().mount("/", routes![uh_oh]);
            Redirect::to("/error");
        }
    };
    rocket::build().mount("/", routes![retrieve, new, uh_oh])
}
