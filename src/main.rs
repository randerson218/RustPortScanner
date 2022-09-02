use std::{env, vec};
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

//Max port able to scan
const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    //&'static str used to pass errors
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            //return arguments struct if input is ok
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        } else {
            let flag = args[1].clone();
            //check for flags
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage:\n
                -j selects how many threads to use\n
                -h or -help displays this message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP Address; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } else {
                return Err("Invalid syntax");
            }
        } 
        
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port = start_port + 1;

    loop {
        match TcpStream::connect((addr,port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        //protect program from overshooting port max
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    //get all args passed
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            }
            else {
                //displays the errors above
                eprintln!("{} problem parsing arguments {}",program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx,rx) = channel();

    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx , i ,addr, num_threads);
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
        println!("{} is open",v);
    }

}
