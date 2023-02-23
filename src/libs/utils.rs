#![allow(unused, dead_code, unused_variables)]

pub fn decode_output(output: &[u8]) -> String {
    let s = String::from_utf8_lossy(output);

    let mut s = s.chars().collect::<Vec<char>>();

    s.retain(|c| *c != '\0');

    s.into_iter().collect::<String>()
}
