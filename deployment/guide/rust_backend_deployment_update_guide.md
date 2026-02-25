# Rust Backend – Patch & Update Guide (via Git)

This document describes a **safe, repeatable procedure** to update your Rust backend when a new version is pushed to Git. It assumes:

- Binary is deployed under `/var/www/rust-backend`
- Service is managed by `systemd`
- Service user: `rustuser`
- Service name: `rust-backend.service`
- Repo lives at `/var/www/rust-backend`

---

## 0. One-time Preconditions (verify once)

### Service user
```bash
id rustuser
```

### Directory ownership
```bash
sudo chown -R rustuser:rustuser /var/www/rust-backend
```

### systemd unit file
```bash
sudo systemctl status rust-backend
```

---

## 1. Standard Update Flow (Recommended)

### Step 1: Switch to service directory
```bash
cd /var/www/rust-backend
```

### Step 2: Pull latest code
```bash
sudo -u rustuser git pull origin main
```
> Replace `main` with your branch if needed.

### Step 3: Build release binary
```bash
sudo -u rustuser cargo build --release
```

Binary output:
```
target/release/rust-backend
```

### Step 4: Restart service
```bash
sudo systemctl restart rust-backend
```

### Step 5: Verify
```bash
sudo systemctl status rust-backend
journalctl -u rust-backend -n 50 --no-pager
```

---

## 2. Zero-Downtime (Safer) Update Pattern

Use this when uptime matters.

### Step 1: Build first (without stopping service)
```bash
cd /var/www/rust-backend
sudo -u rustuser git pull origin main
sudo -u rustuser cargo build --release
```

### Step 2: Restart only after build success
```bash
sudo systemctl restart rust-backend
```

If build fails → **service keeps running**.

---

## 3. Rollback Strategy (Very Important)

### Option A: Roll back via Git
```bash
git log --oneline
sudo -u rustuser git checkout <COMMIT_HASH>
sudo -u rustuser cargo build --release
sudo systemctl restart rust-backend
```

### Option B: Keep old binary (recommended)
```bash
cp target/release/rust-backend target/release/rust-backend.bak
```
Restore:
```bash
mv target/release/rust-backend.bak target/release/rust-backend
sudo systemctl restart rust-backend
```

---

## 4. Environment Variables (.env)

### Location
Use **one stable location**:
```
/etc/rust-backend/.env
```

### Permissions
```bash
sudo chown rustuser:rustuser /etc/rust-backend/.env
sudo chmod 600 /etc/rust-backend/.env
```

### systemd must reference it
```
EnvironmentFile=/etc/rust-backend/.env
```

No need to touch `.env` during updates unless config changes.

---

## 5. systemd Service Checklist

```ini
[Service]
User=rustuser
WorkingDirectory=/var/www/rust-backend
ExecStart=/var/www/rust-backend/target/release/rust-backend
Restart=always
EnvironmentFile=/etc/rust-backend/.env
```

Reload if changed:
```bash
sudo systemctl daemon-reexec
sudo systemctl restart rust-backend
```

---

## 6. Common Mistakes (Avoid These)

❌ Running `cargo build` as `root`

❌ Editing `.env` inside repo

❌ Restarting service before build finishes

❌ Deploying from home directory

---

## 7. Optional: One-Command Update Script

Create `/usr/local/bin/update-rust-backend.sh`
```bash
#!/bin/bash
set -e
cd /var/www/rust-backend
sudo -u rustuser git pull origin main
sudo -u rustuser cargo build --release
sudo systemctl restart rust-backend
```

```bash
sudo chmod +x /usr/local/bin/update-rust-backend.sh
```

Run update:
```bash
sudo update-rust-backend.sh
```

---

## 8. Final Mental Model

- **Git** = version source
- **cargo build --release** = produce binary
- **systemd** = lifecycle manager
- **Cloudflare** = TLS / edge only

Backend stays stable and predictable.

---

If you want next:
- Blue/Green deployment
- Auto deploy via GitHub Actions
- Health-check based restarts
- Log rotation

Just say the word.

