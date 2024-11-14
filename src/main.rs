#[macro_use]
extern crate rocket;
mod url;

use rocket::form::Form;
use rocket::response::Redirect;

const HOST_PREFIX: &str = "https://u.aql.ink/";
const IMG_HOST_PREFIX: &str = "https://i.aql.ink/";

#[derive(FromForm)]
pub(crate) struct Upload {
    data: Option<Vec<u8>>,
}

#[post("/new", data = "<orig_url>")]
fn new(orig_url: String) -> String {
    HOST_PREFIX.to_string() + url::new(orig_url).unwrap().as_str()
}

#[post("/new_image", data = "<upload>")]
fn new_image(upload: Form<Upload>) -> String {
    IMG_HOST_PREFIX.to_string() + url::new_img(upload).unwrap().to_string().as_str()
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
    "500".to_string()
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
    rocket::build().mount("/", routes![retrieve, new, new_image, uh_oh])
}
