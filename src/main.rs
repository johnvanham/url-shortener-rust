#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "URL Shortener"
}

#[get("/<url_key>")]
fn short_url_redirect(url_key: String) -> String {
    format!("Shortened URL: {}", url_key)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![short_url_redirect])
}