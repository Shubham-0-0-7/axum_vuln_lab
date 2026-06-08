# axum_vuln_lab

**WARNING**: This application is intentionally vulnerable to multiple web vulnerabilities. It is designed **strictly for local learning and educational purposes**.
Do not deploy this application to any public-facing server or expose it to untrusted networks.

## Overview
This is a Rust web application built using `axum` and `tokio`. The purpose of this repository is to learn web security by:
1. Building a vulnerable feature
2. Exploiting it to understand the impact
3. Fixing it with secure coding practices

Currently, the lab includes the following modules (with both vulnerable and secure/fixed implementations):
- **SQL Injection (SQLi)**: Exploring unsanitized database queries using `rusqlite`.
- **Cross-Site Scripting (XSS)**: Understanding how unsanitized user input is rendered in the DOM.
- **Cross-Origin Resource Sharing (CORS)**: Learning how permissive origin headers expose sensitive API endpoints to attacker origins.
- **Command Injection (CMDi)**: Exploring the dangers of passing user input directly to the system shell.

## Getting Started
1. Run the application:
   ```bash
   cargo run
   ```
2. The main server will start on `http://127.0.0.1:3000` (and the CORS API server on port `3001`).
3. Explore the different routes to test the vulnerabilities and compare them with their fixed counterparts!
