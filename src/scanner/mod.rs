use crate::info::Info;
use hickory_client::rr::{Record, RecordType};
use hickory_resolver::error::ResolveError;
use hickory_resolver::Resolver;
use std::io;

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
    /// A basic wrapper around hickory.
    /// Possible enhancements:
    /// - accumulate and return IP, MX, NS records
    /// - move out printing into another module
    /// - throw proper errors
    pub fn run(&self) -> Result<Info, ResolveError> {
        let ips = self
            .resolver
            .lookup_ip(&self.host)?
            .as_lookup()
            .records()
            .to_vec();
        let ns = self
            .resolver
            .lookup(&self.host, RecordType::NS)?
            .records()
            .to_vec();
        // optional records;
        let mut mx: Vec<Record> = Vec::new();
        self.resolver
            .lookup(&self.host, RecordType::MX)
            .map(|l| {
                mx = l.records().to_vec();
            })
            .ok();

        Ok(Info { ips, ns, mx })
    }
}
