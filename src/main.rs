use autopilot;
use autopilot::key::{self, Character, Code, Flag, KeyCode, KeyCodeConvertible};
use structopt::StructOpt;

use std::io::prelude::*;
use std::process::{Command, Stdio};

const HOME: Code = Code(KeyCode::Home);
const TAB: Code = Code(KeyCode::Tab);
const SPACE: Code = Code(KeyCode::Space);
const RETURN: Code = Code(KeyCode::Return);
const A: Character = Character('a');
const C: Character = Character('c');
const I: Character = Character('i');
const CTRL: Flag = Flag::Control;
const ALT: Flag = Flag::Alt;
const KEY_TAP_DELAY_IN_MS: u64 = 1;
const MOD_TAP_DELAY_IN_MS: u64 = 10;
const WPM: f64 = 240.0;
const NOISE: f64 = 0.0;
const NUM_ITEMS_TO_ERC_FILE_REPORT_BOX: usize = 4;
const EESCHEMA_LAUNCH_DELAY: std::time::Duration = std::time::Duration::from_millis(1000);
const POPUP_WINDOW_LAUNCH_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
const WAITING_FOR_FILE_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
const ERC_OUTPUT_FILE: &'static str = "/tmp/erc_output";
const XVFB_PORT: &'static str = ":99";

#[derive(StructOpt)]
#[structopt(
    name = "run_erc",
    about = "Run Kicad's Electric Rule Checker by spawning the Kicad gui"
)]
struct Options {
    #[structopt(parse(from_os_str))]
    path_to_sch: std::path::PathBuf,
    #[structopt(long)]
    headless: bool,
}

fn tap_key<Key: KeyCodeConvertible + Copy>(key: Key) {
    key::tap(&key, &[], KEY_TAP_DELAY_IN_MS, 0);
}

fn tap_combo<Key: KeyCodeConvertible + Copy>(flag: Flag, key: Key) {
    key::tap(&key, &[flag], KEY_TAP_DELAY_IN_MS, MOD_TAP_DELAY_IN_MS);
}

fn type_string(s: &str) {
    key::type_string(s, &[], WPM, NOISE);
}

fn get_erc_output_from_gui() -> Result<String, String> {
    // Wait for eeschema to start
    std::thread::sleep(EESCHEMA_LAUNCH_DELAY);
    // Alt + i opens the "Inspect" menu
    tap_combo(ALT, I);
    // c selects the "Electrical Rule Checker" item
    tap_key(C);
    // Wait for the Electrical Rule Checker window to appear
    std::thread::sleep(POPUP_WINDOW_LAUNCH_DELAY);
    // Tab over the UI elements, until "Create ERC File report"
    for _ in 0..NUM_ITEMS_TO_ERC_FILE_REPORT_BOX {
        tap_key(TAB);
    }
    // Tick "Create ERC File report"
    tap_key(SPACE);
    // Hit "Run"
    tap_key(RETURN);
    // Wait for the save dialog
    std::thread::sleep(POPUP_WINDOW_LAUNCH_DELAY);
    tap_key(HOME);
    tap_combo(CTRL, A);
    type_string(ERC_OUTPUT_FILE);

    let mut output = std::path::Path::new(ERC_OUTPUT_FILE);
    if output.exists() {
        std::fs::remove_file(output)
            .map_err(|e| format!("Failed to remove previous erc output: {}", e))?;
    }
    // Let's save to the path we entered and run ERC
    tap_key(RETURN);
    let mut loop_count = 0;
    while !output.exists() && loop_count < 10 {
        output = std::path::Path::new(ERC_OUTPUT_FILE);
        std::thread::sleep(WAITING_FOR_FILE_DELAY);
        loop_count += 1;
    }
    let mut file = std::fs::File::open(output).map_err(|e| format!("Failed to open {}: {}", ERC_OUTPUT_FILE, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read erc output at {}: {}", ERC_OUTPUT_FILE, e))?;
    Ok(contents)
}

fn run_eeschema(path_to_sch: std::path::PathBuf) -> Result<std::process::Child, String> {
    Command::new("eeschema")
        .arg(path_to_sch)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run eeschema: {}", e))
}

fn run_xvfb() -> Result<std::process::Child, String> {
    Command::new("Xvfb")
        .args(&[XVFB_PORT, "-ac", "-nolisten", "tcp"])
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run xvfb: {}", e))
}

fn main() -> Result<(), String> {
    let args = Options::from_args();
    let xvfb_process = if args.headless {
        std::env::set_var("DISPLAY", XVFB_PORT);
        Some(run_xvfb()?)
    } else {
        None
    };
    let mut eeschema_process = run_eeschema(args.path_to_sch)?;
    let erc_output = get_erc_output_from_gui()?;
    // TODO: use the captured stdout and stderr in case of problems to give more context
    let _ = eeschema_process.kill();
    xvfb_process.map(|mut process| {
        std::env::remove_var("DISPLAY");
        process.kill()
    });
    println!("{}", erc_output);
    Ok(())
}
