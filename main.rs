use std::net::{TcpStream};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::*;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("1 argument needs to be passed in, the target ip or --version,
        and optionally the starting port and ending port.");
        return;
    }
    if args.get(1).unwrap() == "--version" {
        let v = env!("CARGO_PKG_VERSION");
        println!("{}", v);
        return;
    }
    let mut start_port: i64 = 0;
    let mut end_port: i64 = 1000;
    if let Some(_e) = args.get(2) {
        if let Ok(_e) = args[2].parse::<i64>() {
            start_port = args[2].parse::<i64>().unwrap();
        }
    }
    if let Some(_e) = args.get(3) {
        if let Ok(_e) = args[3].parse::<i64>() {
            end_port = args[3].parse::<i64>().unwrap();
        } 
    }
    let mut joins = Vec::new();
    let ports: Vec<i64> = Vec::new();
    let ports = Arc::new(Mutex::new(ports));
    for i in start_port..end_port {
        let inpt = Arc::new(Mutex::new(args[1].clone()));
        let i_lock = Arc::new(Mutex::new(i));
        let port_clone = Arc::clone(&ports);
        let thread = thread::spawn(move || {
            let inp = inpt.lock().unwrap();
            let lock_acc = i_lock.lock().unwrap();
            let port = format!("{lock_acc}");
            let mut server_port_fmt = inp.clone() + ":";
            server_port_fmt += &port.to_string();

            println!("{}", server_port_fmt);
            let final_input = server_port_fmt.clone();
            println!("Checking {}...", final_input);

            if let Ok(_e) = TcpStream::connect(final_input.clone()) {
                println!("Port {} is open!", lock_acc);
                println!("Addr: {}", final_input);
                let mut access = port_clone.lock().unwrap();
                access.push(i);
            } else if let Err(e) = TcpStream::connect(final_input.clone()) {
                println!("Port {} is not open.", lock_acc);
                println!("Error: {}", e);
                println!("Addr: {}", final_input);
            }
        });
        joins.push(thread);
    }
    let port_clone = Arc::clone(&ports);
    for join in joins {
        join.join().unwrap();
    }
    {
        println!("Summary: ");
        let info_access = port_clone.lock().unwrap();
        println!("Open Ports: {}", info_access.len());
        println!("Ports: ");
        for port in 0..info_access.len() {
            println!("{}", info_access.get(port).unwrap());
        }
        let end = start.elapsed().as_secs_f32();
        println!("Elapsed: {}s", end);
    }
}   
