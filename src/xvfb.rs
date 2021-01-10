use std::io::Read;
use std::process::{self, Command, Stdio};

pub struct Xvfb {
    process: process::Child,
}

impl Xvfb {
    pub fn run(port: u8) -> Result<Self, String> {
        let port = format!(":{}", port);
        let process = Command::new("Xvfb")
            .args(&[&port, "-ac", "-nolisten", "tcp"])
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to run xvfb: {}", e))?;
        std::env::set_var("DISPLAY", &port);
        Ok(Self { process })
    }
    pub fn dump_stderr(&mut self) -> String {
        let mut buffer = String::new();
        let mut out = self.process.stderr.take().unwrap();
        out.read_to_string(&mut buffer).unwrap();
        buffer
    }
}
impl Drop for Xvfb {
    fn drop(&mut self) {
        std::env::remove_var("DISPLAY");
        self.process.kill().expect("Failed to kill xvfb");
    }
}
