mod args;
mod configuration;
mod cache;

use std::fs;

use args::Cli;
use configuration::Configuration;
use crate::args::Parser;

fn main() {
    let cli = Cli::parse();
    let settings_content = fs::read_to_string(cli.settings.unwrap())
        .expect("Unable to read settings file");
    let Ok(configuration) = Configuration::load_from_json(&settings_content) else {
        eprintln!("Configuration file is invalid or is not available");
        std::process::exit(1);
    };
    let mut cache = cache::load_cache(configuration);
    if cache.is_changed() {
        cache.dump()
    }
}
