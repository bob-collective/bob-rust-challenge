use crate::crypto::Signature;
use crate::{Result, Store};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// TODO: Implement request/response types
// Requirements:
// - Support all store operations (get/put/delete)
// - Include necessary fields for signatures
// - Implement Serialize/Deserialize
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Get {
        key: String,
    },
    Put {
        key: String,
        value: Vec<u8>,
        signature: Signature,
    },
    Delete {
        key: String,
        signature: Signature,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Get { value: Option<Vec<u8>> },
    Put,
    Delete,
    Error { message: String },
}

// TODO: Implement the server
// Requirements:
// - Handle multiple concurrent connections
// - Process requests and return responses
// - Verify signatures for write operations
pub struct Server {
    store: Box<dyn Store>,
}

impl Server {
    pub fn new(store: Box<dyn Store>) -> Self {
        Self { store }
    }

    // TODO: Implement server start method
    // Requirements:
    // - Listen on the given port
    // - Handle multiple connections concurrently
    // - Process client requests
    pub async fn start(&self, port: u16) -> Result<()> {
        // TODO: Implement server startup
        unimplemented!()
    }

    // TODO: Implement connection handler
    // Requirements:
    // - Read requests from the client
    // - Process requests using the store
    // - Send responses back to the client
    // - Handle errors appropriately
    async fn handle_connection(&self, socket: TcpStream) -> Result<()> {
        // TODO: Implement connection handling
        unimplemented!()
    }

    // TODO: Implement request processing
    // Requirements:
    // - Handle all request types
    // - Verify signatures where needed
    // - Return appropriate responses
    async fn process_request(&self, request: Request) -> Result<Response> {
        // TODO: Implement request processing
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryStore;

    #[tokio::test]
    async fn test_server_with_signed_operations() -> Result<()> {
        // Create a store and server
        let store = Box::new(InMemoryStore::new());
        let server = Server::new(store);

        // Start server in a separate task
        let port = 8080;
        let server_handle = tokio::spawn(async move { server.start(port).await });

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Create a test client
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;

        // Test put operation with signature
        let key = "test_key".to_string();
        let value = vec![1, 2, 3];
        let signature = Signature::sign(&value, b"test_key")?;

        let request = Request::Put {
            key: key.clone(),
            value: value.clone(),
            signature,
        };

        // Send request
        let request_json = serde_json::to_string(&request)?;
        stream.write_all(request_json.as_bytes()).await?;

        // Read response
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response: Response = serde_json::from_slice(&buffer[..n])?;

        // Verify response
        match response {
            Response::Put => (), // Success
            Response::Error { message } => panic!("Unexpected error: {}", message),
            _ => panic!("Unexpected response type"),
        }

        // Test get operation
        let request = Request::Get { key: key.clone() };
        let request_json = serde_json::to_string(&request)?;
        stream.write_all(request_json.as_bytes()).await?;

        let n = stream.read(&mut buffer).await?;
        let response: Response = serde_json::from_slice(&buffer[..n])?;

        match response {
            Response::Get { value: Some(v) } => assert_eq!(v, value),
            Response::Get { value: None } => panic!("Expected value, got None"),
            Response::Error { message } => panic!("Unexpected error: {}", message),
            _ => panic!("Unexpected response type"),
        }

        // Clean up
        server_handle.abort();

        Ok(())
    }
}
