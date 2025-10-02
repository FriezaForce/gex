# Contributing to gex

Thank you for your interest in contributing to gex! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/FriezaForce/gex.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Style

We use `rustfmt` and `clippy` for code formatting and linting:

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Pull Request Guidelines

- Keep PRs focused on a single feature or bug fix
- Write clear commit messages
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass
- Follow the existing code style

## Reporting Issues

When reporting issues, please include:

- Your operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Any error messages or logs

## Feature Requests

We welcome feature requests! Please:

- Check if the feature has already been requested
- Clearly describe the feature and its use case
- Explain why it would be useful to most users

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn and grow

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
