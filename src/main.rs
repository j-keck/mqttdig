use env_logger::Env;
use log::error;

#[paw::main]
fn main(args: mqttdig::Args) {
    init_logger(&args);

    if let Err(err) = run(args) {
        error!("Error: {}", err);
    }
}

fn run(args: mqttdig::Args) -> mqttdig::Result<()> {
    let mut mqtt = mqttdig::Mqtt::connect(args.mqtt.clone())?;

    mqtt.subscribe("#")?;
    //mqtt.subscribe("$SYS/#")?;

    mqttdig::httpd::start(args.httpd.clone(), mqtt)?;
    Ok(())
}

fn init_logger(args: &mqttdig::Args) {
    let filter = {
        let level = if args.verbose { "debug" } else { "info" };
        format!("{}={}", env!("CARGO_PKG_NAME"), level)
    };

    env_logger::from_env(Env::default().default_filter_or(filter)).init();
}
