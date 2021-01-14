pub mod drc;
pub mod erc;

use std::{fmt::Debug, thread::sleep, time::Duration};

use autopilot::key::{self, Character, Code, Flag, KeyCode, KeyCodeConvertible};
use log::{debug, error, trace};
use xdotool::{command::options::SearchOption, option_vec, window, OptionVec};

const DOWN: Code = Code(KeyCode::DownArrow);
const HOME: Code = Code(KeyCode::Home);
const RETURN: Code = Code(KeyCode::Return);
const SPACE: Code = Code(KeyCode::Space);
const TAB: Code = Code(KeyCode::Tab);
const UP: Code = Code(KeyCode::UpArrow);
const A: Character = Character('a');
const C: Character = Character('c');
const I: Character = Character('i');
const CTRL: Flag = Flag::Control;
const ALT: Flag = Flag::Alt;
const KEY_TAP_DELAY_IN_MS: u64 = 50;
const MOD_TAP_DELAY_IN_MS: u64 = 50;
const WPM: f64 = 240.0;
const NOISE: f64 = 0.0;

fn tap_key<Key: KeyCodeConvertible + Copy + Debug>(key: Key) {
    trace!("tap_key: {:?}", key);
    key::tap(&key, &[], KEY_TAP_DELAY_IN_MS, 0);
}

fn tap_combo<Key: KeyCodeConvertible + Copy + Debug>(flag: Flag, key: Key) {
    trace!("tap_combo: {:?} + {:?}", flag, key);
    key::tap(&key, &[flag], KEY_TAP_DELAY_IN_MS, MOD_TAP_DELAY_IN_MS);
}

fn type_string(s: &str) {
    trace!("type_string: \"{}\"", s);
    key::type_string(s, &[], WPM, NOISE);
}

// Wait for a window with this name to be present and return its xdotool id
fn wait_for_child_window(name: &str, timeout: Duration) -> Option<String> {
    for _ in 0..timeout.as_millis() / 100 {
        let out = String::from_utf8(
            window::search(
                name,
                option_vec![
                    SearchOption::OnlyVisible,
                    SearchOption::Pid(std::process::id()),
                    SearchOption::Any
                ],
            )
            .stdout,
        )
        .unwrap();
        let ids = out
            .split("\n")
            .filter(|line| line != &"")
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        if ids.len() > 1 {
            error!(
                "Expected only one window with name: {}, got {}",
                name,
                ids.len()
            );
            for id in ids {
                error!(
                    "Got: {} -> {}",
                    id,
                    String::from_utf8(window::get_window_name(&id).stdout).unwrap()
                );
            }
            return None;
        } else if ids.len() == 1 {
            for id in &ids {
                debug!(
                    "Found window spawned by us with id: {} and name: {}",
                    id,
                    String::from_utf8(window::get_window_name(&id).stdout).unwrap()
                );
            }
            return Some(ids[0].clone());
        }
        sleep(Duration::from_millis(100));
    }
    error!("Failed to find any window with name: {}", name);
    None
}
