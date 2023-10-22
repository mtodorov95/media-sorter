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

    let loc = std::env::var("HOME").context("unable to get HOME")?;
    let mut loc = PathBuf::from(loc);
    loc.push("/Hestia/Downloads");
    return Ok(loc);
}

fn get_target(dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = dir {
        return Ok(v);
    }

    let loc = std::env::var("HOME").context("unable to get HOME")?;
    let mut loc = PathBuf::from(loc);
    loc.push("/Hestia/TV");
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
    use crate::opts::Opts;
    use anyhow::{Result, Context};

    #[test]
    fn from_opts_default() -> Result<()> {
        let config: Config = Opts {
            src: None,
            target: None,
            ext: None
        }
        .try_into()?;

        let t = std::env::var("HOME").context("unable to get HOME")?;
        let mut t = PathBuf::from(t);
        t.push("/Hestia/TV");
        let s = std::env::var("HOME").context("unable to get HOME")?;
        let mut s = PathBuf::from(s);
        s.push("/Hestia/Downloads");
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
}
