use std::fs::File;

mod brute;
mod info;
mod input;
mod requester;
mod scanner;
mod scrape;
mod stdout;

use input::Options;
use scanner::Scanner;
use stdout::print_records;

// #	1) Get the host's addresse.
// #	2) Get the nameservers (threaded).
// #	3) get the MX record (threaded).
// #	4) Perform axfr queries on nameservers (threaded).
// #	5) Get extra names via google scraping.
// #	6) Brute force subdomains from file (threaded).
// #	7) Calculate C class domain network ranges and perform whois
// #		queries on them (threaded).
// #	8) Perform reverse lookups on C class or/and whois
// #		network ranges (threaded).
// #	9) Write to domain_ips.txt file non-contiguous ip-blocks results.
fn main() {
    let opts: Options = argh::from_env();

    // test
    match File::open(opts.dns.unwrap()) {
        Ok(f) => {
            brute::enumerate(f);
        }
        Err(e) => {
            println!("\n could not open the file: {}", e);
        }
    };

    let s = Scanner::new(None).unwrap();
    let info = s.run(&opts.host).unwrap_or_else(|err| panic!("{}", err));
    print_records("Host addresses:", &info.ips);
    print_records("Name servers:", &info.ns);
    print_records("MX records:", &info.mx);

    let ns_domains = info
        .ns
        .iter()
        .map(|x| {
            let mut name = x.data().unwrap().to_string();
            // ignore the trailing dot
            name.truncate(name.len() - 1);
            name
        })
        .collect();
    requester::transfer_zones(&opts.host, ns_domains);

    if opts.scrap {
        scrape::google(&opts.host).ok();
    }

    // if (opts.dns.is_none()) {
    //     println!("\ndns file not provided, skipping brute force part..");
    //     return;
    // }
    //
    // match File::open(opts.dns.unwrap()) {
    //     Ok(f) => {
    //         brute::enumerate(f);
    //     }
    //     Err(e) => {
    //         println!("\n could not open the file: {}", e);
    //     }
    // };

    //
    // #	6) Brute force subdomains from file (threaded).
    // #	7) Calculate C class domain network ranges and perform whois
    // #		queries on them (threaded).
    // #	8) Perform reverse lookups on C class or/and whois
    // #		network ranges (threaded).
    // #	9) Write to domain_ips.txt file non-contiguous ip-blocks results.
}
