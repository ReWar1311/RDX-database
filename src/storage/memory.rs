use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Db {
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl Db {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn set(&self, key: String, value: String) {
        self.inner.write().await.insert(key, value);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.inner.read().await.get(key).cloned()
    }
    pub async fn del(&self, key: &str) {
        self.inner.write().await.remove(key);
    }
    pub async fn exists(&self, key: &str) -> bool {
        self.inner.read().await.contains_key(key)
    }
    pub async fn append(&self, key: &str, value: String) {
        self.inner.write().await.entry(key.to_string()).or_default().push_str(&value);
    }
    pub async fn mget(&self, keys: &[String]) -> Vec<Option<String>> {
        let db = self.inner.read().await;
        keys.iter().map(|key| db.get(key).cloned()).collect()
    }
    pub async fn rename(&self, old_key: &str, new_key: &str) -> bool {
        let mut db = self.inner.write().await;
        if let Some(value) = db.remove(old_key) {
            db.insert(new_key.to_string(), value);
            true
        } else {
            false
        }
    }
}
