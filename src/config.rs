use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameLevelConfig {
    pub name: String,
    pub rows: i32,
    pub cols: i32,
    pub mines: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub levels: Vec<GameLevelConfig>,
}

impl Config {
    pub fn new() -> Self {
        let file = File::open("./config.json").expect("Could not open the config file");
        let reader = BufReader::new(file);
        let config: Config =
            serde_json::from_reader(reader).expect("Could not parse the config file");

        config
    }
}
