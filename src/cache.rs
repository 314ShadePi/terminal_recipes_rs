use crate::initializer::create_file_c;
use crate::recipe::Recipe;
use crate::{CACHE_FILE, CONFIG_FILE, RECIPE_DIR};
use anyhow::{anyhow, Context};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Cache {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
}

impl Cache {
    #[tracing::instrument]
    pub fn rebuild() -> anyhow::Result<()> {
        create_file_c(
            PathBuf::from(&<&str>::clone(&CONFIG_FILE).to_string()),
            |_| Ok(()),
        )
        .context("create_file_c() operation failed on cache file.")?;

        let search = Path::new(&<&str>::clone(&RECIPE_DIR))
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
                            println!("ERROR: {e}");
                            return Err(());
                        }
                    };

                    let Ok(file) = serde_json::from_str::<Recipe>(&file_content) else { return Err(()) };

                    Ok(Entry {
                        name: file.name,
                        path: f,
                    })
                }
                Err(_) => Err(()),
            })
            .filter_map(Result::ok)
            .collect::<Vec<Entry>>();

        let cache = Cache { entries };

        let content = serde_json::to_string_pretty(&cache).context("Couldn't serialize cache.")?;

        write(<&str>::clone(&CACHE_FILE), content).context("Couldn't write cache.")
    }

    #[tracing::instrument]
    pub fn get_cache(first: bool) -> anyhow::Result<Self> {
        let cache = read_to_string(<&str>::clone(&CACHE_FILE)).context("Couldn't read cache.")?;

        let cache = match serde_json::from_str::<Cache>(&cache) {
            Ok(c) => c,
            Err(e) => {
                if first {
                    Self::rebuild()?;
                    Self::get_cache(false)?
                } else {
                    let e = anyhow!(e);
                    let e = e.context("Couldn't deserialize cache.");
                    return Err(e);
                }
            }
        };

        Ok(cache)
    }
}

impl std::fmt::Display for Cache {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "To view a recipe use 'view <name>'.")?;
        for (idx, e) in self.entries.iter().enumerate() {
            writeln!(f, "{idx}. {e}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
