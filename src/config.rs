
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Config {
    pub active: bool,
    pub feed: String,
    pub polling_interval: u32
}


impl Config {
    pub fn new(rc_file: Option<String>) -> Self {
        if let Some(file) = rc_file {
            match Config::from(file) {
                Ok(config) => config,
                _ => Config::default()
            }
        } else {
            Config::default()
        }
    }
    pub fn from(rc_file: String) -> Result<Self, Box<std::error::Error>> {
        let file = File::open(Path::new(&rc_file))?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn save(&self, rc_file: String) -> Result<(), std::io::Error> {
        let file = File::create(Path::new(&rc_file))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}
