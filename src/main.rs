use error_chain::error_chain;
use error_chain::quick_main;
use log::info;

mod config;
mod daemonize_utility;
mod flags;
mod logging;

use daemonize_utility::daemonize;

error_chain! {
    links {
        ConfigError(config::Error, config::ErrorKind);
        LoggerConfigError(logging::Error, logging::ErrorKind);
        DaemonizeConfigError(daemonize_utility::Error, daemonize_utility::ErrorKind);
    }
}

quick_main!(|| -> Result<()> {
    let flags = flags::parse_flags();
    let config = config::parse_file(flags.config_file_path)?;
    logging::init_logging(&config.logging)?;
    if let Some(ref daemonize_config) = config.daemonize {
        daemonize(daemonize_config)?;
    }
    info!("{:?}", config);
    Ok(())
});
