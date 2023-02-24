pub mod wsl2hosts_command {
    use clap::{Parser, ValueEnum};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        #[arg(value_enum)]
        pub mode: Option<Mode>,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
    pub enum Mode {
        Run,
        Install,
        Remove,
    }
}
