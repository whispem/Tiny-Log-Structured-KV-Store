use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Log-structured key-value store.
/// All state is in-memory, log is append-only for persistence.
pub struct KvStore {
    pub map: HashMap<String, String>,
    log_file: File,
    log_path: String,
}

impl KvStore {
    /// Open (or create) a log file and load the store from it.
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<KvStore> {
        let log_path = path.as_ref().to_string_lossy().to_string();
        let map = KvStore::replay_log(&log_path)?;

        // Open log file for appending
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        Ok(KvStore { map, log_file, log_path })
    }

    /// Replay the log from disk to rebuild state
    fn replay_log(log_path: &str) -> std::io::Result<HashMap<String, String>> {
        let mut map = HashMap::new();

        if let Ok(file) = File::open(log_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<_> = line.trim().splitn(3, ' ').collect();
                    if parts.len() >= 2 {
                        match parts[0] {
                            "SET" if parts.len() == 3 => {
                                map.insert(parts[1].to_string(), parts[2].to_string());
                            }
                            "DEL" if parts.len() == 2 => {
                                map.remove(parts[1]);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(map)
    }

    /// Set/add a key-value pair with log persistence
    pub fn set(&mut self, key: String, value: String) -> std::io::Result<()> {
        writeln!(self.log_file, "SET {} {}", key, value)?;
        self.log_file.flush()?;
        self.map.insert(key, value);
        Ok(())
    }

    /// Get the value for a key if it exists
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    /// Delete a key, logging the operation
    pub fn delete(&mut self, key: &str) -> std::io::Result<bool> {
        writeln!(self.log_file, "DEL {}", key)?;
        self.log_file.flush()?;
        Ok(self.map.remove(key).is_some())
    }
}
