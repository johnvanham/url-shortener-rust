extern crate redis;
use redis::Commands;

pub fn get_url_from_redis(key: String) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let url: String = con.get(key)?;

    Ok(url)
}