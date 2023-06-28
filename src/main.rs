#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "URL Shortener"
}

#[get("/<url>")]
fn shorten(url: String) -> String {
    format!("Shortened URL: {}", url)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![shorten])
}