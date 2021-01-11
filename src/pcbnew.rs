use std::io::Read;
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
        let process = Command::new("pcbnew")
            .arg(path_to_kicad_pcb)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to run pcbnew: {}", e))?;
        Ok(Self { process })
    }
    pub fn dump_stdout(&mut self) -> String {
        let _ = self.process.kill();
        let mut buffer = String::new();
        let mut out = self.process.stdout.take().unwrap();
        out.read_to_string(&mut buffer).unwrap();
        buffer
    }
    pub fn dump_stderr(&mut self) -> String {
        let _ = self.process.kill();
        let mut buffer = String::new();
        let mut out = self.process.stderr.take().unwrap();
        out.read_to_string(&mut buffer).unwrap();
        buffer
    }
}
impl Drop for Pcbnew {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}
