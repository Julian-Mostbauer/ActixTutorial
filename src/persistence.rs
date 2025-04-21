use crate::User;
use std::collections::HashMap;
use std::fs;

const DATA_PATH: &str = "../data/users.json";

pub fn load_users_from_file() -> HashMap<u32, User> {
    println!("Loading users from file: {}", DATA_PATH);
    if let Ok(data) = fs::read_to_string(DATA_PATH) {
        if let Ok(users) = serde_json::from_str(&data) {
            return users;
        }
    }
    HashMap::new()
}

pub fn save_users_to_file(users: &HashMap<u32, User>) {
    println!("Saving users to file: {}", DATA_PATH);
    if let Ok(data) = serde_json::to_string(users) {
        let _ = fs::write(DATA_PATH, data);
    }
}
