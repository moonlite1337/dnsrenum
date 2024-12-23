use crate::print_record;
use colored::Colorize;
use hickory_client::rr::RecordType;
use hickory_resolver::Resolver;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct Info {
    pub nameservers: Vec<SocketAddr>,
}
pub trait DNS {
    fn run(&self) -> Result<Info, &'static str>;
}
pub struct Scanner {
    host: String,
    resolver: Resolver,
}

impl Scanner {
    pub fn new(host: String, r: Option<Resolver>) -> Result<Self, io::Error> {
        match r {
            Some(r) => Ok(Scanner { host, resolver: r }),
            None => {
                let r = Resolver::from_system_conf()?;
                Ok(Scanner { host, resolver: r })
            }
        }
    }
}

impl DNS for Scanner {
    /// A basic wrapper around hickory.
    /// Possible enhancements:
    /// - accumulate and return IP, MX, NS records
    /// - move out printing into another module
    /// - throw proper errors
    fn run(&self) -> Result<Info, &'static str> {
        self.resolver.lookup_ip(&self.host).map_or_else(
            |e| {
                println!("\nno host addresses found. Exiting..");
                return;
            },
            |l| {
                println!("{}", "Host addresses:".red().underline());
                l.as_lookup()
                    .records()
                    .iter()
                    .for_each(|r| print_record!(r, &Record));
            },
        );

        let mut nameservers: Vec<SocketAddr> = Vec::new();
        self.resolver
            .lookup(&self.host, RecordType::NS)
            .map_or_else(
                |e| {
                    println!("\nno name servers found. Exiting..");
                    return;
                },
                |l| {
                    println!("\n{}", "Name servers:".red().underline());
                    l.records().iter().for_each(|r| {
                        let ns = r.data().unwrap();
                        let addr = format!("{}:53", ns);
                        let s = addr.to_socket_addrs().unwrap().next().unwrap();
                        nameservers.push(s);
                        print_record!(r, &Record)
                    });
                },
            );

        self.resolver
            .lookup(&self.host, RecordType::MX)
            .map_or_else(
                |e| {
                    println!("\nno MX records found..");
                },
                |l| {
                    println!("\n{}", "Mail (MX) servers:".red().underline());
                    l.records().iter().for_each(|r| print_record!(r, &Record));
                },
            );

        Ok(Info { nameservers })
    }
}
