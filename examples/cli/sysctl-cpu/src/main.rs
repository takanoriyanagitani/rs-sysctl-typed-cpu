use std::io::{self, Write};

use rs_sysctl_typed_cpu::CPUInfo;

fn info2json(i: &CPUInfo) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec_pretty(i)
}

fn bytes2stdout(b: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b)
}

fn main() {
    let cpu_info = CPUInfo::new();
    match info2json(&cpu_info) {
        Ok(json_bytes) => {
            if let Err(e) = bytes2stdout(&json_bytes) {
                eprintln!("Error writing to stdout: {e}");
            }
        }
        Err(e) => {
            eprintln!("Error serializing to JSON: {e}");
        }
    }
}
