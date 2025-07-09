# Project Architecture

*Status: Approved*

## 1. Overview

This project adopts a **multi-repo** architecture based on three distinct and specialized repositories. This decision was made to apply the **Principle of Maximum Granularity** and to proactively address the constraints related to the AI assistant's context window during development.

The three repositories are:

1.  **`mio_trainer_docs`**: The strategic brain. It contains all documentation, specifications, and the API contract. It is the Single Source of Truth for the design.
2.  **`mio_trainer_api`**: The engine. It contains the backend implementation (API server) in Rust with Axum.
3.  **`mio_trainer_app`**: The interface. It contains the frontend implementation (WASM) in Rust with Leptos.

## 2. Workflow

Development is contract-driven. Any new feature requiring communication between the frontend and backend must first be defined and approved in the `API_CONTRACT.md` file within the `mio_trainer_docs` repository.

## 3. Deployment Strategy

The project utilizes a Continuous Integration & Continuous Deployment (CI/CD) pipeline for the backend API.

-   **Source Control**: `mio_trainer_api` repository on GitHub.
-   **Automation**: GitHub Actions is used to automate the build and deploy process.
-   **Process**: On every push to the `main` branch, a workflow is triggered to compile the Rust application in a clean environment, creating a production-ready binary.
-   **Deployment**: The compiled binary is deployed to the production VPS using `rsync` over SSH.
-   **Runtime**: The application runs as a `systemd` service on the VPS for robustness and auto-restarts.
-   **Proxy**: `Nginx` is used as a reverse proxy to manage incoming traffic and route it to the application.

## 4. AI Architect Responsibilities

To ensure a precise, error-free, and predictable workflow, the AI Architect (Architeto RUST) MUST adhere to the following crystallized rules:

1.  **Explicit Context Declaration**: At the start of any new work phase, the first action MUST be to declare the current operational repository context (e.g., `mio_trainer_docs`, `mio_trainer_api`).
2.  **Strict Two-Phase Process**: The architect MUST always follow the two-phase process.
    -   **Phase 1: Analysis and Action Plan (Markdown).** Always conclude by requesting explicit user approval.
    -   **Phase 2: Implementation (YAML).** NEVER generate an implementation YAML file without explicit user approval.
3.  **Architectural Purity Principle**: Any proposed file creation or modification MUST strictly respect the designated purpose of the current repository, as defined in this document. No source code in `mio_trainer_docs`, no documentation unrelated to the API in `mio_trainer_api`.
4.  **Best Practice over Apparent Simplicity**: The initial proposed plan MUST always be based on the most robust and professional best practice, with a clear justification, rather than a potentially confusing oversimplification.