# ⚡ High-Performance Concurrent TCP Port Scanner in Rust

A lightweight, blazing-fast CLI network utility engineered in Rust to scan network hosts for open TCP ports using multi-threaded concurrency. Built completely with the Rust standard library—zero third-party dependencies.

## 🚀 Features
- **Highly Concurrent Engine:** Spawns asynchronous lightweight threads to audit thousands of network sockets simultaneously.
- **Dynamic Port Bounds:** Accepts customized ranges from the command line.
- **Beautiful UI:** Provides a live rendering progress bar and structural summary reporting.
- **Safe Architecture:** Guarantees data-race safety using Rust's safe message-passing channels (`std::sync::mpsc`).

## 🛠️ Installation & Setup

1. Clone the repository:
   ```bash
   git clone [https://github.com/Suprajakshaya116/port_scannergit](https://github.com/Suprajakshaya116/port_scanner.git)
   cd port_scanner
