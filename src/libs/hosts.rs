#![allow(unused, dead_code, unused_variables)]

pub mod parse_hosts {
    use crate::libs::wsl_cli::{get_ip, list_all_running_wsl};
    use log::{debug, error, info};
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::{BufRead, BufReader, LineWriter, Lines, Write};
    use std::iter::Flatten;
    use std::path::Path;
    use std::thread::sleep;
    use std::time::Duration;

    #[derive(Debug)]
    struct Hosts {
        comment: String,
        ip: String,
        distro: String,
        only_comment: bool,
    }

    impl Default for Hosts {
        fn default() -> Self {
            Hosts {
                comment: "".to_string(),
                ip: "127.0.0.1".to_string(),
                distro: "".to_string(),
                only_comment: false,
            }
        }
    }

    const HOSTS_FILE_PATH: &str = "C:/Windows/System32/drivers/etc/hosts";

    pub fn parse_hosts() {
        let seconds = 5;
        write_string_to_hosts();
        sleep(Duration::from_secs(seconds));

        debug!(
            "\n\n===============================================================\n\
        =====================write string to hosts=====================\n\
        ==============================================================="
        );
        debug!("sleep {} seconds", seconds);
    }

    fn serde_hosts_file() -> Vec<Hosts> {
        let hosts_file_path = Path::new(HOSTS_FILE_PATH);
        debug!(
            "Start serde hosts file path: {}",
            &hosts_file_path.to_str().unwrap()
        );

        match File::open(hosts_file_path) {
            Ok(hosts_file) => {
                let mut hosts_string = String::new();
                let lines = io::BufReader::new(hosts_file).lines();
                let mut serde_hosts = Vec::new();
                for line in lines.flatten() {
                    let mut hosts = Hosts::default();

                    if line.starts_with('#') {
                        hosts.comment = line;
                        hosts.only_comment = true;
                    } else {
                        parse_hosts_line(&line, &mut hosts);
                    }

                    serde_hosts.append(&mut vec![hosts]);
                }
                debug!("Serde hosts file done");

                serde_hosts
            }
            Err(e) => {
                error!("Open hosts file error {}", e);
                vec![Hosts::default()]
            }
        }
    }

    fn serde_hosts_to_string(serde_hosts: &Vec<Hosts>) -> String {
        debug!("Start serde hosts to string");

        let mut hosts_string = String::new();
        for host in serde_hosts {
            if host.distro.is_empty() && host.comment.is_empty() {
                continue;
            }
            if host.only_comment {
                hosts_string.push_str(host.comment.as_str());
            } else {
                hosts_string.push_str(host.ip.as_str());
                hosts_string.push(' ');
                hosts_string.push_str(host.distro.to_ascii_lowercase().as_str());

                if !host.comment.is_empty() && host.comment.starts_with('#') {
                    hosts_string.push(' ');
                    hosts_string.push_str(host.comment.as_str());
                }
            }
            hosts_string.push('\n');
            debug!("Hosts Info to string done {:?}", &host);
        }

        hosts_string
    }

    fn parse_hosts_line(line: &str, hosts: &mut Hosts) {
        // '::1 A' a local host maybe like this, only five char
        if line.contains(' ') && line.len() >= 5 {
            let host_split = line.split(' ');
            let mut host_vec = Vec::new();
            for info in host_split {
                if !info.is_empty() {
                    host_vec.append(&mut vec![info]);
                }
            }

            if host_vec.len() >= 2 {
                hosts.ip = host_vec[0].to_string();
                hosts.distro = host_vec[1].to_string().to_ascii_lowercase();
            }
            if host_vec.len() == 3 {
                hosts.comment = host_vec[2].to_string();
            }
        }
    }

    fn write_string_to_hosts() {
        debug!("Start write WSL host information to hosts file");
        let mut hosts_serde = serde_hosts_file();

        let mut all_wsl_infos = list_all_running_wsl();

        for mut wsl_info in all_wsl_infos {
            let mut change = false;

            for mut host in &mut hosts_serde {
                if wsl_info.distro.eq_ignore_ascii_case(&host.distro) {
                    host.ip = wsl_info.ip.to_string();
                    host.comment = "# wsl2hosts".to_string();
                    change = true;
                }
            }
            if !change {
                hosts_serde.append(&mut vec![Hosts {
                    comment: "# wsl2hosts".to_string(),
                    ip: wsl_info.ip,
                    distro: wsl_info.distro.to_ascii_lowercase(),
                    only_comment: false,
                }]);
            }
        }

        let hosts_string = serde_hosts_to_string(&hosts_serde);

        match OpenOptions::new()
            .read(true)
            .write(true)
            .open(Path::new(HOSTS_FILE_PATH))
        {
            Ok(file) => {
                let mut f = LineWriter::new(file);
                f.write_all(hosts_string.as_bytes());
                f.flush();
                debug!("All new hosts info write to hosts done");
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}
