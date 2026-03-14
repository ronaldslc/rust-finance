# Contributing to RustForge Terminal

First off, thank you for considering contributing to RustForge! It's people like you that make RustForge such a great tool.

## General Guidelines

- **No Emojis:** Please refrain from using emojis in commit messages, documentation, or code comments. We maintain a strictly professional aesthetic.
- **Commit Messages:** Write clear, concise, and descriptive commit messages. Use the imperative mood (e.g., "Add feature" instead of "Added feature").
- **Branching:** Create a new branch for each feature or bug fix. Do not commit directly to the `main` branch.
- **Code Style:** Follow standard Rust formatting guidelines. Run `cargo fmt` before submitting your code.

## Getting Started

1.  **Fork** the repository on GitHub.
2.  **Clone** your fork locally:
    ```sh
    git clone https://github.com/YOUR_USERNAME/rust-finance.git
    cd rust-finance
    ```
3.  **Create a branch** for your specific changes:
    ```sh
    git checkout -b feature/your-feature-name
    ```

## Development Workflow

### Building

Make sure you have Rust installed. Build the entire workspace:

```sh
cargo build --workspace
```

### Testing

Before submitting a pull request, ensure all tests pass:

```sh
cargo test --workspace
```

## Pull Request Process

1.  Push your changes to your fork on GitHub.
2.  Submit a pull request to the `main` branch of the original repository.
3.  Ensure your PR description clearly states the problem addressed and the solution implemented.
4.  Link any relevant issues in the PR description (e.g., "Fixes #123").
5.  Wait for review. Address any feedback provided by the maintainers.

## Setting Up Environment Variables

If you are working on components that require external APIs (like `daemon` or `ingestion`), make sure you have the necessary environment variables set up locally. We recommend creating a `.env` file (which is gitignored) for your local keys:

```sh
ANTHROPIC_API_KEY="your_key"
FINNHUB_API_KEY="your_key"
ALPACA_API_KEY="your_key"
ALPACA_SECRET_KEY="your_key"
```

## Architecture Notes

Please review the architecture diagram in the `README.md` before making significant structural changes to ensure your modifications align with the overall system design.

Thank you for contributing!
