use hickory_client::rr::Record;

use colored::Colorize;

#[macro_export]
macro_rules! print_record {
    ($r:expr, &Record) => {
        println!(
            "{0}\t{1}\t{2}\t{3}\t{4}",
            $r.name(),
            $r.ttl(),
            $r.dns_class(),
            $r.record_type(),
            $r.data().unwrap()
        );
    };
}

pub fn print_records(title: &str, records: &[Record]) {
    println!("{}\n", title.red().underline());
    if records.is_empty() {
        println!("Hmm... no records found. \n");
        return;
    }

    records.iter().for_each(|r| {
        print_record!(r, &Record);
    });
    println!("\n");
}
