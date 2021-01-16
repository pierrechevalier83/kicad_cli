use crate::gui::*;

use std::fs;
use std::io::Read;
use std::path;
use std::thread::sleep;

use log::{debug, trace};

const NUM_ITEMS_TO_ERC_FILE_REPORT_BOX: usize = 4;
const EESCHEMA_LAUNCH_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(10_000);
const POPUP_WINDOW_LAUNCH_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
const ERC_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(10_000);
const ERC_OUTPUT_FILE: &'static str = "/tmp/erc_output";

pub fn get_erc_output_from_gui() -> Result<String, String> {
    debug!("Wait for eeschema to start");
    let id = wait_for_child_window("Eeschema.*", EESCHEMA_LAUNCH_TIMEOUT);
    if id.is_none() {
        return Err(format!("Failed to launch eeschema"));
    }
    debug!("Try and open the Electrical Rule Checker window");
    // Alt + i opens the "Inspect" menu
    tap_combo(ALT, I);
    // c selects the "Electrical Rule Checker" item
    tap_key(C);
    debug!("Wait for the Electrical Rule Checker window to appear");
    sleep(POPUP_WINDOW_LAUNCH_DELAY);
    // Tab over the UI elements, until "Create ERC File report"
    for _ in 0..NUM_ITEMS_TO_ERC_FILE_REPORT_BOX {
        tap_key(TAB);
    }
    // Tick "Create ERC File report"
    tap_key(SPACE);
    debug!("Hit \"Run\"");
    tap_key(RETURN);
    debug!("Wait for the save dialog");
    sleep(POPUP_WINDOW_LAUNCH_DELAY);
    tap_key(HOME);
    tap_combo(CTRL, A);
    type_string(ERC_OUTPUT_FILE);

    let mut output = path::Path::new(ERC_OUTPUT_FILE);
    if output.exists() {
        debug!("rm {:?}", output);
        fs::remove_file(output)
            .map_err(|e| format!("Failed to remove previous erc output: {}", e))?;
    }
    // Let's save to the path we entered and run ERC
    debug!("Run ERC");
    tap_key(RETURN);
    debug!("Wait for file to be created");
    let mut time_elapsed = Duration::default();
    while !output.exists() && time_elapsed < ERC_TIMEOUT {
        output = path::Path::new(ERC_OUTPUT_FILE);
        sleep(WAIT_INCREMENT);
        time_elapsed += WAIT_INCREMENT;
        trace!(".");
    }
    debug!("Found file or timed out");
    let mut file =
        fs::File::open(output).map_err(|e| format!("Failed to open {}: {}", ERC_OUTPUT_FILE, e))?;
    debug!("Opened file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read erc output at {}: {}", ERC_OUTPUT_FILE, e))?;
    Ok(contents)
}
