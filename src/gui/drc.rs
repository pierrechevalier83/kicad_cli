use crate::gui::*;
use crate::Timeouts;

use std::fs;
use std::io::Read;
use std::path;
use std::thread::sleep;
use std::time::Duration;

use log::{debug, error};
use xdotool::{command::options::SyncOption, option_vec, window, OptionVec};

const NUM_ITEMS_TO_REPORT_ALL_ERRORS_FOR_TRACKS_BOX: usize = 4;
const NUM_ITEMS_TO_CREATE_REPORT_FILE: usize = 2;
// Kicad forces us to use their own extension
const DRC_OUTPUT_FILE: &'static str = "/tmp/drc_output.rpt";

pub fn get_drc_output_from_gui(timeouts: Timeouts) -> Result<String, String> {
    let id = wait_for_child_window("Pcbnew", timeouts.window_launch);
    if id.is_none() {
        return Err(format!("Error expected exactly one Pcbnew window left."));
    }
    let id = id.unwrap();
    window::focus_window(&id, option_vec![SyncOption::Sync]);
    debug!("Try and open the Design Rule Checker window");
    // Select the Inspect menu
    // Note: this doesn't always work. It seems to work in a headless setting and in
    // docker, which is what matters most.
    tap_combo(ALT, I);

    // Navigate to "Run DRC" entry
    tap_key(UP);
    tap_key(RETURN);
    debug!("Wait for the Design Rule Checker window to open");
    // Wait for the popup
    sleep(timeouts.popup_launch);
    debug!("Unfocus pcbnew so it doesn't steal keyboard input");
    // Hiding the main pcbnew window is the only way I found to prevent it from stealing the
    // focus from the drc control window for keyboard input
    window::unmap_window(&id, option_vec![SyncOption::Sync]);
    for _ in 0..NUM_ITEMS_TO_REPORT_ALL_ERRORS_FOR_TRACKS_BOX {
        tap_key(DOWN);
    }
    tap_key(SPACE);
    for _ in 0..NUM_ITEMS_TO_CREATE_REPORT_FILE {
        tap_key(DOWN);
    }
    tap_key(SPACE);
    debug!("Input saving location: {}", DRC_OUTPUT_FILE);
    // Enter the output file path in the box
    type_string(DRC_OUTPUT_FILE);
    let mut output = path::Path::new(DRC_OUTPUT_FILE);
    if output.exists() {
        debug!("rm {}", DRC_OUTPUT_FILE);
        fs::remove_file(output)
            .map_err(|e| format!("Failed to remove previous drc output: {}", e))?;
    }
    // Run DRC
    debug!("Run DRC");
    tap_key(RETURN);
    debug!("Wait for file to be created");
    let mut time_elapsed = Duration::default();
    while !output.exists() && time_elapsed < timeouts.execution {
        output = path::Path::new(DRC_OUTPUT_FILE);
        sleep(WAIT_INCREMENT);
        time_elapsed += WAIT_INCREMENT;
        trace!(".");
    }
    if time_elapsed < timeouts.execution {
        debug!("Found file");
        let output = path::Path::new(DRC_OUTPUT_FILE);
        let mut file = fs::File::open(output)
            .map_err(|e| format!("Failed to open {}: {}", DRC_OUTPUT_FILE, e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read drc output at {}: {}", DRC_OUTPUT_FILE, e))?;
        Ok(contents)
    } else {
        error!("Timed out");
        Err(format!("DRC Timeout"))
    }
}
