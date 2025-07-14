#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod autostart;
mod events;
mod utils;

fn main() {
    if let Err(e) = gui::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}