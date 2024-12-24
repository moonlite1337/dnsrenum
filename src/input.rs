use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "dnsrenum google.com")]
pub struct Options {
    #[argh(positional, description = "host to get DNS records on")]
    pub host: String,
    #[argh(
        option,
        default = "warn",
        short = "l",
        description = "log level, options: off, error, warn, info, debug, trace"
    )]
    pub log_level: String,
}
