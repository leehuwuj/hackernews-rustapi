use std::sync::{Arc, mpsc};
use std::time::Duration;
use crate::hub::NewsHub;
use crate::item::Item;
use crate::utils::MAX_BATCH_ITEMS;
use tokio::runtime;

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
    fn run_sync_data(&mut self);
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
    pub fn fetch_items_async(&self, ids: Vec<i64>, rt: tokio::runtime::Runtime) -> Vec<Item> {
        let (s, r) = mpsc::channel();
        let ids_cnt = ids.len();
        
        for i in ids {
            let sender = s.clone();
            let hub = self.hub.clone();
            rt.spawn(async move {
                match hub.fetch_item_async(i).await {
                    Ok(response) => {
                        if response.len() < 10 {
                            println!("invalid response");
                            let _ = sender.send(None);
                        } else {
                            let item = Item::from(response);
                            let _ = sender.send(Some(item));
                        }
                    }
                    Err(_) => {
                        let _ = sender.send(None);
                    }
                }
            });
        }

        let mut res = vec![];
        for _ in 0..ids_cnt {
            match r.recv_timeout(Duration::from_secs(5)) {
                Ok(item) => {
                    if let Some(item) = item {
                        res.push(item);
                    }
                }
                Err(_) => continue 
            }
        }

        res
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

    fn run_sync_data(&mut self) {
        let latest_item = self.fetch_latest_item().unwrap();
        let last_item = self.client.get_last_item().unwrap();
        let max_item_per_batch = *MAX_BATCH_ITEMS as i64;
        let mut max_item_id = last_item;
        while latest_item > max_item_id {
            // Create single async runtime
            let rt = runtime::Builder::new_multi_thread()
                                .worker_threads(1)
                                .enable_all()
                                .build()
                                .unwrap();
            let to_item_id = std::cmp::min(
                max_item_id + max_item_per_batch, 
                latest_item);
            let batched: Vec<i64> = (max_item_id+1..to_item_id+1).into_iter().collect();
            let res = self.fetch_items_async(batched, rt);
            max_item_id = to_item_id;
            res.into_iter().for_each(|i| {self.client.store_item(i).unwrap();});
            // res.into_iter().for_each(|i| println!("Item: {}", i.id));
            println!("max item: {}", max_item_id);
        }
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
        let store = Store::<sqlite::Connection>::new("resources/items.db");
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

    #[test]
    fn test_run_async() {
        let mut crawler = mock_crawler();
        let _ = crawler.run_sync_data(); 
    }
}