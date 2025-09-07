#![allow(unused)]
use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

const MAX: u16 = 65535; // max port number

const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)); // localhost as fallback

// CLI arguments
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    // Address argument.  Accepts -a and --address and an IpAddr type. Falls back to the above constant.
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    /// The address that you want to sniff.  Must be a valid ipv4 address.  Falls back to 127.0.0.1
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0"),
        fallback(1u16)
    )]
    /// The start port for the sniffer. (must be greater than 0)
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than or equal to 65535"),
        fallback(MAX)
    )]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    pub end_port: u16,
}


fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}


#[tokio::main]
async fn main() {
    let opts = arguments().run();

    let (tx, rx) = channel();
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move {
            scan(tx, i, opts.address).await;
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}





// Old implementation without bpaf below


// impl Arguments {
//     fn new(args: &[String]) -> Result<Arguments, &'static str> {
//         if args.len() < 2 {
//             return Err("Not enough arguments");
//         } else if args.len() > 4 {
//             return Err("Too many arguments");
//         }
//         let f = args[1].clone();
//         if let Ok(ipaddr) = IpAddr::from_str(&f) {
//             return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4});
//         } else {
//             let flag = args[1].clone();
//             if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
//                 println!("Usage: -j to select how many threads to use (default 4). 
//                 \n-h or --help to see this message");
//                 return Err("help");
//             } else if flag.contains("-h") || flag.contains("--help") {
//                 return Err("Too many arguments");
//             } else if flag.contains("-j") {
//                 let ipaddr = match IpAddr::from_str(&args[3]) {
//                     Ok(s) => s,
//                     Err(_) => return Err("Not a valid IP address; must by IPv4 or IPv6"),
//                 };
//                 let threads = match args[2].parse::<u16>() {
//                     Ok(s) => s,
//                     Err(_) => return Err("Failed to pasrse number of threads")
//                 };
//                 return Ok(Arguments {threads, flag, ipaddr});
//             } else {
//                 return Err("Invalid flag/syntax; use -h or --help for usage");
//             }
//         }
//     }   
// }