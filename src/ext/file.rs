use crate::crawler::GenericStoreItem;
use crate::item::Item;
use crate::store::{Store};

/// Wrap File into FileClient
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

// Implement the Store with FileClient
impl Store<FileClient> {
    pub fn new(store_path: &str) -> Self {
        Self {
            backend_client: FileClient::new(store_path.to_string())
        }
    }
}
// Implement trait StoreItem
impl GenericStoreItem<Store<FileClient>> for Store<FileClient> {
    fn get_last_item(&mut self) -> Result<i64, ()> {
        Ok(34103931i64)
    }
    fn store_item(&mut self, item: Item) {
        let _ = self.backend_client.base_path;
        let _ = item;
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::crawler::{GenericStoreItem, ItemsCrawler};
    use crate::ext::file::FileClient;
    use crate::hub::NewsHub;
    use crate::store::Store;
    use crate::utils::CRAWLER_HUB;

    #[test]
    fn test_get_item_file() {
        let hub = NewsHub::new(&**CRAWLER_HUB);
        let store_client  = Store::<FileClient>::new("/tmp/test");
        let mut crawler = ItemsCrawler::new(hub, store_client);
        let item = crawler.client.get_last_item();
        println!("{:?}", item);
    }
}