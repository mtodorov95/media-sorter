use anyhow::{anyhow, Context, Result};
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::config::Config;

#[derive(Debug)]
pub struct Sorter {
    src: PathBuf,
    target: PathBuf,
    ext: Vec<String>,
    keep: bool,
}

impl Sorter {
    pub fn sort(&self) -> Result<()> {
        let mut has_sorted_files = false;
        for entry in self.src.read_dir().context(format!(
            "Couldn't list entries for source directory {:?}",
            self.src
        ))? {
            if let Ok(mut entry) = entry {
                match entry.path().extension() {
                    Some(ext)
                        if self
                            .ext
                            .contains(&ext.to_str().unwrap_or_default().to_string()) =>
                    {
                        let file_path = match self.keep {
                            true => entry.path(),
                            false => Sorter::rename_downloaded_file(&mut entry)?,
                        };
                        let file_name = file_path
                            .file_name()
                            .ok_or(anyhow!(
                                "Couldn't get file name for file path {:?}",
                                file_path
                            ))?
                            .to_str()
                            .ok_or(anyhow!(
                                "Couldn't parse file name {:?} to string",
                                file_path
                            ))?;
                        self.move_to_dir(file_name, &file_path, &self.target)?;
                        has_sorted_files = true;
                    }
                    _ => {}
                }
            }
        }

        if !has_sorted_files {
            println!("No {:?} files found in {:?}", self.ext, self.src);
        }

        return Ok(());
    }

    fn move_to_dir(&self, name: &str, from: &PathBuf, to: &PathBuf) -> Result<()> {
        if let Some(existing_dir) = Sorter::find_in_dir(&to, &name) {
            Sorter::move_file(from, &existing_dir)?;
            return Ok(());
        }
        if let Ok(new_dir) = Sorter::create_new_dir(&to, &name) {
            Sorter::move_file(from, &new_dir)?;
            return Ok(());
        }
        return Err(anyhow!(
            "Couldn't move file {:?} from {:?} to directory {:?}",
            name,
            from,
            to
        ));
    }

    fn rename_downloaded_file(file: &DirEntry) -> Result<PathBuf> {
        if let Some(name) = file.file_name().to_str() {
            let new_name = Sorter::remove_prefix(name);
            let new_path = Sorter::get_new_path(&file.path(), new_name);
            fs::rename(file.path(), &new_path).context(format!("Failed to rename {:?}", file))?;
            return Ok(new_path);
        }
        return Err(anyhow!("Couldn't parse file name for {:?} to string", file));
    }

    fn remove_prefix(value: &str) -> &str {
        if let Some(prefix) = Sorter::get_file_prefix(value) {
            return value.strip_prefix(prefix).unwrap_or(value).trim();
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

        return Sorter::find_similarly_named_in(dir, &dir_name);
    }

    fn find_similarly_named_in(path: &PathBuf, name: &str) -> Option<PathBuf> {
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

        for entry in path.read_dir().expect("should always be a directory") {
            if let Ok(entry) = entry {
                let res = Sorter::find_similarly_named_in(&entry.path(), name);
                if res.is_some() {
                    return res;
                }
            }
        }
        return None;
    }

    fn create_new_dir(parent: &PathBuf, name: &str) -> Result<PathBuf> {
        let name = Sorter::get_new_dir_name_from(name);
        let new_dir = PathBuf::from(parent).join(name);
        fs::create_dir(&new_dir)?;
        return Ok(new_dir);
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
        if let Some(name) = file
            .file_name()
            .ok_or(anyhow!("Couldn't get filename for {:?}", file))?
            .to_str()
        {
            new_path.push(name);
            fs::rename(file, new_path)?;
            return Ok(());
        }
        return Err(anyhow!("Couldn't parse file name for {:?} to string", file));
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
    use std::fs::File;

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
            ext: Some(vec!["txt".to_string()]),
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
            ext: Some(vec!["txt".to_string()]),
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
    fn sorts_multiple_files_with_different_extensions_correctly() -> Result<()> {
        let file1 = "[prefix]Cool show e01.mkv";
        let file2 = "[prefix]Cool show e02.mp4";
        let file3 = "[prefix]Cool show e03.mkv";
        let file4 = "[prefix]Other show e07.mp4";

        let current_dir = std::env::current_dir()?;
        let temp_dir = current_dir.join("temp");
        let tv_dir = temp_dir.join("tv");
        let some_dir = tv_dir.join("some-dir");

        let file_path1 = some_dir.join(file1);
        let file_path2 = temp_dir.join(file2);
        let file_path3 = temp_dir.join(file3);
        let file_path4 = temp_dir.join(file4);

        std::fs::create_dir_all(&some_dir)?;
        File::create(file_path1)?;
        File::create(file_path2)?;
        File::create(file_path3)?;
        File::create(file_path4)?;

        let config = Opts {
            ext: Some(vec!["mkv".to_string(), "mp4".to_string()]),
            target: Some(tv_dir.clone()),
            src: Some(temp_dir.clone()),
            keep: false,
        }
        .try_into()?;
        let sorter = Sorter::from_config(config);
        sorter.sort()?;

        let files = some_dir.read_dir().unwrap().count();

        assert_eq!(files, 3);

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
