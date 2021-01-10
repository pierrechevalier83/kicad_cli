pub mod erc;

use autopilot::key::{self, Character, Code, Flag, KeyCode, KeyCodeConvertible};

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

fn tap_key<Key: KeyCodeConvertible + Copy>(key: Key) {
    key::tap(&key, &[], KEY_TAP_DELAY_IN_MS, 0);
}

fn tap_combo<Key: KeyCodeConvertible + Copy>(flag: Flag, key: Key) {
    key::tap(&key, &[flag], KEY_TAP_DELAY_IN_MS, MOD_TAP_DELAY_IN_MS);
}

fn type_string(s: &str) {
    key::type_string(s, &[], WPM, NOISE);
}
