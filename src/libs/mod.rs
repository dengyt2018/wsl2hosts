pub mod hosts;
pub mod scheduler;
pub mod wsl_cli;

use chrono::Utc;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub(crate) mode: Option<Mode>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Run,
    Install,
    Remove,
    Stop,
    Debug,
}

pub fn decode_output(output: &[u8]) -> String {
    let s = String::from_utf8_lossy(output);

    let mut s = s.chars().collect::<Vec<char>>();

    s.retain(|c| *c != '\0');

    s.into_iter().collect::<String>()
}

pub fn time_rfc3339() -> String {
    let utc_now = Utc::now();
    utc_now.to_rfc3339()
}
