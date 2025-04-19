use std::fs::{OpenOptions};
use std::io::{self, Write};
use std::collections::BTreeMap;
use std::env::consts::OS;
use std::path::Path;
extern crate read_lines_with_blank;
use read_lines_with_blank::read_lines_with_blank;

/// Load INI files into a structured BTreeMap, then edit them.
/// Can also create new INI files.
/// You can access the data directly via config_map, or use the provided functions.
/// This only works on Windows and Linux
pub struct Ini {
    pub config_map: BTreeMap<String, BTreeMap<String, String>>,
    config_file: String,
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

        println!("Number of lines: {}", lines.len());

        for line in lines {
            if line.starts_with(CONFIG_COMMENT) {
                println!("Comment line");
                continue;
            }
            if line.len() == 0 {
                println!("Blank line");
                continue;
            }

            println!("{}", line);

            // Section found
            if line.starts_with(CONFIG_SECTION_START) && line.contains(CONFIG_SECTION_END) {
                let edit = line.replace(CONFIG_SECTION_START, "").replace(CONFIG_SECTION_END, "").trim().to_string();
                ret.config_map.insert(edit.clone(), BTreeMap::new());
                in_section = true;
                println!("Found a section");
                continue;
            }
            // KVP found
            else if line.contains(CONFIG_KVP_SPLIT) {
                if !in_section {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Config file was invalid, KVP entry found before section."));
                }
                println!("Found a kvp split");
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
        println!("Number of sections {}", ret.config_map.len());
        Ok(ret)
    }

    /// Save an INI file after being edited.
    /// Only functions correctly on Windows and Linux.
    /// Ok will contain the size in bytes of the file after writing.
    /// All comments in the INI file will be lost by doing this.
    pub fn save(&self) -> Result<usize, io::Error> {
        let new_line = match OS {
            "linux" => NEW_LINE_LINUX,
            "windows" => NEW_LINE_WINDOWS,
            _ => return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported OS"))
        };

        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&self.config_file)?;

        for (section_k, section_v) in &self.config_map {
            file.write_all(CONFIG_SECTION_START.as_bytes())?;
            file.write_all(section_k.as_bytes())?;
            file.write_all(CONFIG_SECTION_END.as_bytes())?;
            file.write_all(new_line.as_bytes())?;

            for (k,v) in section_v {
                file.write_all(k.as_bytes())?;
                file.write_all(CONFIG_KVP_SPLIT.as_bytes())?;
                file.write_all(v.as_bytes())?;
                file.write_all(new_line.as_bytes())?;
            }

            file.flush()?;
        }

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