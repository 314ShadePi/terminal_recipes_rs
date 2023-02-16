use crate::recipe::Recipe;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write, File};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Cache {
    pub entries: Vec<CacheEntry>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CacheEntry {
    pub name: String,
    pub path: PathBuf,
}

impl Cache {
    pub fn rebuild_cache(data_dir: PathBuf, recipe_dir: PathBuf) {
        let cache_path = data_dir.join("cache.json");
        match cache_path.clone().try_exists() {
            Ok(res) => {
                if !res {
                    match File::create(cache_path.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("ERROR: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(e) => {
                println!("ERROR: {}", e);
                std::process::exit(1);
            }
        }

        let search = recipe_dir.to_str().unwrap().to_string() + "/*.json";

        let t = glob(&search)
            .unwrap()
            .map(|t| match t {
                Ok(f) => {
                    let file_content = match read_to_string(f.clone()) {
                        Ok(f) => f,
                        Err(e) => {
                            format!("ERROR: {}", e)
                        }
                    };

                    let file = match serde_json::from_str::<Recipe>(&file_content) {
                        Ok(c) => c,
                        Err(_) => return Err(()),
                    };

                    Ok(CacheEntry {
                        name: file.name,
                        path: f,
                    })
                }
                Err(e) => Err(()),
            })
            .filter_map(|e| e.ok())
            .collect::<Vec<CacheEntry>>();

        let cache = Cache { entries: t.clone() };

        let content = match serde_json::to_string_pretty(&cache) {
            Ok(c) => c,
            Err(e) => {
                println!("ERROR: {}", e);
                std::process::exit(1);
            }
        };
        match write(cache_path.clone(), content) {
            Ok(_) => {}
            Err(e) => {
                println!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
    }
}
