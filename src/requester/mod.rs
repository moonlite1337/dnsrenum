use std::thread;
use colored::Colorize;
use hickory_client::client::{AsyncClient, Client, SyncClient};
use hickory_client::rr::Name;
use hickory_client::udp::UdpClientConnection;

pub fn transfer_zones(domain: &String, ns: Vec<String>) {
    // make sure this request does not block
    // todo!("reuse logger with verbosity levels")
    println!(
        "\n{}\n",
        "Trying Zone Transfers and getting Bind Versions:"
            .red()
            .underline()
    );
    // https://gokhnayisigi.medium.com/what-is-a-dns-zone-transfer-attack-and-how-to-test-it-12bdc52da086

    let handle = ns.map(|ns| {
        thread::spawn(move || {
            let address = ns.parse().unwrap();
            let conn = UdpClientConnection::new(address).unwrap();
            let client = SyncClient::new(conn);
            client.zone_transfer(&Name::from_utf8(domain).unwrap(), None)
        })
    });

    handle.join().unwrap();
}
