# Blueprint: Initial Setup for `mio_trainer_api` Repository

This document contains all necessary instructions and code to set up the `mio_trainer_api` repository from scratch.

---

## Part 1: GitHub Repository Setup

### 1.1. Create Repository Secrets
In the `mio_trainer_api` repository on GitHub, navigate to `Settings > Secrets and variables > Actions` and create the following "Repository secrets":

-   `VPS_HOST`: The IP address of your VPS.
-   `VPS_USER`: The username for SSH connection (e.g., `root`).
-   `VPS_SSH_PRIVATE_KEY`: The full content of the private SSH key authorized for passwordless login on the VPS.
-   `VPS_TARGET_PATH`: The absolute path on the VPS where the compiled binary will be stored. **This path must match the one in the systemd service file.** (e.g., `/home/your_user/app/mio_trainer_api`).

### 1.2. Create the Deploy Workflow File
Create the file `.github/workflows/deploy.yml` with the following content:

```yaml
# Workflow name
name: Deploy API to VPS

# Trigger this workflow on every push to the main branch
on:
  push:
    branches:
      - main

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Build Rust binary
        run: cargo build --release

      - name: Deploy to VPS
        uses: easingthemes/ssh-deploy@main
        with:
          SSH_PRIVATE_KEY: ${{ secrets.VPS_SSH_PRIVATE_KEY }}
          REMOTE_HOST: ${{ secrets.VPS_HOST }}
          REMOTE_USER: ${{ secrets.VPS_USER }}
          SOURCE: "./target/release/mio_trainer_api"
          TARGET: ${{ secrets.VPS_TARGET_PATH }}
          SCRIPT_AFTER: |
            echo "Restarting the API service..."
            sudo systemctl restart mio_trainer_api.service
            echo "Service restarted."
```

---

## Part 2: VPS Server Setup

### 2.1. Nginx Configuration
The file `/etc/nginx/sites-available/mpt.api.ap0.it` should already exist and be configured as a reverse proxy. Its content should be:

```nginx
server {
    listen 80;
    listen [::]:80;

    server_name mtp.api.ap0.it;

    location / {
        proxy_pass [http://127.0.0.1:3000](http://127.0.0.1:3000);
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 2.2. systemd Service Configuration
Create the file `/etc/systemd/system/mio_trainer_api.service` on the VPS with the following content. **Ensure the paths match `VPS_TARGET_PATH`**.

```ini
[Unit]
Description=Mio Trainer API Service
After=network.target

[Service]
User=your_user # Replace with your actual username
Group=www-data

# This path MUST match the VPS_TARGET_PATH secret in GitHub
WorkingDirectory=/home/your_user/app/mio_trainer_api
ExecStart=/home/your_user/app/mio_trainer_api/mio_trainer_api

Restart=always

[Install]
WantedBy=multi-user.target
```
After creating the file, run `sudo systemctl daemon-reload` on the VPS.