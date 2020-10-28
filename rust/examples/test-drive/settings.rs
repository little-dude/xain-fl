use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Test Drive")]
pub struct Opt {
    #[structopt(
        default_value = "http://127.0.0.1:8081",
        short,
        help = "The URL of the coordinator"
    )]
    pub url: String,

    #[structopt(default_value = "4", short, help = "The length of the model")]
    pub len: u32,

    #[structopt(
        default_value = "1",
        short,
        help = "The time period at which to poll for service data, in seconds"
    )]
    pub period: u64,

    #[structopt(default_value = "10", short, help = "The number of clients")]
    pub nb_client: u32,

    #[cfg_attr(
        feature = "tls",
        structopt(
            short,
            long,
            parse(from_os_str),
            help = "The list of trusted DER/PEM encoded TLS server certificates"
        )
    )]
    pub certificates: Vec<PathBuf>,
}