use colored::Colorize;
use hickory_client::client::{Client, SyncClient};
use hickory_client::rr::Name;
use hickory_client::udp::UdpClientConnection;
use std::thread;

pub fn transfer_zones(domain: String, ns: Vec<String>) {
    // make sure this request does not block
    // todo!("reuse logger with verbosity levels")
    println!(
        "\n{}\n",
        "Trying Zone Transfers and getting Bind Versions:"
            .red()
            .underline()
    );
    // https://gokhnayisigi.medium.com/what-is-a-dns-zone-transfer-attack-and-how-to-test-it-12bdc52da086

    let threads: Vec<_> = ns
        .into_iter()
        .map(|ns| {
            thread::spawn(|| {
                let address = ns.parse().unwrap();
                let conn = UdpClientConnection::new(address).unwrap();
                let client = SyncClient::new(conn);
                let stream = client.zone_transfer(&Name::from_utf8(domain).unwrap(), None);
                let response = stream.unwrap().next().unwrap().unwrap();
                response
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }
}
