use error_chain::error_chain;
use error_chain::quick_main;
use log::info;

mod config;
mod flags;
mod logging;

error_chain! {
    links {
        ConfigError(config::Error, config::ErrorKind);
        LoggerConfigError(logging::Error, logging::ErrorKind);
    }
}

quick_main!(|| -> Result<()> {
    let flags = flags::parse_flags();
    let config = config::parse_file(flags.config_file_path)?;
    logging::init_logging(&config.logging)?;
    info!("hello world");
    Ok(())
});
