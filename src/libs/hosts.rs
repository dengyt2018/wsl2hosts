#![allow(unused, dead_code, unused_variables)]

pub mod parse_hosts {
    use crate::libs::wsl_cli::{get_ip, list_all};
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::{BufRead, BufReader, LineWriter, Lines, Write};
    use std::iter::Flatten;
    use std::path::Path;

    struct Hosts {
        comment: String,
        ip: String,
        distro: String,
        only_comment: bool,
    }

    //const HOSTS_FILE_PATH: &str = "C:/Windows/System32/drivers/etc/hosts";
    const HOSTS_FILE_PATH: &str = "E:/hosts";

    pub fn parse_hosts() {
        write_string_to_hosts();
    }

    fn serde_hosts_file() -> Vec<Hosts> {
        let path = Path::new(HOSTS_FILE_PATH);
        let hosts_file = File::open(path).expect("Open hosts file failed.");

        let mut hosts_string = String::new();

        let lines = io::BufReader::new(hosts_file).lines();
        let mut serde_hosts = Vec::new();
        for line in lines.flatten() {
            let mut hosts = Hosts {
                comment: "\n".to_string(),
                ip: "".to_string(),
                distro: "".to_string(),
                only_comment: false,
            };

            if line.starts_with('#') {
                hosts.comment = line;
                hosts.only_comment = true;
            } else {
                parse_hosts_line(&line, &mut hosts);
            }

            serde_hosts.append(&mut vec![hosts]);
        }

        serde_hosts
    }

    fn serde_hosts_to_string(serde_hosts: &Vec<Hosts>) -> String {
        let mut hosts_string = String::new();
        for host in serde_hosts {
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
        }
        hosts_string
    }

    fn parse_hosts_line(line: &str, hosts: &mut Hosts) {
        if line.contains(' ') {
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
        let mut hosts_serde = serde_hosts_file();

        let mut infos = list_all();

        for mut info in infos {
            get_ip(&mut info);

            let mut change = false;

            for mut host in &mut hosts_serde {
                if info.distro.eq_ignore_ascii_case(&host.distro) {
                    host.ip = info.ip.to_string();
                    host.comment = "# wsl2hosts".to_string();
                    change = true;
                }
            }
            if !change {
                hosts_serde.append(&mut vec![Hosts {
                    comment: "# wsl2hosts".to_string(),
                    ip: info.ip,
                    distro: info.distro.to_ascii_lowercase(),
                    only_comment: false,
                }]);
            }
        }

        let hosts_string = serde_hosts_to_string(&hosts_serde);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(Path::new(HOSTS_FILE_PATH))
            .expect("");

        let mut f = LineWriter::new(file);
        f.write_all(hosts_string.as_bytes());
        f.flush();
    }
}

#[cfg(test)]
mod parse_hosts_file_test {

    const HOSTS_FILE_PATH: &str = "E:/hosts";

    #[test]
    fn parse_test() {}
}
