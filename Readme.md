# 🔐 Rustsign

**rustsign** is a lightweight Rust-based CLI tool for digitally signing and verifying messages using Ed25519 public-key cryptography.

Whether you're verifying the integrity of a message or signing important data, rustsign provides a fast and secure interface with minimal dependencies.

---

## ✨ Features

- ✅ Generate secure Ed25519 keypairs
- ✍️ Sign plain text messages
- 🔍 Verify messages with public keys
- 💾 Save and load keys from local storage
- 🔒 Fully offline; no network dependency

---

## 🛠 Prerequisites

- [Rust](https://rust-lang.org/tools/install) (version 1.70+ recommended)
- Git (for cloning the repo)

---

## 🚀 Installation

### Clone & Build

```
git clone https://github.com/your-username/rustsign.git
cd rustsign
cargo build --release

```

This will generate the binary at target/release/rustsign.

Optionally, copy it to your global binary path:

```
cp target/release/rustsign ~/.cargo/bin/

```

### Usage
```
rustsign <COMMAND> [ARGS]

```

### Generate a Keypair
```

rustsign generate

```

### ✍️ Sign a Message

````

rustsign sign "your message here"

```

example

signet sign "Hello, Mubarak"


✍️ Signature (base64): u+MHwZITj3+TLxWql6s1...


### ✅ Verify a Signature

signet verify "your message here" "base64_signature"


signet verify "Hello, Mubarak" "u+MHwZITj3+TLxWql6s1..."


✅ Signature is valid.

❌ Signature is INVALID!



🔐 Security Notes
Your private key is sensitive — keep keys/private.key safe and encrypted.

Signet does not transmit any data and runs entirely offline.

You can move keys between machines securely via encrypted file transfer.


📄 License
This project is licensed under the MIT License. See LICENSE for details.


🙋 Author
Mubarak Aminu
GitHub • Twitter


Built with 💙 and 🦀 in Rust
