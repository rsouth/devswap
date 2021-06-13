use druid::{im, Data};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::borrow::BorrowMut;

const SETTING_FILE: &str = "devswap-settings.json";

#[serde_as]
#[derive(Data, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub last_project: Option<String>,

    #[serde(
        with = "serde_with::rust::map_as_tuple_list",
        default = "im::HashMap::new"
    )]
    pub project_settings: im::HashMap<String, ProjectSettings>,
}

#[derive(Data, Serialize, Deserialize, Debug, Clone)]
pub struct ProjectSettings {}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            last_project: None,
            project_settings: im::hashmap![],
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
        let serialized = serde_json::to_string(settings).unwrap();
        println!("serialized = {}", serialized);

        std::fs::write(SETTING_FILE, serialized).expect("failed to save settings");
    }

    pub fn update(&mut self, proj_name: String, ps: ProjectSettings) {
        let data = self.project_settings.borrow_mut(); // .pro data.get_settings_mut();
        {
            data.insert(proj_name, ps);
            println!("{:?}", data);
        }
        println!("{:?}", data);
    }
}
