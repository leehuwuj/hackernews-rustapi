use std::env::var;
use std::env::VarError;
use lazy_static::lazy_static;

fn env_or(name: &str, default: String) -> String {
    match var(name) {
        Ok(v) => {v}
        Err(_) => {default}
    }
}

lazy_static! {
    pub static ref CRAWLER_HUB: String = env_or(
        "CRAWLER_HUB",
        format!("https://hacker-news.firebaseio.com/v0/")
    );
    pub static ref MAX_BATCH_ITEMS: u16 = env_or(
        "CRAWLER_HUB",
        format!("10")
    ).parse::<u16>().unwrap();
}