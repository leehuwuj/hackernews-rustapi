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

impl Item {
    /// Parsed all item values into `()` sql query string
    /// followed the order of item attributes
    /// but not contains the INSERT INO... prefix
    /// the string will safty wrapped in single quote
    /// 
    /// ex:\n
    /// 
    ///({id}, {deleted}, '{item_type}', '{who}', {time}, {dead}, '{kids:?}',
    /// '{title}', '{content}', {score}, '{url}', {parent})",
    pub fn to_sql_value(&self) -> String {
        // safety process single quote
        let fmt_str = |the_str: String| the_str.replace("'", "''");
        // convert bool into number
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

impl ToString for Item {
    fn to_string(&self) -> String {
        self.to_sql_value()
    }
}