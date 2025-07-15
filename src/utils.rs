use std::fs;
use serde::{Serialize, Deserialize};
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;

const PATH_CONFIG: &str = "config.json";
const WIDGET_POSITION: (i32, i32) = (285, 150);

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    notification: Notification,
    position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    notified: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: i32,
    y: i32,
}



fn load_config() -> Config {
    if let Ok(data) = fs::read_to_string(PATH_CONFIG) {
        if let Ok(config) = serde_json::from_str::<Config>(&data) {
            return config;
        }
    }

    Config {
        notification: Notification { notified: Vec::new() },
        position: Position { x: 0, y: 0 },
    }
}

fn save_config(config: &Config) {
    if let Ok(data) = serde_json::to_string_pretty(config) {
        let _ = fs::write(PATH_CONFIG, data);
    }
}



pub fn load_position() -> (i32, i32) {
    let config = load_config();

    if config.position.x != 0 || config.position.y != 0 {
        return (config.position.x, config.position.y);
    }

    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    (screen_width - WIDGET_POSITION.0, screen_height - WIDGET_POSITION.1)
}

pub fn save_position(position: (i32, i32)) {
    let mut config = load_config();
    config.position = Position { x: position.0, y: position.1 };
    save_config(&config);
}


static NOTIFIED_IDS: Lazy<Mutex<HashSet<i64>>> = Lazy::new(|| {
    let config = load_config();
    let hs: HashSet<i64> = config.notification.notified.into_iter().collect();
    Mutex::new(hs)
});

pub fn is_notificated(id: i64) -> bool {
    let notified = NOTIFIED_IDS.lock().unwrap();
    notified.contains(&id)
}

pub fn save_notification(days: i64) {
    let mut config = load_config();
    config.notification.notified.push(days);
    let mut notified = NOTIFIED_IDS.lock().expect("Error");
    notified.insert(days);
    save_config(&config);
}
