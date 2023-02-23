mod libs;

use crate::libs::wsl_cli::{get_ip, list_all};

#[allow(unused_imports, dead_code)]

fn main() {
    let infos = list_all();

    for mut distro in infos {
        println!("{:?}", get_ip(&mut distro));
    }
}
