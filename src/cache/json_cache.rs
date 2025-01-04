use super::cache::Cache;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::path::{self, PathBuf};
use std::time::{Duration, SystemTime};

pub struct JsonCache<K> {
    pub time_to_live: Duration,
    pub base_path: path::PathBuf,
    pub get_file_path: Box<dyn Fn(&K) -> path::PathBuf + Send + Sync>,
}

impl<K, V: DeserializeOwned + Serialize> Cache<K, V> for JsonCache<K> {
    fn get(&self, key: &K) -> Option<V> {
        let sub_path = (self.get_file_path)(key);
        let file_path = self.base_path.join(sub_path);
        if self.is_cached(&file_path) {
            let contents = std::fs::read_to_string(file_path).expect("Unable to read file");
            Some(serde_json::from_str(&contents).expect("failed to parse json"))
        } else {
            None
        }
    }

    fn put(&self, key: &K, value: &V) {
        let sub_path = (self.get_file_path)(key);
        let file_path = self.base_path.join(sub_path);

        if file_path.is_dir() {
            std::fs::create_dir_all(file_path).expect("Unable to create directory");
        } else {
            std::fs::write(file_path, serde_json::to_string(value).unwrap())
                .expect("Unable to write file");
        }
    }
}

impl<K> JsonCache<K> {
    fn is_cached(&self, file_path: &PathBuf) -> bool {
        if file_path.is_file() {
            let metadata = std::fs::metadata(file_path).expect("Failed to read file metadata");
            let last_modified = metadata.modified().unwrap();
            let expired =
                SystemTime::now().duration_since(last_modified).unwrap() > self.time_to_live;
            return !expired;
        }
        return false;
    }
}
