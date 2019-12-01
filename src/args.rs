use humantime::parse_duration;
use std::{
    time::Duration,
    path::PathBuf,
};

#[derive(structopt::StructOpt, Debug)]
pub struct Args {
    #[structopt(flatten)]
    pub mqtt: MqttArgs,

    #[structopt(flatten)]
    pub httpd: HttpdArgs,

    #[structopt(short, long)]
    pub verbose: bool,
}

#[derive(structopt::StructOpt, Debug, Clone)]
pub struct MqttArgs {
    #[structopt(long, default_value = "127.0.0.1")]
    pub mqtt_host: String,

    #[structopt(long, default_value = "1883")]
    pub mqtt_port: u16,

    #[structopt(long, default_value = "10s", parse(try_from_str = parse_duration))]
    pub mqtt_keep_alive: Duration,

    #[structopt(long, default_value = "10s", parse(try_from_str = parse_duration))]
    pub mqtt_reconnect_after: Duration,
}

#[derive(structopt::StructOpt, Debug, Clone)]
pub struct HttpdArgs {
    #[structopt(long, default_value = "127.0.0.1")]
    pub httpd_host: String,

    #[structopt(long, default_value = "8888")]
    pub httpd_port: u16,

    #[structopt(long)]
    /// serve static assets from the given directory
    pub httpd_webapp_dir: Option<PathBuf>,
}
