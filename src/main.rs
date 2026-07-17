use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_millis(500);
// Fixed number of worker threads to optimize CPU core utilization
const MAX_WORKERS: usize = 64; 

fn print_usage() {
    println!("==================================================");
    println!("   🚀 ENTERPRISE CONCURRENT TCP PORT SCANNER");
    println!("==================================================");
    println!("Usage:   cargo run -- <IP_ADDRESS> <START_PORT> <END_PORT>");
    println!("Example: cargo run -- 127.0.0.1 1 1000\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        print_usage();
        eprintln!("❌ Error: Missing required arguments.");
        return;
    }

    let target_ip: IpAddr = match args[1].parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("❌ Error: Invalid IP address format.");
            return;
        }
    };

    let start_port: u16 = match args[2].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("❌ Error: Start port must be 1-65535.");
            return;
        }
    };

    let end_port: u16 = match args[3].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("❌ Error: End port must be 1-65535.");
            return;
        }
    };

    if start_port > end_port {
        eprintln!("❌ Error: Start port cannot be greater than End port.");
        return;
    }

    println!("\n🔍 Target Target:  {}", target_ip);
    println!("🔢 Port Range:    {} - {}", start_port, end_port);
    println!("⚡ Architecture:  Fixed Thread Pool Workflow ({} Workers)", MAX_WORKERS);
    println!("--------------------------------------------------");

    let timer = Instant::now();
    let total_ports = (end_port - start_port + 1) as usize;

    // 1. Thread-Safe Queue using Arc (Atomic Reference Counting) and Mutex
    // This allows multiple threads to safely pull the next port to scan
    let ports_queue: Vec<u16> = (start_port..=end_port).collect();
    let shared_queue = Arc::new(Mutex::new(ports_queue));

    // 2. Setup Results Channel (MPSC)
    let (tx, rx) = channel();
    let mut worker_handles = vec![];

    // 3. Spawn a Fixed Thread Pool
    for _ in 0..MAX_WORKERS {
        let queue_clone = Arc::clone(&shared_queue);
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            loop {
                // Safely lock the queue, pop a port, and release the lock immediately
                let port = {
                    let mut queue = queue_clone.lock().unwrap();
                    queue.pop()
                };

                match port {
                    Some(p) => scan_port(tx_clone.clone(), target_ip, p),
                    None => break, // Queue is empty, thread can exit safely
                }
            }
        });
        worker_handles.push(handle);
    }
    drop(tx); // Close original main transmitter instance

    // 4. Process Results & UI Rendering Loop
    let mut open_ports = Vec::new();
    let mut processed_count = 0;

    for port in rx {
        processed_count += 1;
        if port != 0 {
            open_ports.push(port);
        }

        let percent = (processed_count * 100) / total_ports;
        let progress_chars = percent / 4; 
        let bar: String = std::iter::repeat("■").take(progress_chars).collect();
        let spaces: String = std::iter::repeat(" ").take(25 - progress_chars).collect();

        print!(
            "\r🔄 Auditing: [{}{}] {}% ({}/{})", 
            bar, spaces, percent, processed_count, total_ports
        );
        io::stdout().flush().unwrap();
    }

    // Wait for all worker threads to safely clean up
    for handle in worker_handles {
        let _ = handle.join();
    }

    // 5. Build Final Presentation Metrics
    println!("\n--------------------------------------------------");
    println!("✨ Analytics Completed in {:.2?}", timer.elapsed());
    
    if open_ports.is_empty() {
        println!("🔒 Status: Secure. All scanned ports are CLOSED.");
    } else {
        open_ports.sort();
        println!("🔓 Discovered {} Open Socket Network Services:", open_ports.len());
        for port in open_ports {
            let service_guess = match port {
                22 => "SSH",
                80 => "HTTP",
                443 => "HTTPS",
                8080 => "Alternative HTTP",
                _ => "Unknown Service",
            };
            println!("   └── [+] Port {:<5} -> OPEN ({})", port, service_guess);
        }
    }
    println!("==================================================\n");
}

fn scan_port(tx: Sender<u16>, target_ip: IpAddr, port: u16) {
    let socket_address = SocketAddr::new(target_ip, port);
    if TcpStream::connect_timeout(&socket_address, TIMEOUT).is_ok() {
        let _ = tx.send(port);
    } else {
        let _ = tx.send(0);
    }
}