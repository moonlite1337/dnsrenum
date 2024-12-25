use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "dnsrenum zonetransfer.me")]
pub struct Options {
    #[argh(positional, description = "host to get DNS records on")]
    pub host: String,
    #[argh(switch, description = "scrape first page of google subdomain search")]
    pub scrap: bool,
    #[argh(
        option,
        short = 'f',
        description = "a file with subdomains to perform brute force enumeration"
    )]
    pub dns: Option<String>,
}
