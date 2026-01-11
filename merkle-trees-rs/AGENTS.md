# AGENTS.md - Development Guidelines for Merkle Trees Rust Project

This document provides essential information for agentic coding agents working in this Rust blockchain/cryptocurrency project.

## Build & Development Commands

### Core Commands
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release version
- `cargo test` - Run all tests (39 tests currently)
- `cargo test -- --nocapture` - Run tests with stdout output
- `cargo run` - Build and run the main application

### Single Test Execution
- `cargo test test_name` - Run specific test by name
- `cargo test module::tests::test_name` - Run specific test from module
- `cargo test block::difficulty` - Run all tests in difficulty module
- `cargo test --exact test_name` - Run exact match test name

### Linting & Formatting
- `cargo clippy` - Run Rust linter (default rules apply)
- `cargo fmt` - Format code (default rustfmt settings)
- `cargo check` - Quick compile check without generating binaries

### Examples from codebase:
```bash
cargo test block::difficulty::tests::test_creates_bits
cargo test blockchain::tests::test_verifies_chain_validity
cargo test hash::tests::test_hashes_str
```

## Code Style Guidelines

### Import Organization
Order imports in this specific hierarchy:
1. **Standard library**: `use std::time::{SystemTime, UNIX_EPOCH};`
2. **External crates**: `use sha2::{Digest, Sha256};`  
3. **Internal modules**: `use crate::hash::Hash;`

Use brace notation for multiple imports: `use std::fmt::{Display, Write};`

### Naming Conventions
- **Types/Structs**: PascalCase - `Block`, `Hash`, `DifficultyTarget`, `MerkleTree`
- **Functions/Methods**: snake_case - `new()`, `to_bytes()`, `build_block()`, `select_validator()`
- **Variables/Fields**: snake_case - `previous_hash`, `merkle_root`, `difficulty_target`
- **Constants**: SCREAMING_SNAKE_CASE (not extensively used in current codebase)
- **Modules**: snake_case - `blockchain`, `timestamp`, `merkle`

### Code Formatting
- **4-space indentation** (no tabs)
- **Line length**: Keep under 100 characters when practical
- **K&R brace style**: Opening brace on same line
- **Trailing commas**: Use in multi-line arrays/vectors for better diffs
- **Vertical spacing**: One blank line between logical sections

### Type System Patterns
- **Newtype pattern**: `struct Hash([u8; 32])`, `struct Timestamp(u32)`
- **Option for nullable**: `previous_hash: Option<Hash>`
- **Derive macros**: Use `#[derive(Debug, Clone, Eq, PartialEq)]` consistently
- **Custom implementations**: Implement `Display`, `From`, `PartialEq<String>` as needed

## Module Organization

### Project Structure
```
src/
├── main.rs           // Entry point, module declarations
├── block/            // Block-related functionality
│   ├── mod.rs        // Public exports, Block struct
│   ├── header.rs     // BlockHeader implementation  
│   └── difficulty.rs // DifficultyTarget value object
├── blockchain.rs     // Blockchain orchestration
├── hash.rs           // Hash abstraction and utilities
├── merkle.rs         // Merkle tree implementation
├── pow.rs            // Proof of work algorithms
├── pos.rs            // Proof of stake algorithms
└── timestamp.rs      // Timestamp utilities
```

### Module Guidelines
- **mod.rs** serves as public interface with `pub use` re-exports
- **Submodules** group related functionality (block module structure)
- **Minimal cross-dependencies** to maintain modularity
- **Clear separation** of concerns between components

## Testing Conventions

### Test Organization
- **Embedded tests**: Use `#[cfg(test)] mod tests` in each module
- **Descriptive names**: `test_creates_genesis_block()`, `test_builds_prefix()`
- **Arrange-Act-Assert**: Follow this pattern consistently
- **Helper functions**: Use for complex test data setup

### Test Naming Patterns
- `test_creates_[entity]()` - Constructor tests
- `test_[action]s_[entity]()` - Action tests
- `test_[entity]_with_[condition]()` - Conditional behavior tests  
- `test_[negative_scenario]()` - Error/edge case tests

### Example Test Structure:
```rust
#[test]
fn test_creates_block_with_previous() {
    // Arrange
    let genesis = Block::genesis(transactions, Some(Timestamp::new(0)), 0);
    
    // Act  
    let next_block = Block::new(
        Some(genesis.hash()),
        more_transactions,
        Some(Timestamp::new(0)),
        0,
    );
    
    // Assert
    assert_eq!(next_block.header.previous_hash, genesis.hash());
}
```

## Error Handling

### Current Patterns
- **Option for nullable values**: `unwrap_or_default()` for fallbacks
- **Limited panics**: Use `expect()` only for truly unrecoverable situations
- **Match expressions**: Comprehensive Option/Result handling
- **No custom error types**: Current codebase uses simple patterns

### Guidelines
- Prefer `Option<T>` over nullable references
- Use `unwrap_or_default()` for reasonable defaults
- Use `expect()` with descriptive messages for unrecoverable states
- Avoid `unwrap()` in production code without good reason

## Dependencies & External Crates

### Current Dependencies
- `sha2 = "0.10.9"` - Cryptographic hash functions
- `rand = "0.10.0-rc.5"` - Random number generation

### Adding Dependencies
- Keep dependencies minimal and focused
- Prefer stable, well-maintained crates
- Update Cargo.toml version numbers as needed

## Performance & Memory Patterns

### Ownership Patterns
- **Explicit cloning**: Use `.clone()` when ownership transfer needed
- **Borrowing**: Prefer references over copies when possible
- **Box pointers**: Use for recursive structures: `Option<Box<Node>>`

### Functional Patterns
- **Iterator chains**: `.iter().map().fold()` for data transformation
- **Higher-order functions**: `fold()`, `map()`, `filter()` for collections
- **Closures**: Use for local functionality and callbacks

## Special Considerations

### Cryptographic Code
- **Hash operations**: Use SHA-256 via `sha2` crate
- **Security**: Never commit secrets or keys
- **Randomness**: Use `rand` crate for cryptographic operations

### Blockchain Specific
- **Immutable data structures**: Blocks and hashes should be immutable
- **Serialization**: Implement `to_bytes()` for network/storage format
- **Validation**: Implement validation methods as part of core types

## File-Specific Guidelines

### When working with modules:
- **block/**: Core block structure, headers, and difficulty targets
- **hash.rs**: Hash abstraction layer - all hash operations go through here
- **merkle.rs**: Merkle tree implementation for transaction verification
- **blockchain.rs**: High-level blockchain orchestration and validation

### When adding tests:
- Follow existing test patterns in each module
- Use descriptive test names that explain what's being tested
- Include both positive and negative test cases
- Test edge cases and boundary conditions

## Development Workflow

1. **Understand the module**: Read existing code before making changes
2. **Follow conventions**: Match existing patterns for imports, naming, structure
3. **Write tests**: Add tests for new functionality following existing patterns
4. **Run tests locally**: Ensure `cargo test` passes before committing
5. **Check formatting**: Run `cargo fmt` to maintain consistent style
6. **Lint code**: Run `cargo clippy` to catch potential issues

Remember: This codebase emphasizes clarity, consistency, and comprehensive testing. When in doubt, follow the existing patterns in the relevant module.