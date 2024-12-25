use crate::info::Info;
use hickory_client::rr::{Record, RecordType};
use hickory_resolver::error::ResolveError;
use hickory_resolver::Resolver;
use std::io;

pub struct Scanner {
    resolver: Resolver,
}

impl Scanner {
    pub fn new(r: Option<Resolver>) -> Result<Self, io::Error> {
        match r {
            Some(r) => Ok(Scanner { resolver: r }),
            None => {
                let r = Resolver::from_system_conf()?;
                Ok(Scanner { resolver: r })
            }
        }
    }
    /// A basic wrapper around hickory.
    /// Possible enhancements:
    /// - accumulate and return IP, MX, NS records
    /// - move out printing into another module
    /// - throw proper errors
    pub fn run(&self, host: &String) -> Result<Info, ResolveError> {
        let ips = self
            .resolver
            .lookup_ip(host)?
            .as_lookup()
            .records()
            .to_vec();
        let ns = self
            .resolver
            .lookup(host, RecordType::NS)?
            .records()
            .to_vec();
        // optional records;
        let mut mx: Vec<Record> = Vec::new();
        self.resolver
            .lookup(host, RecordType::MX)
            .map(|l| {
                mx = l.records().to_vec();
            })
            .ok();

        Ok(Info { ips, ns, mx })
    }
}
