use crate::error::Result;
use async_trait::async_trait;

// TODO: Implement the store trait
// Requirements:
// - All methods should be async
// - Handle concurrent access safely
// - Verify signatures for write operations
#[async_trait]
pub trait Store: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn put(&self, key: String, value: Vec<u8>) -> Result<()>;
    async fn delete(&self, key: &str) -> Result<()>;
}

// TODO: Implement the in-memory store
// Requirements:
// - Use RwLock for concurrent access
// - Store values with their signatures
// - Handle errors appropriately
pub struct InMemoryStore {
    // Add your fields here
}

impl InMemoryStore {
    pub fn new() -> Self {
        // TODO: Implement constructor
        unimplemented!()
    }
}

#[async_trait]
impl Store for InMemoryStore {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        // TODO: Implement get operation
        unimplemented!()
    }

    async fn put(&self, key: String, value: Vec<u8>) -> Result<()> {
        // TODO: Implement put operation
        // Requirements:
        // - Create a signed value
        // - Store it in the map
        // - Handle concurrent access
        unimplemented!()
    }

    async fn delete(&self, key: &str) -> Result<()> {
        // TODO: Implement delete operation
        // Requirements:
        // - Remove the key-value pair
        // - Handle concurrent access
        // - Return appropriate error if key not found
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryStore;
    // TODO: Implement tests for:
    // - Basic CRUD operations
    // - Concurrent access
    // - Signature verification
    // - Error cases
    #[test]
    fn test_basic_operations() {
        // TODO: Implement test
        unimplemented!()
    }
}
