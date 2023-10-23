use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub src: PathBuf,
    pub target: PathBuf,
    pub ext: String,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(opts: Opts) -> Result<Self> {
        let src = get_source(opts.src)?;
        let target = get_target(opts.target)?;
        let ext = get_extension(opts.ext);

        return Ok(Config { src, target, ext });
    }
}

fn get_source(dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = dir {
        return Ok(v);
    }

    let loc = get_path(true)?;
    let loc = PathBuf::from(loc);
    loc.try_exists()?;
    return Ok(loc);
}

fn get_target(dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = dir {
        return Ok(v);
    }

    let loc = get_path(false)?;
    let loc = PathBuf::from(loc);
    loc.try_exists()?;
    return Ok(loc);
}

fn get_path(is_src: bool) -> Result<String> {
    let mut env_var = "SORTER_TARGET_DIR";
    let mut subdir = "/Videos";
    if is_src {
        env_var = "SORTER_SRC_DIR";
        subdir = "/Downloads";
    }
    let loc = match std::env::var(env_var) {
        Ok(v) => v,
        Err(_) => {
            println!("Env variable {} not found. Using default", env_var);
            let home = std::env::var("HOME").context("Unable to get $HOME")?;
            let home = String::from(home + subdir);
            home
        }
    };
    return Ok(loc);
}

fn get_extension(ext: Option<String>) -> String {
    if let Some(v) = ext {
        return v;
    }

    return String::from("mp4");
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::Config;
    use crate::{config::get_path, opts::Opts};
    use anyhow::Result;

    #[test]
    fn from_opts_default() -> Result<()> {
        std::env::remove_var("SORTER_SRC_DIR");
        std::env::remove_var("SORTER_TARGET_DIR");

        let config: Config = Opts {
            src: None,
            target: None,
            ext: None,
        }
        .try_into()?;

        let home = std::env::var("HOME")?;
        let mut t = PathBuf::from(&home);
        t.push("Videos");
        let mut s = PathBuf::from(&home);
        s.push("Downloads");
        assert_eq!(config.src, s);
        assert_eq!(config.target, t);
        assert_eq!(config.ext, String::from("mp4"));

        return Ok(());
    }

    #[test]
    fn from_opts_with_args() -> Result<()> {
        let config: Config = Opts {
            src: Some(PathBuf::from("/foo")),
            target: Some(PathBuf::from("/bar")),
            ext: Some(String::from("mkv")),
        }
        .try_into()?;

        assert_eq!(config.src, PathBuf::from("/foo"));
        assert_eq!(config.target, PathBuf::from("/bar"));
        assert_eq!(config.ext, String::from("mkv"));

        return Ok(());
    }

    #[test]
    fn get_pathbuf_src() -> Result<()> {
        std::env::remove_var("SORTER_SRC_DIR");
        let mut home = std::env::var("HOME")?;
        home.push_str("/Downloads");
        assert_eq!(get_path(true)?, home);
        return Ok(());
    }

    #[test]
    fn get_pathbuf_target() -> Result<()> {
        std::env::remove_var("SORTER_TARGET_DIR");
        let mut home = std::env::var("HOME")?;
        home.push_str("/Videos");
        assert_eq!(get_path(false)?, home);
        return Ok(());
    }
}
