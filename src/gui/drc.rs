use crate::gui::*;
use std::fs;
use std::io::Read;
use std::path;
use std::thread::sleep;
use std::time::Duration;
use xdotool::{
    command::options::{SearchOption, SyncOption},
    option_vec, window, OptionVec,
};

const NUM_ITEMS_TO_REPORT_ALL_ERRORS_FOR_TRACKS_BOX: usize = 4;
const NUM_ITEMS_TO_CREATE_REPORT_FILE: usize = 2;
const PCBVIEW_LAUNCH_TIMEOUT: Duration = Duration::from_millis(5000);
const POPUP_WINDOW_LAUNCH_DELAY: Duration = Duration::from_millis(1000);
const DRC_TIMEOUT_IN_MS: usize = 15_000;
// Kicad forces us to use their own extension
const DRC_OUTPUT_FILE: &'static str = "/tmp/drc_output.rpt";

// Wait for a window with this name to be present and return its xdotool id
fn wait_for_windows(name: &str, timeout: Duration, n: usize) -> Vec<String> {
    for _ in 0..timeout.as_millis() / 100 {
        let out = String::from_utf8(
            window::search(
                name,
                option_vec![SearchOption::OnlyVisible, SearchOption::Name],
            )
            .stdout,
        )
        .unwrap();
        let ids = out
            .split("\n")
            .filter(|line| line != &"")
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        if ids.len() == n {
            return ids;
        }
        sleep(Duration::from_millis(100));
    }
    Vec::new()
}

pub fn get_drc_output_from_gui() -> Result<String, String> {
    // Wait for pcbnew to start
    let ids = wait_for_windows("Pcbnew", PCBVIEW_LAUNCH_TIMEOUT, 2);
    if ids.is_empty() {
        return Err(format!(
            "Error spawning pcbnew: expected exactly two windows to be spawned."
        ));
    }
    // Dismiss warning about ABI compatibility
    tap_key(RETURN);
    let ids = wait_for_windows("Pcbnew", PCBVIEW_LAUNCH_TIMEOUT, 1);
    if ids.is_empty() {
        return Err(format!("Error expected exactly one Pcbnew window left."));
    }

    // Select the Inspect menu
    tap_combo(ALT, I);
    // Navigate to "Run DRC" entry
    tap_key(UP);
    tap_key(RETURN);
    // Wait for the popup
    sleep(POPUP_WINDOW_LAUNCH_DELAY);
    for id in ids {
        // Hiding the main pcbnew window is the only way I found to prevent it from stealing the
        // focus from the drc control window for keyboard input
        window::unmap_window(&id, option_vec![SyncOption::Sync]);
    }
    for _ in 0..NUM_ITEMS_TO_REPORT_ALL_ERRORS_FOR_TRACKS_BOX {
        tap_key(DOWN);
    }
    tap_key(SPACE);
    for _ in 0..NUM_ITEMS_TO_CREATE_REPORT_FILE {
        tap_key(DOWN);
    }
    tap_key(SPACE);
    // Enter the output file path in the box
    type_string(DRC_OUTPUT_FILE);
    let mut output = path::Path::new(DRC_OUTPUT_FILE);
    if output.exists() {
        fs::remove_file(output)
            .map_err(|e| format!("Failed to remove previous drc output: {}", e))?;
    }
    // Run DRC
    tap_key(RETURN);
    let mut time_elapsed_in_ms = 0;
    while !output.exists() && time_elapsed_in_ms < DRC_TIMEOUT_IN_MS {
        sleep(Duration::from_millis(100));
        time_elapsed_in_ms += 100;
        output = path::Path::new(DRC_OUTPUT_FILE);
    }
    if time_elapsed_in_ms < DRC_TIMEOUT_IN_MS {
        let output = path::Path::new(DRC_OUTPUT_FILE);
        let mut file = fs::File::open(output)
            .map_err(|e| format!("Failed to open {}: {}", DRC_OUTPUT_FILE, e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read drc output at {}: {}", DRC_OUTPUT_FILE, e))?;
        Ok(contents)
    } else {
        println!("Erring");
        Err(format!("DRC Timeout"))
    }
}
