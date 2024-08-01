use crate::*;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn manage_config() -> Config {
    let config_path = "./config.json";
    let path = Path::new(&config_path);

    if path.exists() {
        // Read existing configuration from file
        let file = File::open(&path).expect("Failed to open config file.");
        let config = serde_json::from_reader(file).expect("Failed to parse config file.");
        config
    } else {
        // Prompt user for configuration input
        let config = Config {
            api_key: terminal::input("API Key: "),
            username: terminal::input("Username: "),
            project_path: terminal::input("Project path: "),
        };
        
        write_to_json(&config_path, &config);
        config
    }
}

pub fn write_to_json(file_path: &str, config: &Config) {
    // Convert the Config instance to JSON format
    let json = serde_json::to_string_pretty(config).expect("Failed to serialize config");

    // Open the file in write mode, which will truncate the file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file");
    
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Data written to config.json successfully");
}
