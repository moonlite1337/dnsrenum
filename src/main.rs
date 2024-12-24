use argh;
use colored::Colorize;

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
    // todo!("revert");
    if (opts.host != "google.com") {
        let s = Scanner::new(opts.host, None).unwrap();
        let info = s.run().unwrap_or_else(|err| panic!("{}", err));
        print_records("Host addresses:", &info.ips);
        print_records("Name servers:", &info.ns);
        print_records("MX records:", &info.mx);
    }

    requester::transfer_attempt();

    println!("Exiting..")
}
