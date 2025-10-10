// Tauri application entry point
// All logic is in lib.rs to support both library and binary builds

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    hotkey_prompt_refiner_lib::run();
}
