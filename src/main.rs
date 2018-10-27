use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

#[derive(Serialize, Deserialize)]
pub struct ConfigJson {
    hosts_file_location: String,
    blocked_sites: Vec<String>,
    start_blocking: String,
    end_blocking: String,
}

// Is used to compare two different ConfigJsons together.
impl PartialEq for ConfigJson {
    fn eq(&self, other: &ConfigJson) -> bool {
        self.hosts_file_location == other.hosts_file_location &&
        self.blocked_sites == other.blocked_sites &&
        self.start_blocking == other.start_blocking &&
        self.end_blocking == other.end_blocking
    }
}

/**
 * Read file contents and return them as string.
 * If reading fails return empty string.
 */
fn read_file(fileuri: &str) -> String {
    let path = Path::new(&fileuri);
    // Try open file. If no file found return empty string.
    let mut f = match File::open(path) {
        Err(_why) => return String::new(),
        Ok(f) => f,
    };
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    return contents;
}

/**
 * Save file and return 0 if success, otherwise 1.
 */
fn save_file(fileuri: &str, content: &String) -> i8 {
    let path = Path::new(&fileuri);
    let mut file = match File::create(&path) {
        Err(_why) => return 1,
        Ok(file) => file,
    };
    match file.write_all(content.as_bytes()) {
        Err(_why) => {
            return 0;
        },
        Ok(_) => return 0,
    }
}

/**
 * Load config from file and cast it to ConfigJson type.
 */
fn load_config(configuri: &str) -> ConfigJson {
    let config = read_file(configuri);
    let decoded: ConfigJson = serde_json::from_str(&config).unwrap();
    return decoded;
}

/**
 * Check if current time is in given time frame.
 */
fn check_time(start_time: &String, end_time: &String) -> bool {


    return false;
}

fn main() {
    let config = load_config("./src/config.json");
    let hosts_file = read_file(config.hosts_file_location.as_str());
    let _ = save_file("./src/hosts", &hosts_file);
    print!("{}", &hosts_file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let fileuri = "./src/test.txt";
        let content = "test".to_string();
        assert_eq!(read_file(fileuri), content);
    }

    #[test]
    fn test_save_file() {
        let fileuri = "./src/test2.txt";
        let content = "test";
        assert_eq!(save_file(fileuri, &content.to_string()), 0);
        assert_eq!(read_file(fileuri), content.to_string());
        let _result = fs::remove_file(fileuri);
    }

    #[test]
    fn test_load_configuration() {
        let fileuri = "./src/config.json";
        let mut vec = Vec::new();
        vec.push("facebook.com".to_string());
        let content = ConfigJson {
                        hosts_file_location: "/etc/hosts".to_string(),
                        blocked_sites: vec,
                        start_blocking: "08:00".to_string(),
                        end_blocking: "16:00".to_string(),
                    };
        assert!(load_config(fileuri) == content);
    }
    
    #[test]
    fn test_check_time() {
        let start_time = "08:00".to_string();
        let end_time = "16:00".to_string();
        assert_eq!(check_time(&start_time, &end_time), false);
    }
}