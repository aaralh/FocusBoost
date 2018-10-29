use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{thread, time};

extern crate serde;
extern crate serde_json;

extern crate chrono;
use chrono::{NaiveTime, Local};

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

#[derive(Serialize, Deserialize)]
pub struct ConfigJson {
    hosts_file_location: String,
    blocked_sites: Vec<String>,
    start_blocking: String,
    end_blocking: String,
    sleep_time: u64,
}

// Is used to compare two different ConfigJsons together.
impl PartialEq for ConfigJson {
    fn eq(&self, other: &ConfigJson) -> bool {
        self.hosts_file_location == other.hosts_file_location &&
        self.blocked_sites == other.blocked_sites &&
        self.start_blocking == other.start_blocking &&
        self.end_blocking == other.end_blocking &&
        self.sleep_time == other.sleep_time
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
    let begin = NaiveTime::parse_from_str(start_time, "%H:%M").unwrap();
    let end = NaiveTime::parse_from_str(end_time, "%H:%M").unwrap();
    let current = Local::now().time();
    return (begin < current) && (current < end);
}

fn main() {
    let config = load_config("./src/config.json");
    let mut is_updated = false;

    // Taking backup from original hosts file.
    let hosts = read_file(&config.hosts_file_location);
    let _ = save_file("./src/hosts", &hosts);
    loop {
        let in_frame = check_time(&config.start_blocking, &config.end_blocking);
        if in_frame && !is_updated {
            let mut hosts = read_file(&config.hosts_file_location).to_owned();
            for website in &config.blocked_sites {
                hosts.push_str(&format!("127.0.0.1\t{}", website));
            }
            let _ = save_file(&config.hosts_file_location, &hosts);
            is_updated = true;
        } else {
            let hosts = read_file("./src/hosts").to_owned();
            let _ = save_file(&config.hosts_file_location, &hosts);
            is_updated = false;
        }
        let sleep_time = time::Duration::from_secs(config.sleep_time);
        thread::sleep(sleep_time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let fileuri = "./src/config.json";
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
                        sleep_time: 60,
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