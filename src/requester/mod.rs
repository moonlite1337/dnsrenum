use colored::Colorize;
use hickory_client::client::{Client, SyncClient};
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_client::udp::UdpClientConnection;
use std::net::ToSocketAddrs;
use std::thread;

// send AXFR query to every nameserver for given domain
// https://gokhnayisigi.medium.com/what-is-a-dns-zone-transfer-attack-and-how-to-test-it-12bdc52da086
pub fn transfer_zones(domain: &str, ns: Vec<String>) {
    // make sure this request does not block
    println!(
        "\n{}\n",
        "Trying Zone Transfers and getting Bind Versions:"
            .red()
            .underline()
    );

    let threads = ns.into_iter().map(|ns| {
        let name = Name::from_utf8(domain).unwrap();
        thread::spawn(move || {
            let addr = format!("{ns}:53");
            let address = addr.to_socket_addrs().unwrap().next().unwrap();
            let conn = UdpClientConnection::new(address).unwrap();
            let client = SyncClient::new(conn);
            let r = client.query(&name, DNSClass::IN, RecordType::AXFR).unwrap();
            println!(
                "{}, for: {} - {}",
                r.query().unwrap(),
                addr,
                r.header().response_code()
            );
        })
    });
    for handle in threads {
        handle.join().unwrap();
    }
}
