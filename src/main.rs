use crate::libs::command::wsl2hosts_command::{Cli, Mode};
use crate::libs::hosts::parse_hosts::parse_hosts;
use crate::libs::service::wsl_hosts_service::{delete_service, install_service};
use clap::Parser;
use std::thread::sleep;
use std::time::Duration;

mod libs;

#[cfg(target_os = "windows")]
fn main() {
    let args = Cli::parse();

    match args.mode {
        None => run(),
        Some(mode) => match mode {
            Mode::Run => run(),
            Mode::Install => {
                install_service();
            }
            Mode::Remove => {
                delete_service();
            }
        },
    };
}

fn run() {
    loop {
        parse_hosts();

        sleep(Duration::from_secs(5));
    }
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
