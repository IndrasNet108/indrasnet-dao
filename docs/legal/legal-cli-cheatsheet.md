# 🚀 Legal CLI - Quick Reference

**Quick commands for working with legal documents**

---

## 📋 Main Commands

```bash
# Navigate to CLI directory
cd cli

# Build project
cargo build --release
```

### 🔍 Data Analysis

```bash
# Read JSON file
cargo run -- read-json --input ../legal_disclaimer.json

# Read with statistics output
cargo run -- read-json
```

### 📄 Document Generation

```bash
# GDPR report
cargo run -- gdpr --output gdpr-report.md

# MiCA compliance
cargo run -- mica --output mica-compliance.md

# Quantum cryptography
cargo run -- quantum --output quantum-crypto.md

# JSON export
cargo run -- export-json --output legal-data.json
```

### 🔄 Update

```bash
# Update all documents
cargo run -- update

# Update with parameters
cargo run -- update --output-dir ../docs/legal/ --json-file ../legal_disclaimer.json
```

### 🎯 All-in-One

```bash
# Generate all documents
cargo run -- all
```

---

## 🎯 Typical Scenarios

### First Run

```bash
cd cli
cargo build --release
cargo run -- all
```

### Weekly Update

```bash
cd cli
cargo run -- update
# Review generated documents
git add docs/legal/
git commit -m "Update legal documents"
```

### Create Report

```bash
cd cli
cargo run -- gdpr --output gdpr-report.md
cargo run -- mica --output mica-compliance.md
```

---

## 📊 What Gets Generated

- **`gdpr.md`** - GDPR fines, requirements, statistics
- **`mica.md`** - MiCA compliance, token classification
- **`quantum.md`** - Post-quantum algorithms, benchmarks
- **`legal_disclaimer.json`** - Structured data

---

## 🔧 Parameters

| Command | Parameter | Default | Description |
|---------|-----------|---------|-------------|
| `read-json` | `--input` | `legal_disclaimer.json` | Path to JSON file |
| `gdpr` | `--output` | `docs/legal/gdpr.md` | Output file |
| `mica` | `--output` | `docs/legal/mica.md` | Output file |
| `quantum` | `--output` | `docs/legal/quantum.md` | Output file |
| `export-json` | `--output` | `legal_disclaimer.json` | Output JSON |
| `update` | `--output-dir` | `docs/legal/` | Directory |
| `update` | `--json-file` | `legal_disclaimer.json` | JSON file |
| `all` | `--output-dir` | `docs/legal/` | Directory |

---

## ⚡ Quick Commands

```bash
# Everything in one
cargo run -- all --output-dir ../docs/legal/

# Update and commit
cargo run -- update && git add docs/legal/ && git commit -m "Update legal docs"

# Create report for regulator
cargo run -- gdpr --output regulator-report.md
```

---

## 🐛 Troubleshooting

```bash
# Compilation error
cargo clean && cargo build --release

# JSON error
cargo run -- read-json --input legal_disclaimer.json

# No write permissions
chmod +w docs/legal/
```

---

*Quick Reference v1.0 - October 11, 2025*
