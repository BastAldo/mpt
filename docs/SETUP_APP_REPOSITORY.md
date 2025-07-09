# Blueprint: Initial Setup for `mio_trainer_app` Repository

This document contains all necessary instructions and code to set up the `mio_trainer_app` repository from scratch.

---

## Part 1: Repository Standards

### 1.1. .gitignore
The repository MUST contain a `.gitignore` file in its root with the following content. This is tailored for a Rust-based WebAssembly frontend using Leptos. It ignores Rust build artifacts, frontend-specific directories, and local environment files. The `Cargo.lock` file is intentionally NOT ignored to guarantee reproducible builds.

```gitignore
# Rust/Cargo build artifacts
/target/

# Frontend build artifacts
/pkg/
/dist/
/node_modules/

# Local environment files
.env

# IDE and editor configuration
.vscode/
.idea/
*.swp
```