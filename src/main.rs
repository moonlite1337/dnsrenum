use std::thread;
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
        let s = Scanner::new(None).unwrap();
        let info = s.run(&opts.host).unwrap_or_else(|err| panic!("{}", err));
        print_records("Host addresses:", &info.ips);
        print_records("Name servers:", &info.ns);
        print_records("MX records:", &info.mx);
    }

    let ns = ["ns-884.awsdns-46.net".to_string()];
    let handle = thread::spawn(|| {
        requester::transfer_zones(&opts.host, ns.to_vec());
    });

    println!("waiting for the thread to finish..");
    handle.join().unwrap();
    println!("exiting..")
}
