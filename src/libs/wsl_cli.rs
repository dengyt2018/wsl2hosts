#![allow(unused, dead_code, unused_variables)]

use crate::libs::utils::decode_output;
use path_slash::{PathBufExt, PathExt};
use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct WSLInfo {
    pub distro: String,
    pub state: bool,
    pub version: u8,
    pub ip: String,
}

pub fn list_all() -> Vec<WSLInfo> {
    let stdout = Command::new("wsl")
        .args(["-l", "-v"])
        .output()
        .expect("execute wsl -l -v failed.");

    let output = decode_output(&stdout.stdout);

    let mut infos = Vec::new();

    for distro in output.lines().filter(|line| line.contains("Running")) {
        let part = distro.split(' ');
        let mut v = Vec::new();
        for c in part {
            if !c.is_empty() {
                if c.contains('*') {
                    continue;
                } else {
                    v.append(&mut vec![c]);
                }
            }
        }

        let version = v[2].to_string().parse::<u8>().unwrap();
        let info = WSLInfo {
            distro: v[0].to_string(),
            state: true,
            version,
            ip: String::from("127.0.0.1"),
        };
        infos.append(&mut vec![info]);
    }

    infos
}

fn create_script_file(name: &str) -> String {
    let path = String::from(std::env::temp_dir().to_slash().unwrap());
    let temp_dir = PathBuf::from(path);
    let full_temp_dir = temp_dir.join(name);

    let script = b"#!/usr/bin/sh\n
cat /proc/net/fib_trie | awk '/32 host/ { print f } {f=$2}' | tail -n 1";

    if !Path::exists(&full_temp_dir) {
        let file = File::create(&full_temp_dir).expect("Create script file failed.");

        let mut file = LineWriter::new(file);
        file.write_all(script);
        file.flush();
    }

    let mut path = String::from(full_temp_dir.to_str().unwrap());
    path.retain(|c| c != ':');

    path.to_ascii_lowercase()
}

pub fn get_ip(distro: &mut WSLInfo) -> &WSLInfo {
    if distro.version == 1 {
        return distro;
    }

    let script_name = "getip.sh";
    let path = create_script_file(script_name);

    let stdout = Command::new("wsl")
        .args(["-d", &distro.distro, "--", "sh", &format!("/mnt/{}", &path)])
        .output()
        .unwrap_or_else(|_| panic!("get wsl {} ip address failed", &distro.distro));

    let mut ip_address = decode_output(&stdout.stdout);
    ip_address.retain(|c| c != '\n');

    distro.ip = ip_address;

    distro
}
