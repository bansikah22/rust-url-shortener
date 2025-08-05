use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
struct Db(HashMap<String, String>);

lazy_static! {
    pub static ref STORE: Mutex<HashMap<String, String>> = Mutex::new(load_from_file());
    pub static ref DB_PATH: String =
        env::var("DATABASE_PATH").unwrap_or_else(|_| "db.json".to_string());
}

pub fn shorten(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let hash = format!("{:x}", hasher.finalize());
    let short = &hash[..6];

    let mut store = STORE.lock().unwrap();
    store.insert(short.to_string(), url.to_string());

    save_to_file(&store);
    short.to_string()
}

pub fn expand(short: &str) -> Option<String> {
    let store = STORE.lock().unwrap();
    store.get(short).cloned()
}

fn save_to_file(store: &HashMap<String, String>) {
    let db = Db(store.clone());
    if let Ok(json) = serde_json::to_string_pretty(&db) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(DB_PATH.as_str())
        {
            let _ = file.write_all(json.as_bytes());
        }
    }
}

fn load_from_file() -> HashMap<String, String> {
    if let Ok(mut file) = File::open(DB_PATH.as_str()) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(Db(map)) = serde_json::from_str(&contents) {
                return map;
            }
        }
    }
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_and_expand() {
        let url = "https://example.com";
        let short_code = shorten(url);
        assert_eq!(short_code.len(), 6);

        let expanded_url = expand(&short_code);
        assert_eq!(expanded_url, Some(url.to_string()));

        let missing = expand("abcdef");
        assert_eq!(missing, None);
    }
}
