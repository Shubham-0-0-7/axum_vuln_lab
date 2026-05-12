# Local SQL Injection Learning Exercise

**WARNING**: This application is intentionally vulnerable to SQL Injection. It is designed **strictly for local learning and educational purposes**.

Do not deploy this application to any public-facing server or expose it to untrusted networks.

## Overview
This is a basic Rust web application scaffolding using `axum`, `tokio`, and `rusqlite`.
A SQLite database has been initialized at `db/app.db` with a `users` table and some default data.

The route handlers in `src/main.rs` are currently left as empty stubs. Your task is to implement the route handlers, write the vulnerable query logic, and practice local SQL injection testing.

## Getting Started
1. Run the application:
   ```bash
   cargo run
   ```
2. The server will start on `http://127.0.0.1:3000`.
3. Implement the route handlers in `src/main.rs` to practice writing and exploiting SQL injection vulnerabilities!
