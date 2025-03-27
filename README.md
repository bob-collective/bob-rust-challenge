# Bob Rust Interview Challenge

This is a 1-hour coding challenge designed to evaluate Rust programming skills, particularly focusing on async programming, concurrency, and system design.

## Challenge Overview

You will implement a distributed key-value store with the following requirements:

1. **Core Features** (45 minutes):
   - Implement the `Store` trait in `src/store.rs`
   - Complete the `InMemoryStore` implementation with concurrent access
   - Implement signature verification in `src/crypto.rs`
   - Write tests for your implementation

2. **Technical Requirements**:
   - Use async/await for all I/O operations
   - Use RwLock for concurrent access
   - Implement proper error handling
   - Write clean, idiomatic Rust code

3. **Bonus Challenge** (15 minutes if time permits):
   - Implement the network server in `src/server.rs`
   - Handle client connections and requests
   - Verify signatures for write operations

## Project Structure

- `src/lib.rs`: Core library implementation and re-exports
- `src/error.rs`: Custom error types (TODO: implement error handling)
- `src/store.rs`: Key-value store implementation (TODO: implement Store trait)
- `src/crypto.rs`: Signature verification (TODO: implement signature scheme)
- `src/server.rs`: Network server implementation (Bonus)
- `tests/`: Test files

## Getting Started

1. Review the code structure and TODOs in each file
2. Implement the required functionality
3. Add tests for your implementation
4. If time permits, implement the bonus challenge

## Evaluation Criteria

Your implementation will be evaluated on:

1. Code quality and idiomatic Rust usage
2. Proper handling of concurrency and async operations
3. Error handling and type safety
4. Test coverage and quality
5. Documentation and comments

## Tips

- Start with the core functionality in `store.rs`
- Write tests as you implement features
- Use the provided dependencies effectively
- Consider edge cases and error conditions
- Document your design decisions

## Dependencies

- tokio: Async runtime
- serde: Serialization
- thiserror: Error handling
- sha2: Cryptographic hashing

Good luck! 