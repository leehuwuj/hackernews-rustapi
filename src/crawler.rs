use std::sync::Arc;
use crate::hub::NewsHub;
use crate::item::Item;

pub struct ItemsCrawler<StoreClient> {
    pub hub: Arc<NewsHub>,
    pub client: StoreClient,
}

// Traits

/// Storing item data into generic Store
pub trait GenericStoreItem<T> {
    fn get_last_item(&mut self) -> Result<i64, ()>;
    fn store_item(&mut self, item: Item);
}
/// Generic crawl logic
pub trait GenericCrawlerFlow<T> {
    fn run_one(&mut self) -> Result<String, ()>;
}

// Default implementations

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
        Ok(res)
    }
}

impl<T> GenericCrawlerFlow<T> for ItemsCrawler<T>
    where T: GenericStoreItem<T> {
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

#[cfg(test)]
mod tests {
    use crate::crawler::ItemsCrawler;
    use crate::hub::NewsHub;
    use crate::store::Store;


    fn mock_crawler() -> ItemsCrawler<postgres::Client> {
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let url = format!("postgresql://{}:{}@{}:{}/{}",
                          "hackernews",
                          "hackernews",
                          "localhost",
                          5432,
                          "hackernews");
        let store_client  = postgres::Client::connect(
            &*url,
            postgres::NoTls).unwrap();
        ItemsCrawler::new(hub, store_client)
    }

    // Test crawler with Hubs
    #[test]
    fn test_crawler_get_item() {
        let mut item_crawler = mock_crawler();
        let res = item_crawler.fetch_latest_item().unwrap();
        assert!(res > 0)
    }

    #[test]
    fn test_fetch_item_info() {
        let mut item_crawler = mock_crawler();
        assert!(item_crawler.fetch_item(34103778).unwrap() > format!(""))
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
    }
}