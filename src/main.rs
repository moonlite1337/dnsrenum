mod input;
mod output;

use argh;
use colored::Colorize;
use hickory_client::client::AsyncClient;
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RecordType};
use hickory_client::udp::UdpClientStream;
use input::Options;
use std::net::{SocketAddr, UdpSocket};
use hickory_resolver::Resolver;

fn main() {
    let opts: Options = argh::from_env();
    let resolver = Resolver::from_system_conf().unwrap();

    // todo!("extract lookup into separate mod");
    resolver.lookup_ip(&opts.host).map_or_else(|e| {
        println!("\nno host addresses found. Exiting..");
        return;
    }, |l| {
        println!("{}", "Host addresses:".red().underline());
        l.as_lookup().records().iter().for_each(|r| {print_record!(r, &Record)});
    });

    let mut nameservers: Vec<&Name> = Vec::new();
    resolver.lookup(&opts.host, RecordType::NS).map_or_else(|e| {
        println!("\nno host addresses found. Exiting..");
        return;
    }, |l| {
        println!("\n{}", "Name servers:".red().underline());
        l.records().iter().for_each(|r| {
            nameservers.push(r.name());
            print_record!(r, &Record)
        });
    });

    resolver.lookup(&opts.host, RecordType::MX).map_or_else(|e| {
        println!("\nno host addresses found. Exiting..");
        return;
    }, |l| {
        println!("\n{}", "Mail (MX) servers:".red().underline());
        l.records().iter().for_each(|r| {print_record!(r, &Record)});
    });

    println!("\n{}\n", "Trying Zone Transfers and getting Bind Versions:".red().underline());
    // https://gokhnayisigi.medium.com/what-is-a-dns-zone-transfer-attack-and-how-to-test-it-12bdc52da086
    // use futures::executor::block_on;
    let nameserver: SocketAddr = "127.0.0.1:53".parse()?;
    let stream = UdpClientStream::<UdpSocket>::new(nameserver);
    // todo!("configure loop")
    let (client, bg) = AsyncClient::connect(stream).await?;
    let response: DnsResponse = client.zone_tranfer(
        &Name::from_utf8("example.com").unwrap(),
        DNSClass::IN,
        RecordType::AXFR,
    )?;
}
