use envfile::EnvFile;
use std::path::Path;

pub struct Config {}

pub trait IConfig {
    fn get_config_with_key(&self, key: &str) -> Option<String>;
}

impl IConfig for Config {
    fn get_config_with_key(&self, key: &str) -> Option<String> {
        let env = EnvFile::new(&Path::new("src/config/config.env")).unwrap();
        // let new_key = key.clone();
        match env.get(key) {
            Some(val) => Some(val.to_string()),
            None => None,
        }
    }
}
