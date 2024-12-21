use argh::FromArgs;

// dnsrenum options
#[derive(FromArgs)]
#[argh(description = "dnsrenum google.com")]
pub struct Options {
    #[argh(positional, description = "host to get DNS records on")]
    pub host: String,
}
