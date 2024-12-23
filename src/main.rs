mod input;
mod output;
mod scanner;

use crate::scanner::DNS;
use argh;
use colored::Colorize;
use hickory_client::client::{Client, SyncClient};
use hickory_client::rr::Name;
use hickory_client::udp::UdpClientConnection;
use hickory_resolver::Resolver;
use input::Options;

fn main() {
    let opts: Options = argh::from_env();
    let resolver = Resolver::from_system_conf().unwrap();

    // todo!("extract lookup into separate mod");
    let scanner = scanner::Scanner::new(opts.host, None).unwrap();
    let i = scanner.run().unwrap();
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
