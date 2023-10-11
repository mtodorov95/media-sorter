use anyhow::{anyhow, Result};
use home;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

const PREFIX: &'static str = "[Ohys-Raws] ";
const DEFAULT_SRC: &'static str = "/Hestia/Downloads";
const DEFAULT_TARGET: &'static str = "/Hestia/TV";

fn main() -> Result<()> {
    if let Some(h) = home::home_dir() {
        let mut download_dir = PathBuf::from(&h);
        download_dir.push(DEFAULT_SRC);
        let mut tv_dir = PathBuf::from(&h);
        tv_dir.push(DEFAULT_TARGET);

        for entry in download_dir
            .read_dir()
            .expect("Couldn't list source dir entries")
        {
            if let Ok(mut entry) = entry {
                match entry.path().extension() {
                    Some(ext) if ext == "mp4" => {
                        rename_downloaded_file(&mut entry)?;
                        move_to_dir(entry, &tv_dir)?;
                    }
                    _ => {}
                }
            }
        }
    }
    return Ok(());
}

fn rename_downloaded_file(entry: &mut DirEntry) -> Result<()> {
    let new_name = entry.file_name().to_str().unwrap().replace(PREFIX, "");
    let new_path = change_file_name(&entry.path(), &new_name);
    fs::rename(entry.path(), new_path)?;
    return Ok(());
}

fn change_file_name(path: &PathBuf, name: &str) -> PathBuf {
    let mut new_file = PathBuf::from(path);
    new_file.set_file_name(name);
    if let Some(ext) = path.extension() {
        new_file.set_extension(ext);
    }
    return new_file;
}

fn move_to_dir(entry: DirEntry, to: &PathBuf) -> Result<()> {
    let path = entry.path();
    let mut out = Ok(());
    if let Some(name) = entry.file_name().to_str() {
        if let Some(existing_dir) = find_in_dir(&to, &name) {
            move_file(&path, &existing_dir)?;
            return out;
        } else {
            if let Some(new_dir) = create_new_dir(&to, &name) {
                move_file(&path, &new_dir)?;
                return out;
            }
        }
    }
    out = Err(anyhow!("Couldn't move file to directory"));
    return out;
}

fn find_in_dir(dir: &PathBuf, name: &str) -> Option<PathBuf> {
    if let Some(first_word) = name.split(" ").next() {
        println!("First word {}", first_word);
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
