use clap::{command, error::ErrorKind, Command, Parser};
use pong_pkg::*;

fn init() -> (Args, Command, Vec<Repo>){
    let args = Args::parse();
    let mut cmd = command!();
    let repos = match get_repos() {
        Ok(ok) => ok,
        Err(err) => cmd.error(ErrorKind::Io, err).exit(),
    };
    (args, cmd, repos)
}

fn get_package(repos: Vec<Repo>, package: &str, cmd: &mut Command) {
    let packages = match get_matching_packages(&repos, &package, &None) {
        Ok(ok) => ok,
        Err(err) => cmd.error(ErrorKind::Io, err).exit(),
    };
    match fetch_source(&packages[0]) {
        Ok(_) => (),
        Err(err) => cmd.error(ErrorKind::Io, err).exit(),
    }
}

fn main() {
    let (args, mut cmd, repos) = init();
    get_package(repos, "test-package", &mut cmd)
    //let package_config: PackageConfig = toml::from_str(&e).unwrap();
}
