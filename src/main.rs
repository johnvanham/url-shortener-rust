#[macro_use] extern crate rocket;
use rocket::{response::{Redirect, status::NotFound}, fs::NamedFile, form::Form, State};
use log::{info, error, warn};
mod db_handler;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("public/index.html").await.ok()
}

#[get("/style.css")]
async fn style() -> Option<NamedFile> {
    NamedFile::open("public/style.css").await.ok()
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
    
    let form_data = url_to_shorten.into_inner();
    info!("Shortening URL - Key: {}, URL: {}", url_key, form_data.url);

    let db_result = db_handler::set_url_in_db(url_key.clone(), form_data.url);

    match db_result {
        Ok(r) => {
            info!("Successfully shortened URL for key: {}", url_key);
            format!("URL shortened successfully: {}", r)
        },
        Err(e) => {
            error!("Error shortening URL for key {}: {}", url_key, e);
            format!("Error shortening URL: {}", e)
        }
    }
}

#[get("/<url_key>")]
fn short_url_redirect(url_key: String) -> Result<Redirect, NotFound<String>> {
    let key: String = format!("surl_{}", url_key);
    
    info!("Redirecting request for key: {}", key);

    let db_result = db_handler::get_url_from_db(key.clone());

    match db_result {
        Ok(url) => {
            info!("Redirecting {} to {}", key, url);
            Ok(Redirect::to(url))
        },
        Err(_) => {
            warn!("URL not found for key: {}", url_key);
            Err(NotFound(format!("URL not found for key: {}", url_key)))
        }
    }
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    
    info!("Starting URL Shortener application");
    
    if let Err(e) = db_handler::init_database() {
        error!("Failed to initialize database: {}", e);
        panic!("Database initialization failed");
    }
    
    info!("Application started successfully");
    
    rocket::build()
        .mount("/", routes![index, style, shorten, short_url_redirect])
}