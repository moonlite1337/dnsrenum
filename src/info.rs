use hickory_client::rr::Record;

#[derive(Debug)]
pub struct Info {
    pub ns: Vec<Record>,
    pub ips: Vec<Record>,
    pub mx: Vec<Record>,
}
