use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::{Duration, Instant};

// Connection timeout per port
const TIMEOUT: Duration = Duration::from_millis(600);

fn print_usage() {
    println!("==================================================");
    println!("   🚀 HIGH-PERFORMANCE MULTI-THREADED PORT SCANNER");
    println!("==================================================");
    println!("Usage:   cargo run -- <IP_ADDRESS> <START_PORT> <END_PORT>");
    println!("Example: cargo run -- 127.0.0.1 1 1000\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle help or missing arguments
    if args.len() < 4 {
        print_usage();
        eprintln!("❌ Error: Missing required arguments.");
        return;
    }

    // 1. Parse Input Parameters Safely
    let target_ip: IpAddr = match args[1].parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("❌ Error: Invalid IP address format (e.g., 127.0.0.1).");
            return;
        }
    };

    let start_port: u16 = match args[2].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("❌ Error: Start port must be a number between 1 and 65535.");
            return;
        }
    };

    let end_port: u16 = match args[3].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("❌ Error: End port must be a number between 1 and 65535.");
            return;
        }
    };

    if start_port > end_port {
        eprintln!("❌ Error: Start port cannot be greater than End port.");
        return;
    }

    println!("\n🔍 Target Target:  {}", target_ip);
    println!("🔢 Port Range:    {} - {}", start_port, end_port);
    println!("⚡ Mode:          Asynchronous Multi-threaded");
    println!("--------------------------------------------------");

    let timer = Instant::now();
    let total_ports = (end_port - start_port + 1) as usize;

    // 2. Concurrency Channel Setup
    let (tx, rx) = channel();

    // 3. Spawn Thread Pool Dynamically
    for port in start_port..=end_port {
        let tx = tx.clone();
        thread::spawn(move || {
            scan_port(tx, target_ip, port);
        });
    }
    drop(tx); // Close the main transmitter

    // 4. Process Results and Draw a Live Progress Bar
    let mut open_ports = Vec::new();
    let mut processed_count = 0;

    for port in rx {
        processed_count += 1;
        if port != 0 {
            open_ports.push(port);
        }

        // Calculate and render a clean terminal progress bar
        let percent = (processed_count * 100) / total_ports;
        let progress_chars = percent / 4; // 25 characters max width
        let bar: String = std::iter::repeat("■").take(progress_chars).collect();
        let spaces: String = std::iter::repeat(" ").take(25 - progress_chars).collect();

        print!(
            "\r🔄 Scanning: [{}{}] {}% ({}/{})", 
            bar, spaces, percent, processed_count, total_ports
        );
        io::stdout().flush().unwrap();
    }

    // 5. Present the Final Report Nicely
    println!("\n--------------------------------------------------");
    println!("✨ Scan Finished in {:.2?}", timer.elapsed());
    
    if open_ports.is_empty() {
        println!("🔒 Status: All scanned ports are CLOSED.");
    } else {
        open_ports.sort();
        println!("🔓 Found {} OPEN port(s):", open_ports.len());
        for port in open_ports {
            println!("   └── [+] Port {:<5} -> OPEN", port);
        }
    }
    println!("==================================================\n");
}

fn scan_port(tx: Sender<u16>, target_ip: IpAddr, port: u16) {
    let socket_address = SocketAddr::new(target_ip, port);
    // If connection succeeds, return the port number. If it fails, return 0 (signals progress but port closed)
    if TcpStream::connect_timeout(&socket_address, TIMEOUT).is_ok() {
        let _ = tx.send(port);
    } else {
        let _ = tx.send(0);
    }
}