
use std::collections::HashMap;

#[derive(Debug)]
pub enum ConfigVal {
    String(String),
    Integer(i32),
    Boolean(bool)
}

pub struct Config<'a> {
    config: HashMap<&'a str, ConfigVal>
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        let mut config: HashMap<&str, ConfigVal> = HashMap::new();
        config.insert("base_dir", ConfigVal::String("/Users/tschmitt/Downloads/.picserv/".to_string()));
        config.insert("enable_cache", ConfigVal::Boolean(false));
        config.insert("retry_limit", ConfigVal::Integer(23));

        Config { config: config }
    }

    pub fn get(&self, key: &str) -> &ConfigVal {
        self.config.get(key).unwrap()
    }
}
