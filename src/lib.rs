use std::fs::{OpenOptions};
use std::io::{self, Write};
use std::collections::BTreeMap;
use std::env::consts::OS;
use std::fmt;
use std::path::Path;
extern crate read_lines_with_blank;
use read_lines_with_blank::{read_lines_with_blank, read_lines_with_blank_from_str};

/// Load INI files into a structured BTreeMap, then edit them.
/// Can also create new INI files.
/// You can access the data directly via config_map, or use the provided functions.
/// This only works on Windows and Linux
pub struct Ini {
    pub config_map: BTreeMap<String, BTreeMap<String, String>>,
    pub config_file: String,
}

const CONFIG_SECTION_START: &str = "[";
const CONFIG_SECTION_END: &str = "]";
const CONFIG_KVP_SPLIT: &str = "=";
const CONFIG_COMMENT: &str = "#";

const NEW_LINE_WINDOWS: &str = "\r\n";
const NEW_LINE_LINUX: &str = "\n";

impl Ini {
    /// Load in an INI file and return its structure.
    /// If the file doesn't exist, then returns empty structure.
    pub fn new(location: String) -> Result<Ini, io::Error> {
        let mut ret = Ini{ config_map: BTreeMap::new(), config_file: location.clone() };

        if !Path::new(&location).exists() {
            return Ok(ret);
        }

        let mut in_section = false;

        let lines = match read_lines_with_blank(&location) {
            Ok(x) => x,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to read file"))
        };

        for line in lines {
            if line.starts_with(CONFIG_COMMENT) {
                continue;
            }
            if line.len() == 0 {
                continue;
            }

            // Section found
            if line.starts_with(CONFIG_SECTION_START) && line.contains(CONFIG_SECTION_END) {
                let edit = line.replace(CONFIG_SECTION_START, "").replace(CONFIG_SECTION_END, "").trim().to_string();
                ret.config_map.insert(edit.clone(), BTreeMap::new());
                in_section = true;
                continue;
            }
            // KVP found
            else if line.contains(CONFIG_KVP_SPLIT) {
                if !in_section {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, KVP entry found before section."));
                }

                let kvp = match line.split_once(CONFIG_KVP_SPLIT) {
                    Some(x) => x,
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, KVP entry couldn't be split.")),
                };

                let mut last = match ret.config_map.last_entry() {
                    Some(x) => x,
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, KVP entry didn't have a section.")),
                };

                last.get_mut().insert(kvp.0.to_string(), kvp.1.to_string());

                continue;
            }
            else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, line didn't hit any requirement"));
            }
        }
        Ok(ret)
    }

    /// Dump out the INI file to a string, returns blank string if no data is present
    pub fn to_string(&self) -> Result<String, io::Error> {
        let new_line = match OS {
            "linux" => NEW_LINE_LINUX,
            "windows" => NEW_LINE_WINDOWS,
            _ => return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported OS"))
        };

        let mut ret: String = String::new();

        if self.config_map.is_empty() { return Ok(ret) }

        for (section_k, section_v) in &self.config_map {
            ret.push_str(CONFIG_SECTION_START);
            ret.push_str(section_k);
            ret.push_str(CONFIG_SECTION_END);
            ret.push_str(new_line);

            for (k,v) in section_v {
                ret.push_str(k);
                ret.push_str(CONFIG_KVP_SPLIT);
                ret.push_str(v);
                ret.push_str(new_line);
            }
        }

        Ok(ret)
    }

    /// Create ini structure from a string. Does not set the config_file so save doesn't work unless set manually.
    pub fn from_string(str: String) -> Result<Ini, io::Error> {
        let mut in_section = false;
        let mut ret = Ini{ config_map: BTreeMap::new(), config_file: "".to_string() };

        let lines = match read_lines_with_blank_from_str(&str) {
            Ok(x) => x,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to read file"))
        };

        for line in lines {
            if line.starts_with(CONFIG_COMMENT) {
                continue;
            }
            if line.len() == 0 {
                continue;
            }

            // Section found
            if line.starts_with(CONFIG_SECTION_START) && line.contains(CONFIG_SECTION_END) {
                let edit = line.replace(CONFIG_SECTION_START, "").replace(CONFIG_SECTION_END, "").trim().to_string();
                ret.config_map.insert(edit.clone(), BTreeMap::new());
                in_section = true;
                continue;
            }
            // KVP found
            else if line.contains(CONFIG_KVP_SPLIT) {
                if !in_section {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, KVP entry found before section."));
                }

                let kvp = match line.split_once(CONFIG_KVP_SPLIT) {
                    Some(x) => x,
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, KVP entry couldn't be split.")),
                };

                let mut last = match ret.config_map.last_entry() {
                    Some(x) => x,
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, KVP entry didn't have a section.")),
                };

                last.get_mut().insert(kvp.0.to_string(), kvp.1.to_string());

                continue;
            }
            else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, line didn't hit any requirement"));
            }
        }
        Ok(ret)
    }

    /// Save an INI file after being edited.
    /// Only functions correctly on Windows and Linux.
    /// Ok will contain the size in bytes of the file after writing.
    /// All comments in the INI file will be lost by doing this.
    pub fn save(&self) -> Result<usize, io::Error> {
        if self.config_file.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "config_file is not set. This is likely because this was created using from_string()"))
        }

        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&self.config_file)?;
        let str = match self.to_string() {
            Ok(x) => x,
            Err(e) => return Err(e)
        };

        file.write_all(str.as_bytes())?;
        file.flush()?;
        file.sync_all()?;
        
        Ok(file.metadata()?.len() as usize)
    }
    

    /// Get a value from the INI file.
    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        if let Some(section_map) = self.config_map.get(section) {
            if let Some(value) = section_map.get(key) {
                return Some(value.clone());
            }
        }
        None
    }

    /// Set a value in the INI file.
    /// If the section doesn't exist, it will be created.
    /// If the key doesn't exist, it will be created.
    /// This will not save the file.
    pub fn set(&mut self, section: &str, key: &str, value: &str) {
        let section_map = self.config_map.entry(section.to_string()).or_insert(BTreeMap::new());
        section_map.insert(key.to_string(), value.to_string());
    }

    /// Remove a key from the INI file.
    /// If the section doesn't exist, it will be created.
    /// If the key doesn't exist, it will be created.
    /// This will not save the file.
    pub fn remove(&mut self, section: &str, key: &str) {
        if let Some(section_map) = self.config_map.get_mut(section) {
            section_map.remove(key);
        }
    }

    /// Remove a section from the INI file.
    /// This will not save the file.
    pub fn remove_section(&mut self, section: &str) {
        self.config_map.remove(section);
    }   
}

/// Display trait. Returns the string dump of INI data
impl fmt::Display for Ini {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = self.to_string().unwrap();
        write!(f, "{}", ret)
    }
}