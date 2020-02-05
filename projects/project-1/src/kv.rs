use std::collections::HashMap;
use std::string::String;
///kv
pub struct KvStore {
    // Option<&String>
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    ///get
    ///&str string literal    String -> keep in heap based on vector
    pub fn get(&self, key: String) -> Option<String> {
        // cloned : Option<&T> -> Option<T>
        self.map.get(&key).cloned()
    }
    ///set
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    ///remove
    pub fn remove(&mut self, key: String) -> bool {
        let _v: Option<String> = self.map.remove(&key);
        match _v {
            Some(_) => true,
            None => false,
        }
    }
}
