mod args;
mod configuration;

use std::fs;
use args::Cli;
use configuration::Configuration;
use crate::args::Parser;

fn main() {
    let cli = Cli::parse();
    let settings_content = fs::read_to_string(cli.settings.unwrap())
        .expect("Unable to read settings file");
    let configuration = Configuration::load_from_json(&settings_content);
    println!("{:?}", configuration.expect("Problem with configuration"));
}
