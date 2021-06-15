use druid::{im, Data};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;

const SETTING_FILE: &str = "devswap-settings.json";

#[derive(Data, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub last_project: Option<String>,

    #[serde(default = "im::HashMap::new")]
    project_settings: im::HashMap<String, ProjectSettings>,

    #[serde(default = "im::Vector::new")]
    command_history: im::Vector<String>,

    #[serde(default = "default_command_history")]
    command_history_size: usize,
}

#[derive(Data, Serialize, Deserialize, Debug, Clone)]
pub struct ProjectSettings {}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            last_project: None,
            project_settings: im::hashmap![],
            command_history: im::vector![],
            command_history_size: 100,
        }
    }
}

impl Settings {
    pub fn load() -> Settings {
        let settings = std::fs::read_to_string(SETTING_FILE);
        settings
            .map(|setting_data| {
                println!("loaded json = {:?}", setting_data);
                serde_json::from_str(&setting_data).unwrap()
            })
            .unwrap_or_default()
    }

    pub fn save(settings: &Settings) {
        let serialized = serde_json::to_string_pretty(settings).unwrap();
        println!("serialized = {}", serialized);

        std::fs::write(SETTING_FILE, serialized).expect("failed to save settings");
    }

    pub fn update(&mut self, proj_name: String, ps: ProjectSettings) {
        let data = self.project_settings.borrow_mut();
        {
            data.insert(proj_name, ps);
            println!("{:?}", data);
        }
        println!("{:?}", data);
    }

    pub fn add_to_command_history(&mut self, cmd: &str) {
        if self
            .command_history
            .front()
            .filter(|d| d.as_str() == cmd)
            .is_none()
        {
            self.command_history.push_front(cmd.to_string());
            if self.command_history.len() > self.command_history_size {
                self.command_history.truncate(self.command_history_size);
            }
        }

        println!("Command history: {:?}", self.command_history);
    }
}

fn default_command_history() -> usize {
    100
}
