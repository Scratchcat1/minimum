use super::cache::Cache;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::path;

pub struct FileCache<K> {
    pub base_path: path::PathBuf,
    pub get_file_path: Box<dyn Fn(&K) -> path::PathBuf + Send + Sync>,
}

impl<K, V: DeserializeOwned + Serialize> Cache<K, V> for FileCache<K> {
    fn get(&self, key: &K) -> Option<V> {
        let sub_path = (self.get_file_path)(key);
        let file_path = self.base_path.join(sub_path);
        if file_path.is_file() {
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
        } else if !file_path.is_file() {
            std::fs::write(file_path, serde_json::to_string(value).unwrap())
                .expect("Unable to write file");
        }
    }
}
