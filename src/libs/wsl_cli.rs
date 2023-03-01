#![allow(unused, dead_code, unused_variables)]

use crate::libs::decode_output;
use log::{debug, error, info, warn};
use path_slash::{PathBufExt, PathExt};
use std::fs::File;
use std::io::{stdout, LineWriter, Write};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct WSLInfo {
    pub distro: String,
    pub state: bool,
    pub version: u8,
    pub ip: String,
}

impl Default for WSLInfo {
    fn default() -> Self {
        WSLInfo {
            distro: "".to_string(),
            state: true,
            version: 2,
            ip: "127.0.0.1".to_string(),
        }
    }
}

const EXEC_WSL_PATH: &str = "C:\\Windows\\System32\\wsl.exe";
const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

pub fn list_all_running_wsl() -> Vec<WSLInfo> {
    debug!("Start command wsl -l -v list running wsl");

    let stdout = Command::new("cmd")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/c", "wsl", "-l", "-v"])
        .stdout(Stdio::piped())
        .output()
        .expect("execute wsl -l -v failed.");

    debug!("Raw stdout.status {}", &stdout.status);
    let e = decode_output(&stdout.stderr);
    if !e.is_empty() {
        error!("Raw stdout.stderr {:?}", e);
    }

    let output = decode_output(&stdout.stdout);
    debug!("Command wsl -l -v output message \n{}", &output);

    let mut infos = Vec::new();

    for wsl_output_line in output.lines().filter(|line| line.contains("Running")) {
        debug!("Parse wls -l -v Running Distros {}", wsl_output_line);

        let split_output = wsl_output_line.split(' ');
        let mut distro_info = Vec::new();
        for distro in split_output {
            if !distro.is_empty() {
                if distro.contains('*') {
                    continue;
                } else {
                    distro_info.append(&mut vec![distro.to_string()]);
                }
            }
        }

        let mut info = WSLInfo::default();

        if let Ok(v) = distro_info[2].parse::<u8>() {
            info.version = v;
        }
        info.distro = distro_info[0].to_owned();

        get_ip(&mut info);

        debug!("Get WSL Distro info {:?}", &info);
        infos.append(&mut vec![info]);
    }
    debug!("========= Running WSL Distro *{}* ==========", infos.len());
    debug!("End command wsl -l -v list running wsl");
    infos
}

fn create_script_file(name: &str) -> String {
    debug!("Start create getip.sh script file");

    let path = String::from(std::env::temp_dir().to_slash().unwrap());
    let temp_dir = PathBuf::from(path);
    let full_temp_dir = temp_dir.join(name);

    let script = b"#!/usr/bin/sh\n
cat /proc/net/fib_trie | awk '/32 host/ { print f } {f=$2}' | tail -n 1";

    if !Path::exists(&full_temp_dir) {
        debug!("The getip.sh does not exists, create it");
        match File::create(&full_temp_dir) {
            Ok(file) => {
                let mut file = LineWriter::new(file);
                file.write_all(script);
                file.flush();
                if Path::exists(&full_temp_dir) {
                    debug!("The getip.sh file create success {:?}", &full_temp_dir);
                } else {
                    error!("The getip.sh file create failed");
                }
            }
            Err(e) => {
                error!("The getip.sh file create failed {}", e);
            }
        }
    }

    let mut path = full_temp_dir.to_str().unwrap().to_ascii_lowercase();
    path.retain(|c| c != ':');

    debug!(
        "End create getip.sh script file return linux path {}",
        &path
    );

    path
}

pub fn get_ip(distro: &mut WSLInfo) -> &WSLInfo {
    if distro.version == 1 {
        debug!("WSL version is 1, use default ip 127.0.0.1");
        return distro;
    }

    let script_name = "getip.sh";
    let getip_script_file_path = create_script_file(script_name);

    match Command::new("cmd")
        .creation_flags(CREATE_NO_WINDOW)
        .args([
            "/c",
            "wsl",
            "-d",
            &distro.distro,
            "--",
            "sh",
            &format!("/mnt/{}", &getip_script_file_path),
        ])
        .stdout(Stdio::piped())
        .output()
    {
        Ok(stdout) => {
            let mut ip_address = decode_output(&stdout.stdout);
            ip_address.retain(|c| c != '\n');

            info!("Get WSL ip {}", &ip_address);
            let e = decode_output(&stdout.stderr);
            if !e.is_empty() {
                error!("Execute WSL command error {}", e);
            }

            distro.ip = ip_address;
        }
        Err(e) => {
            error!("Execute WSL command failed {}", e);
        }
    }
    distro
}
