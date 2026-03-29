use dashmap::DashMap;

/// Thread-safe store for open document contents.
pub struct DocumentStore {
    docs: DashMap<String, String>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self {
            docs: DashMap::new(),
        }
    }

    pub fn open(&self, uri: &str, text: String) {
        self.docs.insert(uri.to_string(), text);
    }

    pub fn update(&self, uri: &str, text: String) {
        self.docs.insert(uri.to_string(), text);
    }

    pub fn close(&self, uri: &str) {
        self.docs.remove(uri);
    }

    pub fn get(&self, uri: &str) -> Option<String> {
        self.docs.get(uri).map(|r| r.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_store_lifecycle() {
        let store = DocumentStore::new();
        store.open("file:///a.ach", "let x = 1".into());
        assert_eq!(store.get("file:///a.ach").unwrap(), "let x = 1");
        store.update("file:///a.ach", "let x = 2".into());
        assert_eq!(store.get("file:///a.ach").unwrap(), "let x = 2");
        store.close("file:///a.ach");
        assert!(store.get("file:///a.ach").is_none());
    }
}
