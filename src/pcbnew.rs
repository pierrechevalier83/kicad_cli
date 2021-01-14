use log::{debug, trace};
use std::path;
use std::process::{self, Command, Stdio};

pub struct Pcbnew {
    process: process::Child,
}

impl Pcbnew {
    fn check_schematic_file_looks_valid(p: &path::PathBuf) -> Result<(), String> {
        if !p.exists() {
            Err(format!(
                "Expected path to a kicad pcb file. No such file: {:?}",
                p
            ))
        } else if p.extension() != Some(std::ffi::OsStr::new("kicad_pcb")) {
            Err(format!(
                "Expected path to a kicad pcb file. Extension should be `.kicad_pcb`. It isn't: {:?}",
                p
            ))
        } else {
            Ok(())
        }
    }
    pub fn run(path_to_kicad_pcb: &path::PathBuf) -> Result<Self, String> {
        Self::check_schematic_file_looks_valid(path_to_kicad_pcb)?;
        trace!("Attempting to spawn pcbnew");
        let process = Command::new("pcbnew")
            .arg(path_to_kicad_pcb)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to run pcbnew: {}", e))?;
        debug!("Spawned pcbnew");
        Ok(Self { process })
    }
}
impl Drop for Pcbnew {
    fn drop(&mut self) {
        if self.process.kill().is_ok() {
            debug!("drop: Killed pcbnew");
        }
    }
}
