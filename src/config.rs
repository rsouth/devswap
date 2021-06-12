use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

const SETTING_FILE: &str = "devswap-settings.json";

#[derive(Data, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    last_project: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { last_project: None }
    }
}

impl Settings {
    pub fn load() -> Settings {
        let settings = std::fs::read_to_string(SETTING_FILE);
        // if let Ok(setting_data) = settings {
        //     println!("deserialized = {:?}", setting_data);
        //     serde_json::from_str(&setting_data).unwrap()
        // } else {
        //     Settings::default()
        // }
        settings
            .map(|setting_data| {
                println!("loaded json = {:?}", setting_data);
                serde_json::from_str(&setting_data).unwrap()
            })
            .unwrap_or_default()
    }

    pub fn save(settings: &Settings) {
        let serialized = serde_json::to_string(settings).unwrap();
        println!("serialized = {}", serialized);

        std::fs::write(SETTING_FILE, serialized).expect("failed to save settings");
    }
}
