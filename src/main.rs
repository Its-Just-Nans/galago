#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use bladvak::app::{Bladvak, MainResult};
use galago::GalagoApp;

fn main() -> MainResult {
    Bladvak::<GalagoApp>::bladvak_main()
}
