# Contributing to Hermes

Thank you for your interest in contributing to Hermes! 🎉

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to build something great.

## How to Contribute

### 1. Fork & Clone

```bash
git clone https://github.com/YOUR_USERNAME/hermes.git
cd hermes
```

### 2. Create a Branch

```bash
git checkout -b feature/amazing-feature
```

### 3. Make Changes

- Write clean, documented code
- Follow Rust conventions
- Add tests for new features
- Update documentation

### 4. Test Your Changes

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy

# Build release
cargo build --release
```

### 5. Commit

```bash
git add .
git commit -m "Add amazing feature"
```

**Commit Message Format:**
```
<type>: <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Tests
- `chore`: Maintenance

**Example:**
```
feat: Add file chunking for large files

Implemented chunking mechanism to split files larger than 100MB
into smaller chunks for parallel upload/download.

Closes #42
```

### 6. Push & Create PR

```bash
git push origin feature/amazing-feature
```

Then create a Pull Request on GitHub.

## Development Setup

### Requirements

- Rust 1.70+
- Cargo
- Git

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Run with Logging

```bash
RUST_LOG=debug cargo run -- <command>
```

## Code Style

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Documentation

```rust
/// Brief description
///
/// # Arguments
///
/// * `param` - Parameter description
///
/// # Returns
///
/// Return value description
///
/// # Example
///
/// ```
/// let result = function(param);
/// ```
pub fn function(param: Type) -> Result<Output> {
    // implementation
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        assert_eq!(function(input), expected);
    }
}
```

### Integration Tests

Place in `tests/` directory.

## Areas We Need Help

### High Priority
- [ ] Performance optimizations
- [ ] Additional test coverage
- [ ] Documentation improvements
- [ ] Bug fixes

### Medium Priority
- [ ] File chunking
- [ ] Dead man's switch
- [ ] GUI application
- [ ] Mobile apps

### Low Priority
- [ ] Steganography
- [ ] Post-quantum crypto
- [ ] P2P transfer

## Questions?

- Open an issue
- Email: contact@chronocoder.dev

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
