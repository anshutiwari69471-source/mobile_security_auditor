# Mobile Security Auditor

A fast, Rust-based tool to audit mobile app security reports.

## Features
- Calculates security score (0-100%)
- Classifies Risk Level: HIGH (0-49%), MEDIUM (50-79%), LOW (80-100%)
- Validates JSON reports
- Generates `security_report.json`

## Build
```bash
cargo build --release
./target/release/mobile_security_auditor
./target/release/mobile_security_auditor --help
cargo test
# 11 passed, 0 failed
