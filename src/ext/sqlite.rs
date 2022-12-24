use sqlite::{Connection, State};

use crate::crawler::{GenericStoreItem, ItemsCrawler};
use crate::item::Item;
use crate::store::Store;

/// Init needed db which store items data
fn init_items_table(conn: &Connection) {
    let query = r###"
            CREATE TABLE IF NOT EXISTS `items` (
              `id` int(10) NOT NULL,
              `deleted` tinyint(4) DEFAULT '0',
              `type` varchar(16) DEFAULT NULL,
              `who` varchar(255) DEFAULT NULL,
              `time` int(11) DEFAULT NULL,
              `dead` tinyint(4) DEFAULT '0',
              `kids` text DEFAULT NULL,
              `title` text DEFAULT NULL,
              `content` text DEFAULT NULL,
              `score` int(10) DEFAULT NULL,
              `url` text DEFAULT NULL,
              `parent` int(10) DEFAULT NULL
            )"###;
    conn.execute(query).unwrap()
}

// Implement Store for postgres
impl Store<Connection> {
    pub fn new(uri: &str) -> Self {
        let conn = Connection::open(uri).unwrap();
        init_items_table(&conn);
        Self {
            backend_client: conn
        }
    }
}

// Implement item data to Postgres
impl GenericStoreItem<Store<Connection>> for Store<Connection> {
    fn get_last_item(&mut self) -> Result<i64, ()> {
        let sql = "select max(id) as max_id from items";

        let mut statement = self.backend_client
            .prepare(sql)
            .unwrap();
        match statement.next().unwrap() {
            State::Row => {
                let max_id = statement.read::<i64, _>("max_id").unwrap();
                return Ok(max_id)
            }
            State::Done => {Err(())}
        }
    }
    fn store_item(&mut self, item: Item) {
        let sql = format!("INSERT INTO `items` VALUES {}", item.to_string());
        self.backend_client.execute(sql).unwrap();
    }
}

impl Store<Connection> {
    fn store_items_batch(&mut self, items: Vec<Item>) {
        let batched_values = items.iter()
            .fold("".to_string(),
                |i1, i2|
                    format!("{},{}", i1.to_string(), i2.to_string()))
            .split_at(1).1.to_string();
        let sql = format!("INSERT INTO `items` VALUES {}", batched_values.to_string());
        self.backend_client.execute(sql).unwrap();
    }
}

// This implementation will override the GenericCrawlerFlow above
impl ItemsCrawler<Store<Connection>> {
    /// Run the crawler which get latest item in Hub and store to Postgres
    pub fn run_one(&mut self) -> Result<String, ()> {
        let latest_item_id = self.fetch_latest_item().unwrap();
        let last_item_id = self.client.get_last_item().unwrap();
        if latest_item_id > last_item_id {
            let res = self.fetch_item(latest_item_id).unwrap();
            let item  = Item::from(res.to_string());
            self.client.store_item(item);
        }

        println!("Execute in SQLite");
        println!("Last item: \t\t{:?}\nLatest item: \t{:?}", last_item_id, latest_item_id);
        Ok("Generic!".to_string())
    }

    pub fn run_many(&mut self) {
        let latest_item_id = self.fetch_latest_item().unwrap();
        let mut last_item_id = self.client.get_last_item().unwrap();
        let mut counter = 0;
        while (latest_item_id > last_item_id) && counter < 5 {
            let res = self.fetch_item(last_item_id).unwrap();
            let item  = Item::from(res.to_string());
            self.client.store_item(item);
            last_item_id += 1;
            counter += 1;
        }
        println!("Inserted {counter} items!")
    }

    /// Fetch items synchronously and insert into Sqlite in a batched of all items
    pub fn run_many_insert_batch(&mut self) {
        let latest_item_id = self.fetch_latest_item().unwrap();
        let mut last_item_id = self.client.get_last_item().unwrap();
        let mut counter = 0;
        let mut items: Vec<Item> = vec![];
        while (latest_item_id > last_item_id) && counter < 2 {
            let res = self.fetch_item(last_item_id).unwrap();
            let item  = Item::from(res.to_string());
            items.push(item);
            last_item_id += 1;
            counter += 1;
        }
        self.client.store_items_batch(items);
        println!("Inserted {counter} items!")
    }
}

#[cfg(test)]
mod tests {
    use sqlite::{Connection, State};
    use crate::crawler::{GenericStoreItem, ItemsCrawler};
    use crate::hub::NewsHub;
    use crate::store::Store;

    fn mock_sqlite_memory() -> Store<Connection> {
        let url = format!(":memory:");
        Store::<Connection>::new(&url)
    }

    fn mock_tmp_db() -> Store<Connection> {
        let url = format!("items.db");
        Store::<Connection>::new(&url)
    }

    #[test]
    fn test_run_one() {
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let store_client = mock_tmp_db();
        let mut crawler = ItemsCrawler::new(hub, store_client);
        let _ = crawler.run_one();
    }

    #[test]
    fn test_run_many() {
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let store_client = mock_tmp_db();
        let mut crawler = ItemsCrawler::new(hub, store_client);
        let _ = crawler.run_many();
    }

    #[test]
    fn test_run_many_insert_batch() {
        let hub = NewsHub::new("https://hacker-news.firebaseio.com/v0/");
        let store_client = mock_tmp_db();
        let mut crawler = ItemsCrawler::new(hub, store_client);
        let _ = crawler.run_many_insert_batch();

    }
    #[test]
    fn test_init_sql_store() {
        let store_client = mock_sqlite_memory();
        let query =
            "SELECT count(1) \
            as cnt FROM sqlite_master \
            where \
                type='table' \
                and name='items'";
        let mut statement =store_client.backend_client.prepare(query).unwrap();
        let mut is_existed: bool = false;
        match statement.next().unwrap() {
            State::Row => {
                is_existed = statement.read::<i64, _>("cnt").unwrap() > 0;
            }
            State::Done => {}
        }
        assert!(is_existed);
    }

    #[test]
    fn test_init_tmp_store() {
        let store_client = mock_tmp_db();
        let query =
            "SELECT count(1) \
            as cnt FROM sqlite_master \
            where \
                type='table' \
                and name='items'";
        let mut statement =store_client.backend_client.prepare(query).unwrap();
        let mut is_existed: bool = false;
        match statement.next().unwrap() {
            State::Row => {
                is_existed = statement.read::<i64, _>("cnt").unwrap() > 0;
            }
            State::Done => {}
        }
        assert!(is_existed);
    }
}

