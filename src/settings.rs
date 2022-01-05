use std::default::Default;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

use clap::Parser;
use serde::Deserialize;

use crate::error::MyError;

#[derive(clap::Parser, Debug, Clone)]
#[clap(about, version, author)]
struct Args {
    /// Path to the directory containing the wiki Git repository.
    #[clap(parse(from_os_str))]
    git_repo: Option<PathBuf>,
}

#[derive(Default, Deserialize)]
struct Config {
    /// The name of the index page. "Home" by default.
    index_page: Option<String>,
    /// Whether the first H1 should become the title of a page.
    h1_title: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Settings {
    git_repo: PathBuf,
    index_page: String,
    h1_title: bool,
}

impl Settings {
    #[cfg(test)]
    pub(crate) fn new(index_page: &str, h1_title: bool) -> Settings {
        Settings {
            git_repo: PathBuf::new(),
            index_page: index_page.to_owned(),
            h1_title,
        }
    }

    pub fn git_repo(&self) -> &PathBuf {
        &self.git_repo
    }
    pub fn index_page(&self) -> &str {
        &self.index_page
    }
    pub fn h1_title(&self) -> bool {
        self.h1_title
    }
}

fn load_config(git_repo: &Path) -> Result<Config, MyError> {
    let mut config_path = git_repo.to_path_buf();
    config_path.push("smeagol.toml");
    if config_path.is_file() {
        match std::fs::read_to_string(config_path) {
            Ok(config_str) => Ok(toml::from_str(&config_str)?),
            Err(err) => Err(MyError::ConfigReadError { source: err }),
        }
    } else {
        Ok(Default::default())
    }
}

pub fn parse_settings_from_args() -> Result<Settings, MyError> {
    let args = Args::parse();

    let git_repo = if let Some(dir) = args.git_repo {
        dir
    } else {
        std::env::current_dir()?
    };
    let git_repo = canonicalize(git_repo)?;

    if !git_repo.is_dir() {
        return Err(MyError::GitRepoDoesNotExist);
    }

    let config = load_config(&git_repo)?;

    let ret = Settings {
        git_repo,
        index_page: config.index_page.unwrap_or_else(|| "Home".into()),
        h1_title: config.h1_title.unwrap_or(false),
    };
    Ok(ret)
}
