# ⚡ High-Performance Concurrent TCP Port Scanner in Rust

A lightweight, blazing-fast CLI network utility engineered in Rust to scan network hosts for open TCP ports using multi-threaded concurrency. Built completely with the Rust standard library—zero third-party dependencies.

## 🚀 Features
- **Highly Concurrent Engine:** Spawns asynchronous lightweight threads to audit thousands of network sockets simultaneously.
- **Dynamic Port Bounds:** Accepts customized ranges from the command line.
- **Beautiful UI:** Provides a live rendering progress bar and structural summary reporting.
- **Safe Architecture:** Guarantees data-race safety using Rust's safe message-passing channels (`std::sync::mpsc`).

## 🏗️ Technical Architecture Details
- **Resource-Optimized Thread Pool:** Instead of unbounded thread creation, the architecture manages resources deterministically via a configured worker pool (`MAX_WORKERS`), preventing kernel thread starvation.
- **Thread-Safe Shared Queue:** Utilizes thread-safe smart pointers `std::sync::Arc` wrapped around a mutual exclusion primitive `std::sync::Mutex` to handle dynamic atomic task distribution across workers without deadlocks.
- **Service Banner Identification:** Features standard protocol signature tracking to automatically cross-reference open socket hits to native service handlers (e.g., HTTP, SSH, HTTPS).

## 🛠️ Installation & Setup

1. Clone the repository:
   ```bash
   git clone [https://github.com/Suprajakshaya116/port_scannergit](https://github.com/Suprajakshaya116/port_scanner.git)
   cd port_scanner
