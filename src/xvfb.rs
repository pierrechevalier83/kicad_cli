use std::process::{self, Command, Stdio};

use log::{debug, trace};

pub struct Xvfb {
    process: process::Child,
}

impl Xvfb {
    pub fn run(port: u8) -> Result<Self, String> {
        let port = format!(":{}", port);
        debug!("Spawning Xvbp on port {}", port);
        let process = Command::new("Xvfb")
            .args(&[&port, "-ac", "-nolisten", "tcp"])
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to run xvfb: {}", e))?;
        trace!("Xvfb process was spawned successfully");
        std::env::set_var("DISPLAY", &port);
        Ok(Self { process })
    }
}
impl Drop for Xvfb {
    fn drop(&mut self) {
        std::env::remove_var("DISPLAY");
        if self.process.kill().is_ok() {
            debug!("Killed Xvfb");
        }
    }
}
