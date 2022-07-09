use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

type StorageMap = HashMap<String, i64>;

#[derive(Debug)]
pub struct Storage {
    items: StorageMap,
    file_path: String,
}

#[allow(dead_code)]
impl Storage {
    pub fn new(file_path: String) -> Self {
        if !Path::new(&file_path).exists() {
            let items = StorageMap::new();

            let storage_str = serde_json::to_string(&items).unwrap();

            let mut file = File::create(&file_path).unwrap();

            file.write_all(storage_str.as_bytes()).unwrap();

            return Storage { items, file_path };
        }

        let file_content = read_to_string(&file_path).unwrap();

        let items: StorageMap = serde_json::from_str(&file_content).unwrap();

        Storage { items, file_path }
    }

    /// The storage map kept in memory is dumped into the file specified at initialization
    /// of this structure.
    pub fn dump_data(&self) {
        let storage_str = serde_json::to_string(&self.items).unwrap();

        let mut file = File::create(&self.file_path).unwrap();

        file.write_all(storage_str.as_bytes()).unwrap();
    }

    /// Checks if the key passed in exists in the storage map.
    pub fn contains(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Adds a key-value pair to the storage map.
    /// If the entry already exists, the value is overwritten.
    pub fn add(&mut self, key: &str, value: i64) {
        self.items.insert(key.to_string(), value);
    }

    /// Removes a key-value pair from the storage map.
    /// If the entry doesn't exist, nothing is changed.
    pub fn remove(&mut self, key: &str) {
        self.items.remove(key);
    }

    /// Simply get all the items from the storage.
    pub fn get_all(&self) -> StorageMap {
        self.items.clone()
    }
}
