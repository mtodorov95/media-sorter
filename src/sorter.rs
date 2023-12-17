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
    ext: String,
    keep: bool,
}

impl Sorter {
    pub fn sort(&self) -> Result<()> {
        let mut has_sorted_files = false;
        for entry in self
            .src
            .read_dir()
            .expect("Couldn't list source dir entries")
        {
            if let Ok(mut entry) = entry {
                match entry.path().extension() {
                    Some(ext) if ext == self.ext.as_str() => {
                        let file_path = match self.keep {
                            true => entry.path(),
                            false => Sorter::rename_downloaded_file(&mut entry)?,
                        };
                        let file_name = file_path
                            .file_name()
                            .ok_or(anyhow!("Couldn't get file name from file path"))?
                            .to_str()
                            .ok_or(anyhow!("Couldn't get file name string"))?;
                        self.move_to_dir(file_name, &file_path, &self.target)?;
                        has_sorted_files = true;
                    }
                    _ => {}
                }
            }
        }

        if !has_sorted_files {
            println!("No {} files found in {:?}.", self.ext, self.src);
        }

        return Ok(());
    }

    fn move_to_dir(&self, name: &str, from: &PathBuf, to: &PathBuf) -> Result<()> {
        if let Some(existing_dir) = Sorter::find_in_dir(&to, &name) {
            println!("Moving {:?} to existing dir {:?}", from, existing_dir);
            Sorter::move_file(from, &existing_dir)?;
            return Ok(());
        }
        if let Some(new_dir) = Sorter::create_new_dir(&to, &name) {
            println!("Moving {:?} to new dir {:?}", from, new_dir);
            Sorter::move_file(from, &new_dir)?;
            return Ok(());
        }
        println!(
            "Can't find or create dir for {:?}, from {:?}, to {:?}",
            name, from, to
        );
        return Err(anyhow!("Couldn't move file to directory"));
    }

    fn rename_downloaded_file(file: &DirEntry) -> Result<PathBuf> {
        if let Some(name) = file.file_name().to_str() {
            let new_name = Sorter::remove_prefix(name);
            let new_name = new_name.trim();
            let new_path = Sorter::get_new_path(&file.path(), new_name);
            fs::rename(file.path(), &new_path)?;
            return Ok(new_path);
        }
        return Err(anyhow!("Couldn't get file name"));
    }

    fn remove_prefix(value: &str) -> &str {
        if let Some(prefix) = Sorter::get_file_prefix(value) {
            return value.strip_prefix(prefix).unwrap_or(value);
        }
        return value;
    }

    fn get_file_prefix(value: &str) -> Option<&str> {
        if let Some(opening_bracket_idx) = value.find("[") {
            if let Some(closing_bracket_idx) = value.find("]") {
                return Some(&value[opening_bracket_idx..=closing_bracket_idx]);
            }
        }
        return None;
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
        let name_without_prefix = Sorter::remove_prefix(name);
        let dir_name = Sorter::get_new_dir_name_from(name_without_prefix);
        let dir_name: String = dir_name
            .split_whitespace()
            .take(2)
            .fold(String::new(), |acc, word| acc + word + " ");

        return Sorter::temp(dir, &dir_name);
    }

    fn temp(path: &PathBuf, name: &str) -> Option<PathBuf> {
        if path.is_file() {
            if path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("")
                .contains(name)
            {
                match path.parent() {
                    Some(p) => return Some(PathBuf::from(p)),
                    None => return None,
                }
            }
            return None;
        }

        for entry in path
            .read_dir()
            .expect(&format!("Couldn't list {:?} entries", path))
        {
            if let Ok(entry) = entry {
                let res = Sorter::temp(&entry.path(), name);
                if res.is_some() {
                    return res;
                }
            }
        }
        return None;
    }

    fn create_new_dir(parent: &PathBuf, name: &str) -> Option<PathBuf> {
        let name = Sorter::get_new_dir_name_from(name);
        let new_dir = PathBuf::from(parent).join(name);
        if fs::create_dir(&new_dir).is_ok() {
            return Some(new_dir);
        }
        return None;
    }

    fn get_new_dir_name_from(file_name: &str) -> &str {
        return match file_name.find("-") {
            Some(idx) => file_name[0..idx].trim(),
            None => match file_name.find(".") {
                Some(idx) => file_name[0..idx].trim(),
                None => file_name,
            },
        };
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
            ext: config.ext,
            keep: config.keep,
        };
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::opts::Opts;

    use super::Sorter;

    #[test]
    fn remove_prefix_when_present() -> Result<()> {
        let value = "[dasd]after_bracket";

        let value = Sorter::remove_prefix(value);
        assert_eq!(value, "after_bracket");
        return Ok(());
    }

    #[test]
    fn remove_prefix_when_not_present() -> Result<()> {
        let value = "after_bracket";

        let value = Sorter::remove_prefix(value);
        assert_eq!(value, "after_bracket");
        return Ok(());
    }

    #[test]
    fn removes_prefix_when_keep_is_false() -> Result<()> {
        let value = "[prefix]removes_prefix.txt";
        let current_dir = std::env::current_dir()?;
        let temp_dir = current_dir.clone().join("temp").join("removes");
        let file_path = temp_dir.join(value);
        std::fs::create_dir_all(&temp_dir)?;
        std::fs::write(file_path, "Some value")?;

        let config = Opts {
            ext: Some("txt".to_string()),
            target: Some(temp_dir.clone()),
            src: Some(temp_dir.clone()),
            keep: false,
        }
        .try_into()?;
        let sorter = Sorter::from_config(config);
        sorter.sort()?;

        for entry in temp_dir
            .read_dir()
            .expect("Couldn't list source dir entries")
        {
            if let Ok(entry) = entry {
                assert_eq!(entry.file_name().to_str().unwrap(), "removes_prefix");
            }
        }

        std::fs::remove_dir_all(temp_dir)?;
        return Ok(());
    }

    #[test]
    fn keeps_prefix_when_keep_is_true() -> Result<()> {
        let value = "[prefix]keeps_prefix.txt";
        let current_dir = std::env::current_dir()?;
        let temp_dir = current_dir.clone().join("temp").join("keeps");
        let file_path = temp_dir.join(value);
        std::fs::create_dir_all(&temp_dir)?;
        std::fs::write(file_path, "Some value")?;

        let config = Opts {
            ext: Some("txt".to_string()),
            target: Some(temp_dir.clone()),
            src: Some(temp_dir.clone()),
            keep: true,
        }
        .try_into()?;
        let sorter = Sorter::from_config(config);
        sorter.sort()?;

        for entry in temp_dir
            .read_dir()
            .expect("Couldn't list source dir entries")
        {
            if let Ok(entry) = entry {
                assert_eq!(entry.file_name().to_str().unwrap(), "[prefix]keeps_prefix");
            }
        }

        std::fs::remove_dir_all(temp_dir)?;
        return Ok(());
    }

    #[test]
    fn gets_correct_dir_name_from_filename_with_dashes() -> Result<()> {
        let name = "filename-withdash.txt";
        let dir_name = Sorter::get_new_dir_name_from(name);
        assert_eq!(dir_name, "filename");
        return Ok(());
    }

    #[test]
    fn gets_correct_dir_name_from_filename_without_dashes() -> Result<()> {
        let name = "filename.txt";
        let dir_name = Sorter::get_new_dir_name_from(name);
        assert_eq!(dir_name, "filename");
        return Ok(());
    }

    #[test]
    fn gets_correct_dir_name_from_filename_without_extension() -> Result<()> {
        let name = "filename";
        let dir_name = Sorter::get_new_dir_name_from(name);
        assert_eq!(dir_name, "filename");
        return Ok(());
    }

    #[test]
    fn finds_dir_containing_file_with_similar_name() -> Result<()> {
        let current_dir = std::env::current_dir()?;
        let temp_dir = current_dir.join("temp");
        let existing_dir = temp_dir.join("Some long name");
        let file =
            "Some long name that doesn't need to fully match! 2nd season - 14 (1920x1080 x264 AAC).txt";
        let existing_file = existing_dir.join("Some long name that doesn't - 12.txt");
        std::fs::create_dir_all(&existing_dir)?;
        std::fs::write(existing_file, "Something")?;

        let path = Sorter::find_in_dir(&temp_dir, file);
        assert_eq!(path, Some(existing_dir.clone()));
        std::fs::remove_dir_all(existing_dir)?;
        return Ok(());
    }

    #[test]
    fn finds_dir_with_random_name_containing_file_with_similar_name() -> Result<()> {
        let current_dir = std::env::current_dir()?;
        let temp_dir = current_dir.join("temp");
        let existing_dir = temp_dir.join("Random name");
        let file = "A very cool series - EP 02.txt";
        let existing_file = existing_dir.join("A very cool series - EP 01.txt");
        std::fs::create_dir_all(&existing_dir)?;
        std::fs::write(existing_file, "Something")?;

        let path = Sorter::find_in_dir(&temp_dir, file);
        assert_eq!(path, Some(existing_dir.clone()));
        std::fs::remove_dir_all(existing_dir)?;
        return Ok(());
    }

    #[test]
    fn finds_nested_dir_with_random_name_containing_file_with_similar_name() -> Result<()> {
        let current_dir = std::env::current_dir()?;
        let temp_dir = current_dir.join("temp");
        let existing_dir = temp_dir.join("TV/2023/WTF");
        let file = "Best show - EP 02.txt";
        let existing_file = existing_dir.join("Best show - EP 01.txt");
        std::fs::create_dir_all(&existing_dir)?;
        std::fs::write(existing_file, "Something")?;

        let path = Sorter::find_in_dir(&temp_dir, file);
        assert_eq!(path, Some(existing_dir.clone()));
        std::fs::remove_dir_all(temp_dir.join("TV"))?;
        return Ok(());
    }

    #[test]
    fn gets_correct_prefix_from_filename() -> Result<()> {
        let file = "[prefix]filename.txt";
        let prefix = Sorter::get_file_prefix(file);
        assert_eq!(prefix, Some("[prefix]"));
        return Ok(());
    }

    #[test]
    fn returns_none_when_filename_has_no_prefix() -> Result<()> {
        let file = "filename.txt";
        let prefix = Sorter::get_file_prefix(file);
        assert_eq!(prefix, None);
        return Ok(());
    }

    #[test]
    fn removes_prefix_from_filename() -> Result<()> {
        let file = "[prefix]filename.txt";
        let name = Sorter::remove_prefix(file);
        assert_eq!(name, "filename.txt");
        return Ok(());
    }

    #[test]
    fn removes_same_filename_when_no_prefix() -> Result<()> {
        let file = "filename.txt";
        let name = Sorter::remove_prefix(file);
        assert_eq!(name, "filename.txt");
        return Ok(());
    }
}
