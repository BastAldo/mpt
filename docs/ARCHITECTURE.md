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
