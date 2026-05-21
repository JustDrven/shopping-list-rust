use std::collections::HashMap;
use std::path::Path;

use envfile::EnvFile;

use crate::application;

async fn get_environments(path: &Path) -> EnvFile {
    EnvFile::new(path).unwrap()
}

pub async fn load() -> HashMap<String, String> {
    let path: &Path = Path::new(application::util::constants::ENV_FILE_NAME);
    let mut to_return: HashMap<String, String> = HashMap::new();

    if path.exists() {
        return to_return;
    }

    let environment: EnvFile = get_environments(path).await;
    for (x, y) in &environment.store {
        let key: String = x.to_string();
        let value: String = y.to_string();

        to_return.insert(key, value);
    }

    to_return

}



