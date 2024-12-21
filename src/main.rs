mod input;
use argh;
use colored::Colorize;
use input::Options;
use trust_dns_resolver::Resolver;

fn main() {
    let opts: Options = argh::from_env();
    println!("{}", opts.host);
    // let mut resolver = Resolver::from_system_conf().unwrap();
    // let mut response = resolver.lookup_ip("www.example.com.").unwrap();
    println!("{}", "Host addresses:".red().underline());
}
