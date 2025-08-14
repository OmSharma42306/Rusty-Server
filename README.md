# 🦀 Simple HTTP Server in Rust

This project demonstrates building a basic HTTP server entirely from scratch using Rust's standard library. It's designed to provide a deep understanding of networking, concurrency, and HTTP protocol fundamentals without relying on high-level frameworks.

## ✨ Core Technologies
- **std::net**: For low-level TCP communication (`TcpListener`, `TcpStream`).
- **std::thread**: For managing concurrent connections, eventually via a custom thread pool.
- **Manual Parsing**: Implementing custom logic to parse HTTP request bytes and format responses.

## 🚀 Key Features
- **TCP Listener**: Binds to a local address (e.g., `127.0.0.1:7878`) and accepts incoming connections.
- **Request Handling**: Reads raw bytes from the TCP stream and sends back simple HTTP responses.
- **Concurrency Model**: Utilizes a custom thread pool to efficiently handle multiple client requests simultaneously.
- **HTTP Request Parsing**: Manually parses the request line (Method, Path, HTTP-Version) and headers.
- **Static File Serving**: Serves static HTML files from a designated `public/` directory.
- **Basic Error Handling**: Returns `404 Not Found` for missing files and manages other common I/O errors.
- **Graceful Shutdown**: Implements a mechanism for clean server termination (e.g., via `Ctrl+C`).
- **Simple Logging**: Logs incoming requests for monitoring.

## 🛠️ Rust Concepts Reinforced
This project is an excellent way to solidify your understanding of:
- **Ownership & Borrowing**: Crucial for safe data sharing across threads.
- **Enums & Pattern Matching**: For structuring HTTP methods and request types.
- **Concurrency Primitives**: Deep dive into `std::thread`, channels (`std::sync::mpsc`), and mutexes.
- **Error Handling**: Extensive use of `Result` for robust application flow.
- **Byte & String Manipulation**: Efficient handling of network buffers and text processing.

## ▶️ Getting Started
To run this server:
1. Clone the repository.
2. Ensure you have Rust and Cargo installed.
3. Navigate to the project directory.
4. Run the server using `cargo run`.
5. Access it via your browser at `http://localhost:7878`.
