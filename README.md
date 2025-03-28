# BOB Rust Challenge

A distributed bulletin board system implementation challenge in Rust.

## Overview

This challenge involves implementing a distributed bulletin board system with the following key features:

- Message posting with signatures
- Message retrieval by public key
- Network communication using TCP
- Concurrent request handling
- In-memory message storage

## Requirements

### 1. Cryptographic Signatures

Implement a signature scheme in `src/crypto.rs`:
- Implement message signing
- Support message verification with public keys
- Handle errors appropriately
- Include comprehensive tests

### 2. Message Storage

Implement a storage interface in `src/store.rs`:
- Store messages by public key
- Support concurrent access
- Handle errors appropriately
- Include comprehensive tests

### 3. Network Server

Implement a TCP server in `src/server.rs`:
- Handle multiple concurrent connections
- Process post and get requests
- Verify signatures for post operations
- Return appropriate responses
- Handle errors appropriately
- Include comprehensive tests

## Project Structure

```
src/
├── crypto.rs    # Cryptographic signature implementation
├── error.rs     # Error types
├── lib.rs       # Library exports
├── server.rs    # Network server implementation
└── store.rs     # Message storage implementation
```

## Implementation Details

### Message Posting
- Messages must be signed with a private key
- Signatures must be verified using the corresponding public key
- Messages are stored and indexed by public key

### Message Retrieval
- Messages can be retrieved using a public key
- Returns all messages posted with that public key
- No signature verification needed for retrieval

### Network Protocol
- JSON-based request/response format
- TCP-based communication
- Support for concurrent connections

## Getting Started

1. Clone the repository
2. Implement the required functionality
3. Run the tests: `cargo test`
4. Run the server: `cargo run`

## Testing

The project includes comprehensive tests for each component:
- Cryptographic signature tests
- Storage interface tests
- Network server tests
- Integration tests

Run the tests with:
```bash
cargo test
```

## Evaluation Criteria

The implementation will be evaluated based on:
- Correctness of the cryptographic implementation
- Proper error handling
- Code organization and readability
- Test coverage
- Performance with concurrent connections
- Documentation quality 