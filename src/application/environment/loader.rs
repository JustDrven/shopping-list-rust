use std::collections::HashMap;
use std::path::Path;
use envfile::EnvFile;

use crate::application;

pub async fn load() -> HashMap<String, String> {
    let path = Path::new(application::util::constants::ENV_FILE_NAME);
    let environment = EnvFile::new(&Path::new(path)).unwrap();
    let mut to_return: HashMap<String, String> = HashMap::new();

    for (key, value) in &environment.store {
    to_return.insert(key.to_string(), value.to_string());
    }

    to_return
}