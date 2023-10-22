use anyhow::{anyhow, Result};
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::config::Config;

#[derive(Debug)]
pub struct Sorter {
    src: PathBuf,
    target: PathBuf,
    ext: String
}

impl Sorter {
    pub fn sort(&self) -> Result<()> {
        for entry in self
            .src
            .read_dir()
            .expect("Couldn't list source dir entries")
        {
            if let Ok(mut entry) = entry {
                match entry.path().extension() {
                    Some(ext) if ext == self.ext.as_str() => {
                        let new_path = Sorter::rename_downloaded_file(&mut entry)?;
                        self.move_to_dir(new_path, &self.target)?;
                    }
                    _ => {}
                }
            }
        }

        return Ok(());
    }

    fn move_to_dir(&self, from: PathBuf, to: &PathBuf) -> Result<()> {
        let name = from
            .file_name()
            .ok_or(anyhow!("Couldn't get file name from OsStr"))?;
        if let Some(name) = name.to_str() {
            if let Some(existing_dir) = Sorter::find_in_dir(&to, &name) {
                Sorter::move_file(&from, &existing_dir)?;
                return Ok(());
            } else {
                if let Some(new_dir) = Sorter::create_new_dir(&to, &name) {
                    Sorter::move_file(&from, &new_dir)?;
                    return Ok(());
                }
            }
        }
        return Err(anyhow!("Couldn't move file to directory"));
    }

    fn rename_downloaded_file(file: &DirEntry) -> Result<PathBuf> {
        if let Some(name) = file.file_name().to_str() {
            let new_name = Sorter::remove_prefixes(name)?;
            let new_name = new_name.trim();
            let new_path = Sorter::get_new_path(&file.path(), new_name);
            fs::rename(file.path(), &new_path)?;
            return Ok(new_path);
        }
        return Err(anyhow!("Couldn't get file name"));
    }

    fn remove_prefixes(value: &str) -> Result<String> {
        let mut new = String::from(value);
        if let Some(opening_bracket_idx) = value.find("[") {
            if let Some(closing_bracket_idx) = value.find("]") {
                new.drain(opening_bracket_idx..=closing_bracket_idx);
            }
        }
        return Ok(new);
    }

    fn get_new_path(path: &PathBuf, name: &str) -> PathBuf {
        let mut new_file = PathBuf::from(path);
        new_file.set_file_name(name);
        if let Some(ext) = path.extension() {
            new_file.set_extension(ext);
        }
        return new_file;
    }

    fn find_in_dir(dir: &PathBuf, name: &str) -> Option<PathBuf> {
        if let Some(first_word) = name.split(" ").next() {
            for entry in dir
                .read_dir()
                .expect(&format!("Couldn't list {} entries", dir.display()))
            {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        if entry
                            .file_name()
                            .to_str()
                            .unwrap_or("")
                            .contains(first_word)
                        {
                            return Some(entry.path());
                        }
                    }
                }
            }
        }

        return None;
    }

    fn create_new_dir(parent: &PathBuf, name: &str) -> Option<PathBuf> {
        if let Some(idx) = name.find("-") {
            let name = name[0..idx].trim();
            let mut new_dir = PathBuf::from(parent);
            new_dir.push(name);
            if fs::create_dir(&new_dir).is_ok() {
                return Some(new_dir);
            }
        }
        return None;
    }

    fn move_file(file: &PathBuf, to: &PathBuf) -> Result<()> {
        let mut new_path = PathBuf::from(to);
        if let Some(name) = file.file_name().unwrap().to_str() {
            new_path.push(name);
            fs::rename(file, new_path)?;
            return Ok(());
        }
        return Err(anyhow!("Couldn't get the file name"));
    }

    pub fn from_config(config: Config) -> Self {
        return Sorter {
            src: config.src,
            target: config.target,
            ext: config.ext
        };
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::Sorter;

    #[test]
    fn remove_prefix_when_present() -> Result<()> {
        let value = "[dasd]after_bracket";

        let value = Sorter::remove_prefixes(value)?;
        assert_eq!(value, "after_bracket");
        return Ok(());
    }

    #[test]
    fn remove_prefix_when_not_present() -> Result<()> {
        let value = "after_bracket";

        let value = Sorter::remove_prefixes(value)?;
        assert_eq!(value, "after_bracket");
        return Ok(());
    }
}
