use std::{fs::File, io::{BufReader, BufWriter}};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub favorite_folders: Vec<FavoriteFolder>
}

#[derive(Deserialize, Serialize, Debug)]
pub 
struct FavoriteFolder {
    pub name: String,
    pub path: String
}

impl Default for Settings {
    fn default() -> Self {
        serde_json::from_reader(BufReader::new(File::open("settings.json").unwrap())).unwrap()
    }
}

impl Settings {
    pub fn save_to_file(&self){
        let file = File::create("settings.json").unwrap();
        serde_json::to_writer_pretty(BufWriter::new(file), self).unwrap();
    }
    
    pub fn add_favorite_folder(&mut self, name: String, path: String) {
        self.favorite_folders.push(FavoriteFolder { name, path });
        self.save_to_file();
    }
}