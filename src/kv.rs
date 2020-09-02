use std::collections::HashMap;
use std::path::PathBuf;

use crate::{KvsError, Result};

/// The `KvStore` stores key/value pairs.
///
/// Data is stored in memory through a HashMap.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default, Debug)]
pub struct KvStore {
    db: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        KvStore { db: HashMap::new() }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore { db: HashMap::new() })
    }



    /// Insert a key/value pair.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key, value);
        Ok(())
    }

    /// Get a value for a given key.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.db.get(&key).cloned())
    }

    /// Remove a key/value for a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.db.remove(&key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
