use std::sync::Arc;
use crate::hub::NewsHub;
use crate::item::Item;

pub struct ItemsCrawler<StoreClient> {
    pub hub: Arc<NewsHub>,
    pub client: StoreClient,
}

// Default Traits
/// Storing item data into generic Store
pub trait GenericStoreItem<T> {
    fn get_last_item(&mut self) -> Result<i64, ()>;
    fn store_item(&mut self, item: Item) -> Result<bool, ()>;
}
/// Generic crawl logic
pub trait GenericCrawlerFlow<T> {
    fn run_one(&mut self) -> Result<(), ()>;
}

// Default Implementations
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
    pub fn fetch_item(&mut self, item_id: i64) -> Result<Item, ()> {
        let res = self.hub.fetch_item(item_id).unwrap();
        Ok(Item::from(res))
    }
}

impl<T> GenericCrawlerFlow<T> for ItemsCrawler<T>
    where T: GenericStoreItem<T> {
    /// Run the crawler which get latest item in Hub and store to the store
    fn run_one(&mut self) -> Result<(), ()> {
        println!("Execute in Generic");
        let latest_item_id = self.fetch_latest_item().unwrap();
        let last_item_id = self.client.get_last_item().unwrap();
        if latest_item_id > last_item_id {
            let item = self.fetch_item(latest_item_id).unwrap();
            println!("{:?}", item.to_string());
            self.client.store_item(item).unwrap();
        }
        println!("Store 1 item into store!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::crawler::{GenericCrawlerFlow, ItemsCrawler};
    use crate::hub::NewsHub;
    use crate::store::Store;
    use crate::utils::CRAWLER_HUB;

    /// Create crawler for item which using sqlite memory as storage
    fn mock_crawler() -> ItemsCrawler<Store<sqlite::Connection>> {
        let hub = NewsHub::new(&**CRAWLER_HUB);
        let store = Store::<sqlite::Connection>::new(":memory:");
        ItemsCrawler::new(hub, store)
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
        item_crawler.fetch_item(34103778).unwrap();
        assert!(true);
    }


    // Test crawler flow
    #[test]
    fn test_run_one() {
        let mut crawler = mock_crawler();
        let _ = crawler.run_one();
    }
}