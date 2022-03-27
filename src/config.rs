//     Copyright (C) 2022  cleonyc
// Licensed under GNU Affero General Public License. https://www.gnu.org/licenses/agpl-3.0.en.html
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub database: Database,
    pub bot: Bot,
    pub owner: Owner,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct Database {
    pub url: String,
    pub first_start: bool,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct Bot {
    pub token: String,
    pub application_id: u64,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Owner {
    pub logs: Vec<String>,
    pub ids: Vec<u64>,
}
impl Config {
    pub fn load() -> anyhow::Result<Config> {
        let file = File::open("config.toml")?;
        let mut bufreader = std::io::BufReader::new(file);
        let mut contents = String::new();
        bufreader.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents).unwrap())
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open("config.toml")?;
        let mut bufwriter = std::io::BufWriter::new(file);
        bufwriter.write_all(toml::to_string(&self).unwrap().as_ref())?;
        Ok(())
    }
}
