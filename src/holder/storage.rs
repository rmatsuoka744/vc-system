use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::credential::CredentialResponse;

pub trait Storage: Send + Sync {
    fn store(&self, id: String, credential: CredentialResponse) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<CredentialResponse>, String>;
    fn get(&self, id: &str) -> Result<Option<CredentialResponse>, String>;
}

pub struct MemoryStorage {
    credentials: Arc<Mutex<HashMap<String, CredentialResponse>>>,
}

impl MemoryStorage {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MemoryStorage {
            credentials: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Storage for MemoryStorage {
    fn store(&self, id: String, credential: CredentialResponse) -> Result<(), String> {
        let mut credentials = self.credentials.lock().map_err(|_| "Failed to acquire lock")?;
        credentials.insert(id, credential);
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<CredentialResponse>, String> {
        let credentials = self.credentials.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(credentials.values().cloned().collect())
    }

    fn get(&self, id: &str) -> Result<Option<CredentialResponse>, String> {
        let credentials = self.credentials.lock().map_err(|_| "Failed to acquire lock")?;
        Ok(credentials.get(id).cloned())
    }
}

#[cfg(test)]
pub mod test_storage {
    use super::*;

    pub struct TestStorage {
        credentials: Mutex<HashMap<String, CredentialResponse>>,
    }

    impl TestStorage {
        pub fn new() -> Self {
            TestStorage {
                credentials: Mutex::new(HashMap::new()),
            }
        }
    }

    impl Storage for TestStorage {
        fn store(&self, id: String, credential: CredentialResponse) -> Result<(), String> {
            let mut credentials = self.credentials.lock().map_err(|_| "Failed to acquire lock")?;
            credentials.insert(id, credential);
            Ok(())
        }

        fn get_all(&self) -> Result<Vec<CredentialResponse>, String> {
            let credentials = self.credentials.lock().map_err(|_| "Failed to acquire lock")?;
            Ok(credentials.values().cloned().collect())
        }

        fn get(&self, id: &str) -> Result<Option<CredentialResponse>, String> {
            let credentials = self.credentials.lock().map_err(|_| "Failed to acquire lock")?;
            Ok(credentials.get(id).cloned())
        }
    }
}
