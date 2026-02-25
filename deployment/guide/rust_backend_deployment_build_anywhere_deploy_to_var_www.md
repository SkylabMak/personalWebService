# Rust Backend Deployment (Build Anywhere → Deploy to /var/www)

This version avoids `cd /var/www/rust-backend` entirely.
You can build **from any working copy** (home directory, CI runner, temp folder) and only **copy artifacts** into `/var/www/rust-backend`.

This is safer, cleaner, and closer to real production practice.

---

## Directory Model

- **Source (anywhere)**
  ```
  /home/skylab135/personalWebService   # git repo
  ```

- **Runtime (fixed location)**
  ```
  /var/www/rust-backend
  ├── rust-backend        # binary only
  └── .env                # runtime env (NOT in git)
  ```

`systemd` only cares about `/var/www/rust-backend`.

---

## One-Time Server Setup

### 1. Create runtime directory

```bash
sudo mkdir -p /var/www/rust-backend
sudo chown -R rustuser:rustuser /var/www/rust-backend
```

### 2. Place environment file

```bash
sudo nano /var/www/rust-backend/.env
```

Example:
```env
DATABASE_URL=mysql://...
RUST_LOG=info
```

Permissions:
```bash
sudo chown rustuser:rustuser /var/www/rust-backend/.env
sudo chmod 600 /var/www/rust-backend/.env
```

---

## Standard Update Flow (Recommended)

You stay **inside your git repo**.

### 1. Pull latest code

```bash
git pull origin main
```

### 2. Build release binary (local path)

```bash
cargo build --release
```

Output:
```text
target/release/rust-backend
```

---

### 3. Stop service

```bash
sudo systemctl stop rust-backend
```

---

### 4. Copy binary to runtime directory

```bash
sudo cp target/release/rust-backend /var/www/rust-backend/
```

Set ownership & permissions:
```bash
sudo chown rustuser:rustuser /var/www/rust-backend/rust-backend
sudo chmod +x /var/www/rust-backend/rust-backend
```

---

### 5. Start service

```bash
sudo systemctl start rust-backend
```

Verify:
```bash
sudo systemctl status rust-backend
```

---

## Optional: Atomic Update (No Partial Deploy)

Safer when traffic is high.

```bash
sudo systemctl stop rust-backend
sudo cp target/release/rust-backend /var/www/rust-backend/rust-backend.new
sudo mv /var/www/rust-backend/rust-backend.new /var/www/rust-backend/rust-backend
sudo chown rustuser:rustuser /var/www/rust-backend/rust-backend
sudo chmod +x /var/www/rust-backend/rust-backend
sudo systemctl start rust-backend
```

`mv` is atomic → no half-written binary.

---

## Rollback Strategy (Critical)

Before copying new binary:

```bash
sudo cp /var/www/rust-backend/rust-backend \
        /var/www/rust-backend/rust-backend.backup
```

Rollback:
```bash
sudo systemctl stop rust-backend
sudo mv /var/www/rust-backend/rust-backend.backup \
        /var/www/rust-backend/rust-backend
sudo systemctl start rust-backend
```

---

## Why This Approach Is Better

- ✔ Source code never runs in `/var/www`
- ✔ `/var/www` is **runtime-only**
- ✔ Cleaner permissions
- ✔ CI/CD friendly
- ✔ Easy rollback
- ✔ Matches real-world prod layout

---

## systemd Reminder (Expected)

```ini
[Service]
User=rustuser
WorkingDirectory=/var/www/rust-backend
ExecStart=/var/www/rust-backend/rust-backend
EnvironmentFile=/var/www/rust-backend/.env
Restart=always
```

---

## Mental Model (Important)

> **Build anywhere. Run in one place.**

This separation is exactly what you want long-term.

---

If you want next:
- zero-downtime restart
- GitHub Actions → auto deploy
- checksum validation before replace
- multi-binary versioned releases

Tell me which one and I’ll extend this doc.

