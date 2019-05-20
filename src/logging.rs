use super::config;
use chrono;
use error_chain::error_chain;
use fern;
use log;

error_chain! {
    foreign_links {
        FernError(fern::InitError);
        LoggerPathError(std::io::Error);
    }

    errors {
        UnrecognizedLogLevel(level: String) {
            description("unrecoginized log level")
            display("unrecoginized log level: {}", level)
        }
    }
}

pub fn init_logging(config: &config::Logging) -> Result<()> {
    let mut dispatch = fern::Dispatch::new();
    dispatch = dispatch.format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.6f%:z]"),
            record.target(),
            record.level(),
            message
        ))
    });
    dispatch = dispatch.level(get_level(config)?);
    for output in get_outputs(config)? {
        dispatch = dispatch.chain(output);
    }
    dispatch.apply().map_err(|err| fern::InitError::from(err))?;
    Ok(())
}

fn get_level(config: &config::Logging) -> Result<log::LevelFilter> {
    match config.log_level.to_lowercase().as_ref() {
        "trace" => Ok(log::LevelFilter::Trace),
        "debug" => Ok(log::LevelFilter::Debug),
        "info" => Ok(log::LevelFilter::Info),
        "warn" => Ok(log::LevelFilter::Warn),
        "error" => Ok(log::LevelFilter::Error),
        _ => Err(ErrorKind::UnrecognizedLogLevel(config.log_level.clone()).into()),
    }
}

fn get_outputs(config: &config::Logging) -> Result<Vec<fern::Output>> {
    let mut outputs: Vec<fern::Output> = Vec::with_capacity(config.log_outputs.len());
    for output in &config.log_outputs {
        match output.as_ref() {
            "stdout" => outputs.push(std::io::stdout().into()),
            "stderr" => outputs.push(std::io::stderr().into()),
            _ => outputs.push(fern::log_file(output).map(|output| output.into())?),
        }
    }
    Ok(outputs)
}
