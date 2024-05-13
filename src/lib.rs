use clap::{command, Parser};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    install: bool,
    #[arg(short, long, default_value_t = false)]
    remove: bool,
    #[arg(short, long, default_value_t = false)]
    select: bool,
    #[arg(short, long, default_value_t = false)]
    deselect: bool,
    #[arg(short, long)]
    package: String,
}

#[derive(Deserialize)]
pub struct RepoConfig {
    config_version: (u8, u8, u8),
    repo_version: (u8, u8, u8),
    fetch: PathBuf,
}

impl RepoConfig {
    pub fn config_version(&self) -> &(u8, u8, u8) {
        &self.config_version
    }
    pub fn repo_version(&self) -> &(u8, u8, u8) {
        &self.repo_version
    }
    pub fn fetch(&self) -> &PathBuf {
        &self.fetch
    }
}

pub struct Repo {
    config: RepoConfig,
    name: String,
    path: PathBuf,
}

impl Repo {
    pub fn config(&self) -> &RepoConfig {
        &self.config
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Deserialize)]
pub struct PackageConfig {
    config_version: (u8, u8, u8),
    package_version: (u8, u8, u8),
    fetch: PathBuf,
    install: PathBuf,
    remove: PathBuf,
}

impl PackageConfig {
    pub fn config_version(&self) -> &(u8, u8, u8) {
        &self.config_version
    }
    pub fn package_version(&self) -> &(u8, u8, u8) {
        &self.package_version
    }
    pub fn fetch(&self) -> &PathBuf {
        &self.fetch
    }
    pub fn install(&self) -> &PathBuf {
        &self.install
    }
    pub fn remove(&self) -> &PathBuf {
        &self.remove
    }
}

pub struct Package<'a> {
    config: PackageConfig,
    name: String,
    repo: &'a Repo,
    path: PathBuf,
}

impl<'a> Package<'a> {
    pub fn config(&self) -> &PackageConfig {
        &self.config
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn repo(&self) -> &'a Repo {
        &self.repo
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

pub fn get_repos() -> Result<Vec<Repo>, String> {
    let dir_contents = match std::fs::read_dir("/home/set/Code/pong-pkg/pong-pkg/repos/") {
        Ok(ok) => ok,
        Err(err) => return Err(format!("Failed to get repositories with error \"{}\" while getting contents of directory",err.to_string())),
    };
    let mut dirs: Vec<PathBuf> = Vec::new();
    for dir_item in dir_contents {
        match dir_item {
            Ok(ok) => dirs.push(ok.path()),
            Err(err) => return Err(format!("Failed to get repositories with error \"{}\" while verifying contents of directory",err.to_string())),
        }
    }
    for dir_item in dirs.iter() {
        if !dir_item.is_dir() {
            return Err("Non-directory found in repositories folder".to_string());
        }
    }
    let mut repos: Vec<Repo> = Vec::with_capacity(dirs.len());
    for dir in dirs {
        let config_str = match fs::read_to_string(&dir.join(Path::new(".repo/Repo.toml"))) {
            Ok(ok) => ok,
            Err(err) => return Err(format!("Failed to get repositories with error \"{}\" while reading config ",err.to_string())),
        };
        let config: RepoConfig = match toml::from_str(&config_str) {
            Ok(ok) => ok,
            Err(err) => return Err(format!("Failed to get repositories with error \"{}\" while parsing config",err.to_string())),
        };
        let repo = Repo {
            config: config,
            name: dir.file_name().unwrap().to_string_lossy().into_owned(),
            path: dir,
        };
        repos.push(repo);
    }
    Ok(repos)
}

pub fn get_matching_packages<'a>(
    repos: &'a Vec<Repo>,
    package_name: &str,
    version: &Option<(u8,u8,u8)>
) -> Result<Vec<Package<'a>>, String> {
    let mut packages: Vec<Package> = Vec::new();
    for repo in repos.iter() {
        let config_path = match version {
            None => repo.path.join(Path::new(&package_name)).join(Path::new("default")).join(Path::new("Package.toml")),
            Some((major,minor,patch)) => repo.path.join(Path::new(&package_name)).join(Path::new(&format!("{}.{}.{}", major, minor, patch))).join(Path::new("Package.toml")),
        };
        let config_str = match fs::read_to_string(config_path) {
            Ok(ok) => ok,
            Err(err) => return Err(format!("Failed to get package with error \"{}\" while reading config",err.to_string())),
        };
        let config: PackageConfig = match toml::from_str(&config_str) {
            Ok(ok) => ok,
            Err(err) => return Err(format!("Failed to get package with error \"{}\" while parsing config",err.to_string())),
        };
        let package = Package {
            config: config,
            name: package_name.to_string(),
            repo: &repo,
            path: repo.path.join(Path::new(&package_name)),
        };
        packages.push(package);
    }
    Ok(packages)
}

pub fn fetch_source(package: &Package) -> Result<(), String> {
    match Command::new(&package.path.join(Path::new("scripts/")).join(&package.config.fetch))
        .spawn() {
            Ok(_) => Ok(()),
            Err(err) => return Err(err.to_string()),
        }
}
