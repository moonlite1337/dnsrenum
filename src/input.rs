use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "dnsrenum zonetransfer.me")]
pub struct Options {
    #[argh(positional, description = "host to get DNS records on")]
    pub host: String,
}
