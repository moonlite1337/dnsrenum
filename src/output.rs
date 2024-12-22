#[macro_export]
macro_rules! print_record {
    ($r:expr, &Record) => {
        println!("{0}\t{1}\t{2}\t{3}\t{4}", $r.name(), $r.ttl(), $r.dns_class(), $r.record_type(), $r.data().unwrap());
    };
}