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

    // todo!("revert")
    if (opts.host != "zonetransfer.me") {
        let s = Scanner::new(None).unwrap();
        let info = s.run(&opts.host).unwrap_or_else(|err| panic!("{}", err));
        print_records("Host addresses:", &info.ips);
        print_records("Name servers:", &info.ns);
        print_records("MX records:", &info.mx);
    }

    let ns = ["ns-884.awsdns-46.net".to_string()];
    requester::transfer_zones(opts.host.clone(), ns.to_vec());
    // let handle = thread::spawn(move || {
    //     requester::transfer_zones(opts.host.clone(), ns.to_vec());
    // });
    //
    // println!("waiting for the thread to finish..");
    // handle.join().unwrap();
    println!("exiting..")
}
