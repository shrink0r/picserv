
use std::collections::HashMap;

#[derive(Debug)]
pub enum ConfigVal {
    String(String),
    Integer(i32),
    Boolean(bool)
}

pub struct Config {
    config: HashMap<&'static str, ConfigVal>
}

impl Config {
    pub fn new() -> Config {
        let mut config: HashMap<&'static str, ConfigVal> = HashMap::new();
        config.insert("base_dir", ConfigVal::String("/Users/tschmitt/Downloads/.picserv/".to_string()));
        config.insert("enable_cache", ConfigVal::Boolean(false));
        config.insert("retry_limit", ConfigVal::Integer(23));

        Config { config: config }
    }

    pub fn get(&self, key: &'static str) -> &ConfigVal {
        self.config.get(key).unwrap()
    }
}
