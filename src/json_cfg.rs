use serde::Deserialize;
use simd_json::from_reader;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub logo_generator: String,
    pub post_title: bool,
    pub post_image: bool,
    pub post_url: bool,
    pub log_posts: bool,
    pub log_file: String,
    pub twitter_send_tweet: bool,
    pub twitter_consumer_key: String,
    pub twiter_consumer_secret: String,
    pub twitter_access_token: String,
    pub twitter_access_token_secret: String,
    pub mastodon_send_toot: bool,
    pub mastodon_access_token: String,
    pub mastodon_api_base_url: String,
    pub mastodon_username: String,
    pub mastodon_password: String
}

/// Return Config struct parsed from config file.
pub fn get_config(path: &str) -> Result<Config, std::io::Error>
{
    let reader = get_file_handle(path)?;
    let cfg = from_reader(reader)?;
    return Ok(cfg);
}

/// Get a buffered reader for the config file.
fn get_file_handle(path: &str) -> Result<std::io::BufReader<std::fs::File>, std::io::Error>
{
    let file = File::open(path)?;
    return Ok(BufReader::new(file));
}