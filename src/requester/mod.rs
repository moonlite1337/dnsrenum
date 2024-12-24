use colored::Colorize;
use hickory_client::client::AsyncClient;
use hickory_client::udp::UdpClientConnection;
use hickory_client::{Client, UdpTransport};

pub fn transfer_zones() {
    // make sure this request does not block
    // todo!("reuse logger with verbosity levels")
    println!(
        "\n{}\n",
        "Trying Zone Transfers and getting Bind Versions:"
            .red()
            .underline()
    );
    // https://gokhnayisigi.medium.com/what-is-a-dns-zone-transfer-attack-and-how-to-test-it-12bdc52da086
    // use futures::executor::block_on;
    // let tasks = info.ns.iter().for_each(|s| println!("{}", s.name()));

    // let address = "8.8.8.8:53".parse().unwrap();
    // let conn = UdpClientConnection::new(address).unwrap();
    // // and then create the Client
    // let client = SyncClient::new(conn);
    // let response = client
    //     .zone_transfer(&Name::from_utf8("example.com").unwrap(), None)
    //     .unwrap();

    let address = "ns-884.awsdns-46.net".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = AsyncClient::new(conn);
}
