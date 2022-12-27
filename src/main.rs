// #![allow(dead_code, unused_imports, unused_variables, unused_qualifications)]

use clap::Parser;
use crate::crawler::{GenericCrawlerFlow, ItemsCrawler};
use crate::hub::NewsHub;
use crate::store::Store;
use crate::utils::CRAWLER_HUB;

pub mod store;
pub mod item;
pub mod hub;
pub mod crawler;
pub mod ext;
mod utils;

#[derive(Parser,Default,Debug)]
struct Args {
    #[clap(short, long, default_value_t = String::from("sqlite"))]
    store: String,
    #[clap(long)]
    store_uri: String,
    #[clap(short, long, default_value_t=5)]
    n_items: u16,
    #[clap(long, default_value_t = String::from("run_one"))]
    run_type: String
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let hub = NewsHub::new(&CRAWLER_HUB);
    match args.store.as_str() {
        "sqlite" => {
            let store = Store::<sqlite::Connection>::new(args.store_uri.trim());
            let mut crawler = ItemsCrawler::new(hub, store);
            match args.run_type.as_str() {
                "run_one" => {
                    crawler.run_one().unwrap();
                }
                "run_many" => {
                   crawler.run_many();
                }
                "sync_data" => {
                    crawler.run_sync_data();
                }
                &_ => {
                    panic!("`run_type` {:?} is not supported for sqlite!", args.run_type);
                }
            }
        }
        "postgres" => {
            let store = Store::<postgres::Client>::new(&args.store_uri);
            let mut crawler = ItemsCrawler::new(hub, store);
            match args.run_type.as_str() {
                "run_one" => {
                    crawler.run_one().unwrap();
                }
                &_ => {
                    panic!("`run_type` {:?} is not supported for `postgres`! ", args.run_type);
                }
            }
        }
        &_ => {
            panic!("`store` {:?} is not supported!", args.store)
        }
    };
}