use crate::gui::*;
use std::fs;
use std::io::Read;
use std::path;
use std::thread::sleep;

const NUM_ITEMS_TO_ERC_FILE_REPORT_BOX: usize = 4;
const EESCHEMA_LAUNCH_DELAY: std::time::Duration = std::time::Duration::from_millis(1000);
const POPUP_WINDOW_LAUNCH_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
const WAITING_FOR_FILE_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
const ERC_OUTPUT_FILE: &'static str = "/tmp/erc_output";

pub fn get_erc_output_from_gui() -> Result<String, String> {
    // Wait for eeschema to start
    sleep(EESCHEMA_LAUNCH_DELAY);
    // Alt + i opens the "Inspect" menu
    tap_combo(ALT, I);
    // c selects the "Electrical Rule Checker" item
    tap_key(C);
    // Wait for the Electrical Rule Checker window to appear
    sleep(POPUP_WINDOW_LAUNCH_DELAY);
    // Tab over the UI elements, until "Create ERC File report"
    for _ in 0..NUM_ITEMS_TO_ERC_FILE_REPORT_BOX {
        tap_key(TAB);
    }
    // Tick "Create ERC File report"
    tap_key(SPACE);
    // Hit "Run"
    tap_key(RETURN);
    // Wait for the save dialog
    sleep(POPUP_WINDOW_LAUNCH_DELAY);
    tap_key(HOME);
    tap_combo(CTRL, A);
    type_string(ERC_OUTPUT_FILE);

    let mut output = path::Path::new(ERC_OUTPUT_FILE);
    if output.exists() {
        fs::remove_file(output)
            .map_err(|e| format!("Failed to remove previous erc output: {}", e))?;
    }
    // Let's save to the path we entered and run ERC
    tap_key(RETURN);
    let mut loop_count = 0;
    while !output.exists() && loop_count < 10 {
        output = path::Path::new(ERC_OUTPUT_FILE);
        sleep(WAITING_FOR_FILE_DELAY);
        loop_count += 1;
    }
    let mut file =
        fs::File::open(output).map_err(|e| format!("Failed to open {}: {}", ERC_OUTPUT_FILE, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read erc output at {}: {}", ERC_OUTPUT_FILE, e))?;
    Ok(contents)
}
