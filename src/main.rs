use crate::libs::command::wsl2hosts_command::{Cli, Mode};
use crate::libs::hosts::parse_hosts::parse_hosts;
use clap::Parser;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use log::debug;

mod libs;

fn prepare_logging() -> Result<(), Box<dyn std::error::Error>> {
    let mut exe_dir = std::env::current_exe()?;
    exe_dir.pop();

    Logger::try_with_env_or_str("debug")?
        .log_to_file(FileSpec::default().directory(exe_dir).suppress_timestamp())
        .append()
        .rotate(
            Criterion::Size(1024 * 1024 * 2),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(2),
        )
        .duplicate_to_stderr(flexi_logger::Duplicate::Info)
        .format_for_files(|w, now, record| {
            write!(
                w,
                "{} [{}] {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                &record.args()
            )
        })
        .format_for_stderr(|w, now, record| {
            write!(
                w,
                "{} [{}] {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                &record.args()
            )
        })
        .start()?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn main() {
    debug!("********** LAUNCH **********");
    let args = Cli::parse();

    prepare_logging().unwrap();

    match args.mode {
        None => {
            debug!("launch without argument");
        }
        Some(mode) => match mode {
            Mode::Run => {
                debug!("launch with argument RUN");
            }
            Mode::Install => {
                debug!("launch with argument INSTALL");
            }
            Mode::Remove => {
                debug!("launch with argument REMOVE");
            }
            Mode::Stop => {
                debug!("launch with argument STOP");
                todo!()
            }
            Mode::Debug => {
                debug!("launch with argument DEBUG");
                parse_hosts();
            }
        },
    };
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
