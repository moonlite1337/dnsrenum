use argh;

mod info;
mod input;
mod requester;
mod scanner;
mod stdout;

use input::Options;
use scanner::Scanner;
use stdout::print_records;

fn main() {
    let opts: Options = argh::from_env();

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
    requester::transfer_zones(opts.host.clone(), ns_domains);
}
