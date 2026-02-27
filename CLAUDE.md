# Development Setup

## Quick Development Commands

### Fast feedback loop (recommended):
```bash
# Watch for changes and run checks automatically
cargo watch -x check -x test

# Watch and run only the main binary on changes
cargo watch -x 'run --bin ruchess-cli'

# Watch with clear screen between runs
cargo watch -c -x check -x test
```

### Manual testing:
```bash
# Quick syntax/type check (fastest)
cargo check

# Run tests
cargo test

# Run the main binary
cargo run --bin ruchess-cli

# Build everything
cargo build
```

### Lint and formatting:
```bash
# Format code
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Run clippy linter
cargo clippy
```

## Project Structure
- `crates/ruchess-core/` - Core chess logic library
- `crates/ruchess-cli/` - Main binary and CLI interface

## Development Workflow
1. Use `cargo watch -x check -x test` for continuous feedback
2. Make your changes
3. Run `cargo fmt` before committing
4. Run `cargo clippy` to catch common issues