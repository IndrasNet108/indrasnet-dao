# 📚 Legal CLI - Usage Guide

**Automated legal document generator for IndrasNet DAO**

---

## 🚀 Quick Start

### Installation and Build

```bash
# Clone repository
git clone https://gitlab.com/IndrasNet/indrasnet-dao.git
cd indrasnet-dao

# Go to CLI directory
cd cli

# Build project
cargo build --release

# Or run directly
cargo run -- --help
```

---

## 📋 Main Commands

### 1. 📊 Read Existing Data

```bash
# Read and analyze JSON file
cargo run -- read-json --input ../legal_disclaimer.json

# Result:
# ✅ JSON file successfully read: ../legal_disclaimer.json
# 📊 Jurisdiction: EU
# 💰 Total GDPR fines volume: €5.88 billion
# 🏢 Violations count: 1847
# 🔐 Quantum algorithms: 4
# 📅 Last update: 2025-10-11T15:20:00Z
# ⭐ Compliance score: 100/100
```

### 2. 📄 Generate Individual Documents

```bash
# GDPR documentation
cargo run -- gdpr --output gdpr-report.md

# Legal disclaimer
cargo run -- disclaimer --output legal-disclaimer.md

# Terms of service
cargo run -- terms --output terms-of-service.md
```

### 3. 🔄 Update Existing Documents

```bash
# Update legal disclaimer with new data
cargo run -- update --input legal_disclaimer.json --output updated-disclaimer.md
```

---

## 📚 Full Documentation

For complete documentation, see:
- `legal-cli-cheatsheet.md` - Quick reference
- CLI help: `cargo run -- --help`

---

**Note:** This is a summary. For detailed usage, refer to the full documentation.
