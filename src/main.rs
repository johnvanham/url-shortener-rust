#[macro_use] extern crate rocket;
use rocket::{response::{Redirect, status::NotFound}, fs::NamedFile, form::Form};
mod redis_handler;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("public/index.html").await.ok()
}

#[derive(FromForm)]
pub struct ShortenURLFormData {
    #[field(name = "key")]
    pub key: String,

    #[field(name = "url")]
    pub url: String,
}

#[post("/shorten", data = "<url_to_shorten>")]
fn shorten(url_to_shorten: Form<ShortenURLFormData>) -> String {

    let mut url_key = String::from("surl_");
    url_key.push_str(&url_to_shorten.key);

    let redis_result: Result<String, redis::RedisError> = redis_handler::set_url_in_redis(
        url_key,
        url_to_shorten.into_inner().url
    );

    match redis_result {
        Ok(r) => {
            format!("URL shortened successfully: {}", r)
        },
        Err(e) => {
            format!("Error shortening URL: {}", e)
        }
    }
}

#[get("/<url_key>")]
fn short_url_redirect(url_key: String) -> Result<Redirect, NotFound<String>> {

    let key: String = format!("surl_{}", url_key);

    let redis_result: Result<String, redis::RedisError> = redis_handler::get_url_from_redis(key);

    match redis_result {
        Ok(url) => {
            Ok(Redirect::to(url))
        },
        Err(_) => {
            Err(NotFound(format!("URL not found for key: {}", url_key)))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, shorten, short_url_redirect])
}