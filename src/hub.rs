use reqwest::IntoUrl;

pub struct NewsHub {
    base_uri: String,
}
impl NewsHub {
    pub fn new(base_uri: &str) -> Self {
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
        ).await
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