use super::config;
use daemonize::Daemonize;
use error_chain::error_chain;
use log::info;

error_chain! {
    foreign_links {
        DaemonizeError(daemonize::DaemonizeError);
    }
}

pub fn daemonize(config: &config::Daemonize) -> Result<()> {
    let mut daemonize = Daemonize::new()
        .umask(config.umask)
        .privileged_action(|| info!("Daemonize ..."));
    if let Some(ref pidfile_path) = config.pidfile_path {
        daemonize = daemonize.pid_file(pidfile_path).chown_pid_file(true);
    }
    if let Some(ref directory) = config.directory {
        daemonize = daemonize.working_directory(directory);
    }
    if let Some(uid) = config.uid {
        daemonize = daemonize.user(uid)
    }
    if let Some(gid) = config.gid {
        daemonize = daemonize.group(gid)
    }
    daemonize.start()?;
    Ok(())
}
