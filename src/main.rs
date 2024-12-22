mod input;
mod output;

use argh;
use colored::Colorize;
use trust_dns_resolver::proto::rr::RecordType;
use input::Options;
use trust_dns_resolver::Resolver;

fn main() {
    let opts: Options = argh::from_env();
    let resolver = Resolver::from_system_conf().unwrap();

    resolver.lookup_ip(&opts.host).map_or_else(|e| {
        println!("\nno host addresses found. Exiting..");
        return;
    }, |l| {
        println!("{}", "Host addresses:".red().underline().red().underline());
        l.as_lookup().records().iter().for_each(|r| {print_record!(r, &Record)});
    });

    resolver.lookup(&opts.host, RecordType::NS).map_or_else(|e| {
        println!("\nno host addresses found. Exiting..");
        return;
    }, |l| {
        println!("\n{}", "Name servers:".red().underline().red().underline());
        l.records().iter().for_each(|r| {print_record!(r, &Record)});
    });

    resolver.lookup(&opts.host, RecordType::MX).map_or_else(|e| {
        println!("\nno host addresses found. Exiting..");
        return;
    }, |l| {
        println!("\n{}", "Mail (MX) servers:".red().underline().red().underline());
        l.records().iter().for_each(|r| {print_record!(r, &Record)});
    });

}
