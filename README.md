# 🌟 zenith

Zenith is a next-generation build infrastructure system designed to go beyond the capabilities of [summit](https://github.com/serpent-os/summit).

## 🎯 Goals

### 🔒 Security & Updates Tracking
  - Track security vulnerabilities and updates across packages
  - Flag packages with CVEs and show which updates would resolve them

### 📊 Smart Release Monitoring
  - Domain-specific provider system for checking updates across different platforms (GitHub, PyPI, etc)
  - Centralized source analysis and metadata extraction
  - RSS feed monitoring support
  - Caching of upstream check results

### 🏗️ Enhanced Build System
  - Transient repos per PR with try build support
  - Log streaming from builders via gRPC
  - Testing before release promotion

### 🔄 Modern Architecture
  - gRPC-based core daemon with separate web frontend
  - Token-based authentication and access control
  - Rate limiting support
  - Structured logging via tracing framework

### 🔄 Smart Update Grouping
  - Stack/tier grouping for updates (e.g. GNOME, KDE Plasma)
  - Continuous PR updates as new component versions are released
  - Dependency-aware PR generation

### 🔐 Security
  - EdDSA signed JWTs for authentication
  - Trusted hosts system with public key pairing
  - Role-based access mapped to public keys
  - No need for DNS entries for builder nodes

## ⏳ Current Status

Work in progress with basic gRPC communication between components and tracing integration implemented.
The project aims to fulfill the original vision for summit while avoiding shortcuts and maintaining better state management.

# ⚖️ License

`zenith` is available under the terms of the [MPL-2.0](https://spdx.org/licenses/MPL-2.0.html)
