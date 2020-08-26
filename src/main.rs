use std::env;
use std::net::{TcpStream};

fn is_port_open(ip: &str, port: i32) -> bool {
    match TcpStream::connect(format!("{}:{}", ip, port)) {
        Ok(stream) => {
            let mut data = [0 as u8; 6];
            match stream.read_exact(&mut data) {
                Ok(_) => {

                },
                Err(e) => {
                    return false;
                }
            }
            true
        },
        Err(e) => {
            return false;
        }, 
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 0 || args.len() > 4 {
        println!("Usage: port-scanner IP-Address [starting-Port] [ending-Port]");
    }

    let mut minport: i32 = 0;
    let mut maxport: i32 = 0;
    
    let ip = &args[1];
    println!("{} {}", args[2], args[3]);

    if args.len() >= 3 {
        minport = args[2].parse::<i32>().unwrap();
    }
    
    if args.len() == 4 {
        maxport = args[3].parse::<i32>().unwrap();
    }

    for p in minport..maxport {
        println!("Checking Port: {} on Ip: {}", p, ip);
        match is_port_open(ip, p) {
            true => {
                println!("Port {} is open", p);
            },
            false => { },
        }
    }
}
