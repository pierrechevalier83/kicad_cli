use std::io::Read;
use std::path;
use std::process::{self, Command, Stdio};

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
        let process = Command::new("eeschema")
            .arg(path_to_sch)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to run eeschema: {}", e))?;
        Ok(Self { process })
    }
    pub fn dump_stdout(&mut self) -> String {
        let mut buffer = String::new();
        let mut out = self.process.stdout.take().unwrap();
        out.read_to_string(&mut buffer).unwrap();
        buffer
    }
    pub fn dump_stderr(&mut self) -> String {
        let mut buffer = String::new();
        let mut out = self.process.stderr.take().unwrap();
        out.read_to_string(&mut buffer).unwrap();
        buffer
    }
}
impl Drop for Eeschema {
    fn drop(&mut self) {
        self.process.kill().expect("Failed to kill eeschema");
    }
}
