use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub places: HashMap<String, Place>,
    pub commands: HashMap<String, Command>,
}

#[derive(Debug, Deserialize)]
pub struct Place {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub command: String,
}

impl Collection {
    pub fn load(content: &str) -> Result<Collection, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    pub fn list_places(&self) {
        for (name, place) in &self.places {
            println!("{} | {}", name, place.path);
        }
    }

    pub fn get_place(&self, place_name: &str) {
        if let Some(place) = self.places.get(place_name) {
            println!("{}", place.path);
        }
        else {
            eprintln!("Place not found.");
        }
    }

    pub fn list_commands(&self) {
        for (name, command) in &self.commands {
            println!("{} | {}", name, command.command);
        }
    }

    pub fn get_command(&self, command_name: &str) {
        if let Some(command) = self.commands.get(command_name) {
            println!("{}", command.command);
        }
        else {
            eprintln!("Command not found.");
        }
    }
}


