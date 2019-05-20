use error_chain::error_chain;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub logging: Logging,
    pub process: Process,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    #[serde(default = "server::default_daemonize")]
    pub daemonize: bool,
    #[serde(default = "server::default_host")]
    pub bind_host: String,
    #[serde(default = "server::default_port")]
    pub bind_port: String,
    pub pidfile_path: Option<String>,
    pub directory: Option<String>,
    pub gid: Option<u32>,
    pub uid: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Logging {
    #[serde(default = "logging::default_log_level")]
    pub log_level: String,
    #[serde(default = "logging::default_log_outputs")]
    pub log_outputs: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Process {
    pub name: Option<String>,
    pub shell: Option<String>,
    pub command: Option<Vec<String>>,
    pub directory: Option<String>,
    pub umask: Option<i32>,
    pub environment: Option<HashMap<String, String>>,
    pub stdout_logfile: Option<String>,
    pub stderr_logfile: Option<String>,
    #[serde(default = "process::default_bool")]
    pub redirect_stderr: bool,
    #[serde(default = "process::default_bool")]
    pub auto_start: bool,
    #[serde(default = "process::default_bool")]
    pub auto_restart: bool,
    #[serde(default = "process::default_exit_codes")]
    pub exit_codes: Vec<u8>,
    #[serde(default = "process::default_stop_signal")]
    pub stop_signal: i32,
    #[serde(default = "process::default_stop_wait_seconds")]
    pub stop_wait_seconds: u32,
    #[serde(default = "process::default_processes")]
    pub processes: u16,
    #[serde(default = "process::default_restarts")]
    pub restarts: u16,
}

error_chain! {
    foreign_links {
        ReadFileError(std::io::Error);
        ParseFileError(toml::de::Error);
    }
}

pub fn parse_file<P>(file_path: P) -> Result<Config>
where
    P: AsRef<Path>,
{
    let path = file_path.as_ref().to_path_buf();
    let source = read_config(&path).chain_err(|| "Failed to read config file")?;
    let config: Config = toml::from_str(&source).chain_err(|| "Failed to parse config file")?;
    Ok(config)
}

fn read_config(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path)
}

mod server {
    pub(super) fn default_daemonize() -> bool {
        false
    }

    pub(super) fn default_host() -> String {
        "0.0.0.0".to_string()
    }

    pub(super) fn default_port() -> String {
        "http".to_string()
    }
}

mod logging {
    pub(super) fn default_log_level() -> String {
        "DEBUG".to_string()
    }

    pub(super) fn default_log_outputs() -> Vec<String> {
        vec!["stderr".to_string()]
    }
}

mod process {
    pub(super) fn default_bool() -> bool {
        false
    }

    pub(super) fn default_exit_codes() -> Vec<u8> {
        vec![0]
    }

    pub(super) fn default_stop_signal() -> i32 {
        2
    }

    pub(super) fn default_stop_wait_seconds() -> u32 {
        10
    }

    pub(super) fn default_processes() -> u16 {
        1
    }

    pub(super) fn default_restarts() -> u16 {
        3
    }
}
