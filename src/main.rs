#![allow(dead_code, unused_imports, unused_variables, unused_qualifications)]

extern crate core;

/// Represent for Hackernews data
mod item {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Item {
        pub id: i64,
        #[serde(default)]
        pub deleted: bool,
        #[serde(rename = "type")]
        pub tp: String,
        #[serde(rename = "by")]
        #[serde(default)]
        pub who: String,
        pub time: i64,
        #[serde(default)]
        pub dead: bool,
        #[serde(default)]
        pub kids: Vec<i64>,
        #[serde(default)]
        pub title: String,
        #[serde(default)]
        pub score: i64,
        #[serde(default)]
        pub text: String,
        #[serde(default)]
        pub url: String,
        #[serde(default)]
        pub parent: i64,
    }

    impl From<String> for Item {
        fn from(s: String) -> Self {
            let p: Item = serde_json::from_str(&s).unwrap();
            p
        }
    }

    impl ToString for Item {
        fn to_string(&self) -> String {
            let fmt_str = |the_str: String| the_str.replace("'", "''");
            let fmt_bool = |the_bool: bool| {
                match the_bool { true => 1, _ => 0 }
            };

            format!(
                "({id}, {deleted}, '{item_type}', '{who}', {time}, {dead}, '{kids:?}', \
            '{title}', '{content}', {score}, '{url}', {parent})",
                id=self.id,
                deleted=fmt_bool(self.deleted),
                item_type=fmt_str(self.tp.to_string()),
                who=fmt_str(self.who.to_string()),
                time=self.time,
                dead=fmt_bool(self.dead),
                kids=self.kids,
                title=fmt_str(self.title.to_string()),
                content=fmt_str(self.text.to_string()),
                score=self.score,
                url=fmt_str(self.url.to_string()),
                parent=self.parent
            )
        }
    }
}

/// Integrate with Hackernews API
mod hub {
    use reqwest::IntoUrl;

    pub struct NewsHub {
        base_uri: String,
    }
    impl NewsHub {
        pub fn new(base_uri: impl Into<String>) -> Self {
            Self {
                base_uri: base_uri.into(),
            }
        }

        pub fn fetch_max_item(&self) -> Result<String, reqwest::Error> {
            self.fetch_res_by_uri("/maxitem.json?print=pretty")
        }

        pub async fn fetch_item_async(&self, item_id: i64) -> Result<String, reqwest::Error> {
            self.fetch_res_by_uri_async(
                &("/item/".to_owned() + &item_id.to_string() + ".json?print=pretty"),
            )
                .await
        }

        pub fn fetch_item(&self, item_id: i64) -> Result<String, reqwest::Error> {
            self.fetch_res_by_uri(
                &("/item/".to_owned() + &item_id.to_string() + ".json?print=pretty"),
            )
        }

        pub fn blocking_fetch<T: IntoUrl>(&self, url: T) -> Result<reqwest::blocking::Response, reqwest::Error> {
            reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap()
                .get(url)
                .send()
        }

        fn fetch_res_by_uri(&self, query: &str) -> Result<String, reqwest::Error> {
            self.blocking_fetch(&(self.base_uri.clone() + query))?.text()
        }

        async fn fetch_res_by_uri_async(&self, query: &str) -> Result<String, reqwest::Error> {
            reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build().unwrap()
                .get(&(self.base_uri.clone() + query))
                .send()
                .await?.text().await
        }
    }
}

mod store {
    use std::fs::File;
    use postgres::{Client, NoTls};

    pub struct Store<T> {
        pub backend_client: T
    }

    impl Store<postgres::Client> {
        pub fn new(uri: &str) -> Self {
            Self {
                backend_client: Client::connect(uri, NoTls).unwrap()
            }
        }
    }

    pub struct FileClient {
        base_path: String
    }
    impl FileClient {
        pub fn new(dir: String) -> Self {
            Self {
                base_path: dir
            }
        }
    }

    impl Store<FileClient> {
        pub fn new(store_path: &str) -> Self {
            Self {
                backend_client: FileClient::new(store_path.to_string())
            }
        }
    }
}


/// Define crawling flow
mod crawler {
    use std::fs::File;
    use std::os::unix::fs::FileExt;
    use log::{info, trace, warn};
    use std::sync::Arc;
    use postgres::{NoTls};
    use crate::hub::NewsHub;
    use crate::store::{FileClient, Store};

    pub struct ItemsCrawler<StoreClient> {
        hub: Arc<NewsHub>,
        client: StoreClient,
    }

    /// Default functions
    impl<StoreClient> ItemsCrawler<StoreClient> {
        pub fn new(hub: NewsHub, store_client: StoreClient) -> Self {
            Self {
                hub: Arc::new(hub),
                client: store_client
            }
        }
        pub fn fetch_latest_item(&mut self) -> Result<i64, ()> {
            let res = self.hub.fetch_max_item().unwrap();
            let latest_item_id = res.trim().parse::<i64>().unwrap();
            Ok(latest_item_id)
        }
        pub fn fetch_item(&mut self, item_id: i64) -> Result<String, ()> {
            let res = self.hub.fetch_item(item_id).unwrap();
            println!("{:?}", res);
            Ok(res)
        }
    }

    /// Storing data
    pub trait StoreItem<T> {
        fn get_last_item(&mut self) -> Result<i64, ()>;
        fn store_item(&mut self);
    }
    // Implement store data to File
    impl StoreItem<Store<FileClient>> for Store<FileClient> {
        fn get_last_item(&mut self) -> Result<i64, ()> {
            Ok(34103931i64)
        }
        fn store_item(&mut self) {
            todo!();
        }
    }
    // Implement store data to Postgres
    impl StoreItem<Store<postgres::Client>> for Store<postgres::Client> {
        fn get_last_item(&mut self) -> Result<i64, ()> {
            println!("Getting last item from store...");
            let row = self.backend_client.query_one("select max(id) from items", &[]);
            let result = match row {
                Ok(row) => Ok(row.get(0)),
                Err(_) => Err(())
            };
            result
        }
        fn store_item(&mut self) {
            todo!()
        }
    }


    /// Generic crawl logic
    pub trait GenericCrawlerFlow<T> {
        fn run_one(&mut self) -> Result<String, ()>;
    }
    impl<T> GenericCrawlerFlow<T> for ItemsCrawler<T>
    where T: StoreItem<T> {
        /// Run the crawler which get latest item in Hub and store to store
        fn run_one(&mut self) -> Result<String, ()> {
            let latest_item_id = self.fetch_latest_item().unwrap();
            let last_item_id = self.client.get_last_item().unwrap();
            if latest_item_id > last_item_id {
                let item = self.fetch_item(latest_item_id).unwrap();
                println!("{:?}", item)
                // self.client.store_item()
            }
            println!("Execute in Generic");
            println!("Last item: \t\t{:?}\nLatest item: \t{:?}", last_item_id, latest_item_id);
            Ok("Generic!".to_string())
        }
    }

    // This implementation will override the GenericCrawlerFlow above
    // impl ItemsCrawler<Store<postgres::Client>> {
    //     /// Run the crawler which get latest item in Hub and store to Postgres
    //     fn run_one(&mut self) -> Result<String, ()> {
    //         let latest_item_id = self.fetch_latest_item().unwrap();
    //         let last_item_id = self.client.get_last_item();
    //         println!("Execute in postgres");
    //         println!("Last item: \t\t{:?}\nLatest item: \t{:?}", last_item_id, latest_item_id);
    //         Ok("Generic!".to_string())
    //     }
    // }


    /// Test
    #[cfg(test)]
    fn mock_crawler() -> ItemsCrawler<postgres::Client> {
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let url = format!("postgresql://{}:{}@{}:{}/{}",
                          "hackernews",
                          "hackernews",
                          "localhost",
                          5432,
                          "hackernews");
        let store_client  = postgres::Client::connect(&*url, NoTls).unwrap();
        ItemsCrawler::new(hub, store_client)
    }

    // Test crawler with Hubs
    #[test]
    fn test_crawler_get_item() {
            let mut item_crawler = mock_crawler();
            let res = item_crawler.fetch_latest_item().unwrap();
            println!("{:?}", res);
            assert!(res > 0)
        }
    #[test]
    fn test_fetch_item_info() {
            let mut item_crawler = mock_crawler();
            assert!(item_crawler.fetch_item(34103778).unwrap() > format!(""))
        }

    // Test crawler with Store
    #[test]
    fn test_get_item_postgres() {
        // let mut crawler = mock_crawler();
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let url = format!("postgresql://{}:{}@{}:{}/{}",
                          "hackernews",
                          "hackernews",
                          "localhost",
                          5432,
                          "hackernews");
        let store_client  = Store::<postgres::Client>::new(&url);
        let mut crawler = ItemsCrawler::new(hub, store_client);
        let item = crawler.client.get_last_item();
        println!("{:?}", item);
        panic!("");
        // crawler.get
    }
    #[test]
    fn test_get_item_file() {
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let store_client  = Store::<FileClient>::new("/tmp/test");
        let mut crawler = ItemsCrawler::new(hub, store_client);
        let item = crawler.client.get_last_item();
        println!("{:?}", item);
        // crawler.get
    }

    // Test crawler flow
    #[test]
    fn test_run_one() {
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let url = format!("postgresql://{}:{}@{}:{}/{}",
                          "hackernews",
                          "hackernews",
                          "localhost",
                          5432,
                          "hackernews");
        let store_client  = Store::<postgres::Client>::new(&url);
        let mut crawler = ItemsCrawler::new(hub, store_client);
        let _ = crawler.run_one();
        // crawler.run_one();
        // crawler.run_one();
    }

}

fn main() {

}