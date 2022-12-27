use postgres::NoTls;
use crate::crawler::{ItemsCrawler, GenericStoreItem};
use crate::item::Item;
use crate::store::Store;

// Implement Store for postgres
impl Store<postgres::Client> {
    pub fn new(uri: &str) -> Self {
        Self {
            backend_client: postgres::Client::connect(uri, NoTls).unwrap()
        }
    }
}

// Implement item data to Postgres
impl GenericStoreItem<Store<postgres::Client>> for Store<postgres::Client> {
    fn get_last_item(&mut self) -> Result<i64, ()> {
        println!("Getting last item from store...");
        let row = self.backend_client.query_one("select max(id) from items", &[]);
        match row {
            Ok(row) => Ok(row.get(0)),
            Err(_) => Err(())
        }
    }
    fn store_item(&mut self, item: Item) -> Result<bool, ()> {
        let _ = item;
        todo!()
    }
}

// This implementation will override the GenericCrawlerFlow above
impl ItemsCrawler<Store<postgres::Client>> {
    /// Run the crawler which get latest item in Hub and store to Postgres
    pub fn run_one(&mut self) -> Result<String, ()> {
        let latest_item_id = self.fetch_latest_item().unwrap();
        let last_item_id = self.client.get_last_item();
        println!("Execute in postgres");
        println!("Last item: \t\t{:?}\nLatest item: \t{:?}", last_item_id, latest_item_id);
        Ok("Generic!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::crawler::{GenericStoreItem, ItemsCrawler};
    use crate::hub::NewsHub;
    use crate::store::Store;
    use crate::utils::CRAWLER_HUB;

    #[test]
    fn test_get_item_postgres() {
        // let mut crawler = mock_crawler();
        let hub = NewsHub::new(&**CRAWLER_HUB);
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
    }
}