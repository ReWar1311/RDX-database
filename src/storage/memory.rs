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
}
