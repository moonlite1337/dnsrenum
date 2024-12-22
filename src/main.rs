mod input;
mod output;

use std::fmt::Debug;
use argh;
use colored::Colorize;
use input::Options;
use trust_dns_resolver::Resolver;

fn main() {
    let opts: Options = argh::from_env();
    println!("{}", opts.host);

    // host addresses
    let resolver = Resolver::from_system_conf().unwrap();
    let lookup = resolver.lookup_ip(opts.host).unwrap();
    println!("{}", "Host addresses:".red().underline().red().underline());

    for r in lookup.as_lookup().records() {
        printip!(r.name(), r.ttl(), r.dns_class(), r.record_type(), r.data().unwrap())
    }

    // name servers
    
}
