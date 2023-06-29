#[macro_use] extern crate rocket;
use rocket::response::{Redirect, status::NotFound};
mod redis_handler;

#[get("/")]
fn index() -> &'static str {
    "URL Shortener"
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
        .mount("/", routes![index])
        .mount("/", routes![short_url_redirect])
}