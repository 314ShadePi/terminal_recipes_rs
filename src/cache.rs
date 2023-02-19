use crate::initializer::create_file_c;
use crate::recipe::Recipe;
use crate::{CACHE_FILE, CONFIG_FILE, RECIPE_DIR};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

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
    pub fn rebuild() -> Result<(), ()> {
        create_file_c(PathBuf::from(&CONFIG_FILE.clone().to_string()), |_| Ok(()))?;

        let search = Path::new(&RECIPE_DIR.clone())
            .join("*.json")
            .to_str()
            .unwrap()
            .to_string();

        let entries = glob(&search)
            .unwrap()
            .map(|t| match t {
                Ok(f) => {
                    let file_content = match read_to_string(f.clone()) {
                        Ok(f) => f,
                        Err(e) => {
                            return Err(());
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
                Err(_) => Err(()),
            })
            .filter_map(|e| e.ok())
            .collect::<Vec<CacheEntry>>();

        let cache = Cache { entries };

        let content = match serde_json::to_string_pretty(&cache) {
            Ok(c) => c,
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(());
            }
        };

        return match write(&CACHE_FILE.clone(), content) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("ERROR: {}", e);
                Err(())
            }
        };
    }

    pub fn get_cache(first: bool) -> Result<Self, ()> {
        let cache = match read_to_string(&CACHE_FILE.clone()) {
            Ok(s) => s,
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(());
            }
        };

        let cache = match serde_json::from_str::<Cache>(&cache) {
            Ok(c) => c,
            Err(e) => {
                if first {
                    Self::rebuild()?;
                    Self::get_cache(false)?
                } else {
                    println!("ERROR: {}", e);
                    return Err(());
                }
            }
        };

        Ok(cache)
    }
}

impl std::fmt::Display for Cache {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "To view a recipe use 'view <name>'.");
        for (idx, e) in self.entries.iter().enumerate() {
            writeln!(f, "{}. {}", idx, e);
        }
        Ok(())
    }
}

impl std::fmt::Display for CacheEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
