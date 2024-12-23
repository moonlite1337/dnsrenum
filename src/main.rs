use argh;
use colored::Colorize;
use hickory_client::client::{Client, SyncClient};
use hickory_client::rr::Name;
use hickory_client::udp::UdpClientConnection;

mod input;
mod scanner;
mod stdout;

use input::Options;
use scanner::Scanner;
use stdout::print_records;

fn main() {
    let opts: Options = argh::from_env();
    let s = Scanner::new(opts.host, None).unwrap();
    let info = s.run().unwrap_or_else(|err| panic!("{}", err));
    print_records("Host addresses:", info.ips);
    print_records("Name servers:", info.ns);
    print_records("MX records:", info.mx);

    println!(
        "\n{}\n",
        "Trying Zone Transfers and getting Bind Versions:"
            .red()
            .underline()
    );
    // https://gokhnayisigi.medium.com/what-is-a-dns-zone-transfer-attack-and-how-to-test-it-12bdc52da086
    // use futures::executor::block_on;
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    // and then create the Client
    let client = SyncClient::new(conn);
    let response = client
        .zone_transfer(&Name::from_utf8("example.com").unwrap(), None)
        .unwrap();

    println!("Exiting..")
}
