extern crate redis;
use redis::Commands;

pub fn get_url_from_redis(key: String) -> redis::RedisResult<String> {
    let client = redis::Client::open(get_redis_url())?;
    let mut con = client.get_connection()?;

    let url: String = con.get(key)?;

    Ok(url)
}

pub fn set_url_in_redis(key: String, url: String) -> redis::RedisResult<String> {
    let client = redis::Client::open(get_redis_url())?;
    let mut con = client.get_connection()?;

    let result = con.set(key, url)?;

    Ok(result)
}

// Function to get REDIS_URL from environment variable
pub fn get_redis_url() -> String {
    let redis_url_var = std::env::var("REDIS_URL");
    let redis_url = match redis_url_var {
        Ok(url) => Some(url),
        Err(_) => None
    };

    redis_url.unwrap().to_string()
}