use std::path;
use std::process::{self, Command, Stdio};

use log::{debug, trace};

pub struct Eeschema {
    process: process::Child,
}

impl Eeschema {
    fn check_schematic_file_looks_valid(p: &path::PathBuf) -> Result<(), String> {
        if !p.exists() {
            Err(format!(
                "Expected path to a schematic file. No such file: {:?}",
                p
            ))
        } else if p.extension() != Some(std::ffi::OsStr::new("sch")) {
            Err(format!(
                "Expected path to a schematic file. Extension should be `.sch`. It isn't: {:?}",
                p
            ))
        } else {
            Ok(())
        }
    }
    pub fn run(path_to_sch: &path::PathBuf) -> Result<Self, String> {
        Self::check_schematic_file_looks_valid(path_to_sch)?;
        trace!("Spawning eeschema");
        let process = Command::new("eeschema")
            .arg(path_to_sch)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to run eeschema: {}", e))?;
        debug!("Spawned eeschema");
        Ok(Self { process })
    }
}
impl Drop for Eeschema {
    fn drop(&mut self) {
        if self.process.kill().is_ok() {
            debug!("drop: Killed eeschema");
        }
    }
}
