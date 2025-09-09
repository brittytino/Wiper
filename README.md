# 🧼 SecureWipe — Secure Data Wiping for Trustworthy IT Asset Recycling

A cross-platform, user-friendly, and verifiable data wiping tool built with **Tauri**, **Vue 3**, and **Rust** to promote secure e-waste recycling in India and beyond.

---

## 🚨 Problem Statement

India generates over **1.75 million tonnes of e-waste annually**, but **data privacy concerns** prevent users from recycling old devices. Most tools for secure data sanitization are **either too technical, expensive, or unverifiable**, leading to over ₹50,000 crore worth of IT assets being hoarded in homes and offices.

To address this, we’re building a **tamper-proof, auditable, and easy-to-use data wiping solution** that encourages responsible IT asset disposal, in alignment with global standards.

---

## 🎯 Objectives

- ✅ **Securely wipe user data**, including hidden sectors like HPA/DCO and SSD remapped blocks.
- ✅ **Generate digitally signed wipe certificates** (PDF + JSON) for each operation.
- ✅ **One-click operation** for accessibility by non-technical users.
- ✅ **Offline usability** for field technicians (bootable ISO/USB support).
- ✅ **Enable 3rd-party verification** of wipe status for trust and compliance.
- ✅ **Comply with NIST SP 800-88** data sanitization standards.
- ✅ **Cross-platform** support: Windows, Linux, and Android (prototype).

---

## 🧱 Tech Stack

| Layer           | Technology                              |
|-----------------|-----------------------------------------|
| **Frontend**    | Vue 3, Vite, TypeScript                 |
| **Backend**     | Rust, Tauri                             |
| **Certificate** | JSON + Digital Signature, PDF (planned) |
| **Tooling**     | GitHub Codespaces, Cargo, npm           |
| **Packaging**   | Tauri CLI                               |

---

## 🖥️ App Preview

> A simple, elegant UI with step-by-step wipe status and certificate generation.

🟡 **Coming soon**: Screenshots and demo GIFs.

---

## 🛠️ How to Run (Dev Mode)

### 📦 Requirements

- **Rust & Cargo**
- **Node.js & npm**
- GitHub Codespaces or local dev environment
- **Tauri CLI** (install via `cargo install tauri-cli`)

### 🚀 Setup Instructions

```bash
# Clone the repo
git clone https://github.com/yourusername/securewipe.git
cd securewipe

# Frontend setup
npm install
npm run tauri dev  # Starts Vite dev server

# In another terminal: Run Tauri dev
cargo tauri dev
