<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Stable-brightgreen.svg" /> 
  <img src="https://img.shields.io/badge/Library-Serde-blue.svg" />
  <img src="https://img.shields.io/badge/Library-Chrono-red.svg" />
</p>

<h1 align="center">🛡️ Sentinel Key CLI Rust - Security Access Log System</h1>

<p align="center">
  A robust Command Line Interface (CLI) engine demonstrating advanced Rust patterns: Enums with associated data, Nested Structures, and persistent JSON Auditing.
</p>

---

## 🎓 Educational Disclaimer
This repository is part of my **Personal Apprenticeship** and learning journey with the Rust programming language. 
* **Purpose**: This code is strictly for educational purposes. It serves as a practical sandbox to master Rust's ownership system, complex nested vectors, and type-safe enums.
* **Evolution**: This project marks a significant step forward from IronVault, introducing deeper data hierarchies and sophisticated error handling via pattern matching.
* **Feedback**: Constructive feedback is welcome as I work towards becoming a proficient Rust developer!

## 🌟 Features
* **Advanced Enums**: Implements `ServiceType` with associated data (IP and Protocol) to categorize local vs remote assets.
* **Nested Data Persistence**: Manages complex `Vec<AccessEvent>` inside a parent `SentinelVault` struct, all serialized via `Serde`.
* **Automated Auditing**: Every access attempt is tracked with a high-precision `Chrono` UTC timestamp.
* **Resilient I/O**: Uses pattern matching (`match`) to handle database initialization, ensuring the system never crashes on a missing file.

## 🛠️ Technical Deep Dive
* **Associated Data**: Unlike simple enums, `ServiceType::Remote` carries a `String` IP and `String` Protocol, showcasing Rust's unique type capabilities.
* **Mutable Borrowing**: Leverages `get_mut()` to safely modify specific services within the vector without cloning data.
* **Safe Parsing**: Uses exhaustive `match` blocks to distinguish between a "First Run" scenario (empty DB) and an existing database.



---

## 🚀 How to Run
1. Clone the repository.
2. Ensure you have [Rust](https://www.rust-lang.org/) installed.
3. Run the following command in your terminal:
   ```bash
   cargo run

## ⚖️ License & Copyright

Copyright (c) 2026 **[dandiest]**

This project is licensed under the MIT License.

*You are free to use, study, and modify this code for educational purposes. Please provide attribution if you use significant portions of this logic in your own learning journey.*
