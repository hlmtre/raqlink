#[macro_use]
extern crate rocket;
mod url;

use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::response::Redirect;

const HOST_PREFIX: &str = "https://u.aql.ink/";
//const IMG_HOST_PREFIX: &str = "https://u.aql.ink/i/";
const IMG_HOST_PREFIX: &str = "http://localhost:8193/i/";

#[derive(FromForm, Debug)]
pub(crate) struct Upload<'r> {
    image: TempFile<'r>,
}

#[post("/new", data = "<orig_url>")]
fn new(orig_url: String) -> String {
    HOST_PREFIX.to_string() + url::new(orig_url).unwrap().as_str()
}

#[post("/new_image", data = "<upload>")]
async fn new_image(upload: Form<Upload<'_>>) -> String {
    //eprintln!("form: {:?}", upload);
    IMG_HOST_PREFIX.to_owned() + &url::new_img(upload).await.unwrap()
}

/*
  IF YOU WANT IT TO REDIRECT *TO* SOMEWHERE, THE FUNCTION'S RETURN TYPE
  HAS TO BE A REDIRECT
*/
#[get("/i/<uuid>")]
async fn retrieve_img(uuid: String) -> Option<NamedFile> {
    NamedFile::open(std::path::Path::new(&url::retrieve_img(uuid).unwrap()))
        .await
        .ok()
}

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
    rocket::build().mount("/", routes![retrieve, retrieve_img, new, new_image, uh_oh])
}
