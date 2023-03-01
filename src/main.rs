#![windows_subsystem = "windows"]

use crate::libs::hosts::parse_hosts::parse_hosts;
use crate::libs::scheduler::wsl_task_scheduler::{create_task_scheduler, task_scheduler};
use crate::libs::{Cli, Mode};
use clap::Parser;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use log::debug;

mod libs;

fn prepare_logging(out: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut exe_dir = std::env::current_exe()?;
    exe_dir.pop();

    Logger::try_with_env_or_str(out)?
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
    let args = Cli::parse();

    match args.mode {
        None => {
            prepare_logging("info").unwrap();
            loop_parse();
            debug!("launch without argument");
        }
        Some(mode) => match mode {
            Mode::Run => {
                task_scheduler(Mode::Run).unwrap();
                debug!("launch with argument RUN");
            }
            Mode::Install => {
                create_task_scheduler().unwrap();
                task_scheduler(Mode::Run).unwrap();
                debug!("launch with argument INSTALL");
            }
            Mode::Remove => {
                task_scheduler(Mode::Stop).unwrap();
                task_scheduler(Mode::Remove).unwrap();
                debug!("launch with argument REMOVE");
            }
            Mode::Stop => {
                task_scheduler(Mode::Stop).unwrap();
                debug!("launch with argument STOP");
            }
            Mode::Debug => {
                prepare_logging("debug").unwrap();
                debug!("launch with argument DEBUG");
                parse_hosts();
            }
        },
    };
}

fn loop_parse() {
    loop {
        parse_hosts();
    }
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
