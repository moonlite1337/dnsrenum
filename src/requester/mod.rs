use std::net::ToSocketAddrs;
use colored::Colorize;
use hickory_client::client::{Client, SyncClient};
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_client::udp::UdpClientConnection;

pub fn transfer_zones(domain: String, ns: Vec<String>) {
    // make sure this request does not block
    println!(
        "\n{}\n",
        "Trying Zone Transfers and getting Bind Versions:"
            .red()
            .underline()
    );
    // https://gokhnayisigi.medium.com/what-is-a-dns-zone-transfer-attack-and-how-to-test-it-12bdc52da086

    // socket addr needs port
    let first = ns.first().unwrap();
    let addr = format!("{first}:53");
    let address = addr.to_socket_addrs().unwrap().next().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let r = client
        .query(
            &Name::from_utf8(domain).unwrap(),
            DNSClass::IN,
            RecordType::AXFR,
        )
        .unwrap();
    println!("{} - {}", r.query().unwrap(), r.header().response_code());
    // let threads: Vec<_> = ns
    //     .into_iter()
    //     .map(|ns| {
    //         thread::spawn(|| {
    //             let address = ns.parse().unwrap();
    //             let conn = UdpClientConnection::new(address).unwrap();
    //             let client = SyncClient::new(conn);
    //             let stream = client.zone_transfer(&Name::from_utf8(domain).unwrap(), None);
    //             let response = stream.unwrap().next().unwrap().unwrap();
    //             response
    //         })
    //     })
    //     .collect();
    //
    // for handle in threads {
    //     handle.join().unwrap();
    // }
}
