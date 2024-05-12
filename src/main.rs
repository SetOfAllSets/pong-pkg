use serde::Deserialize;
use std::fs::{self, read_dir, read_to_string, FileType};
use std::path::PathBuf;
use std::process::Stdio;
use std::rc::Rc;
use std::str;
use clap::error::ErrorKind;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

fn validate_input(args: &Args) -> Result<(), ErrorKind> {
    let mut count = 0;
    for arg in [args.install, args.remove, args.select, args.deselect].iter() {
        if *arg == true {
            count += 1;
        }
    }
    if count < 1 {
        return Err(ErrorKind::MissingRequiredArgument);
    } else if count > 1 {
        return Err(ErrorKind::ArgumentConflict);
    } else {
        return Ok(());
    }
}

fn find_package(package: &String) -> Result<Package, String> {
    let repos = match read_dir(PathBuf::from("e")) {
        Ok(iter) => iter,
        Err(err) => return Err(err.to_string()),
    };
    for repo in repos {
        let repo = match repo {
            Ok(ok) => ok,
            Err(err) => return Err(err.to_string()),
        };
        let config = match fs::read_to_string(repo.path().join(PathBuf::from("./Repo.toml"))) {
            Ok(ok) => ok,
            Err(err) => return Err(err.to_string()),
        };
    };
}

/*
let package = {
    let mut package: Package = match toml::from_str(&contents) {
        Ok(package) => package,
        Err(err) => {
            cmd.error(
                ErrorKind::Io,
                format!(
                    "Unable to parse config file \"{}\" for package \"{}\" due to error \"{}\"",
                    path.to_string_lossy(),
                    args.package,
                    err.to_string()
                ),
            )
            .exit();
        }
    };
    package.name = args.package;
    package
};
*/

#[derive(Deserialize)]
struct Package {
    //Ignore the config version for now, if the config fields change we'll use it then.
    config_version: (u8, u8, u8),
    package_version: (u8, u8, u8),
    fetch: PathBuf,
    install: PathBuf,
    remove: PathBuf,
    //Don't deserialize the name, we already have it so the config file doesn't need to
    #[serde(skip_deserializing)]
    name: String,
    //Serde doesn't let this be a reference so we have to use Repo instead of &Repo. This should be changed if serde ever fixes this.
    repo: Repo,
}

#[derive(Deserialize)]
struct Repo {
    config_version: (u8,u8,u8),
    repo_version: (u8,u8,u8),
    fetch: PathBuf,
    #[serde(skip_deserializing)]
    name: String,
}

fn install(package: Package) {
    print!(
        "{}",
        str::from_utf8(
            std::process::Command::new("echo")
                .arg(package.name)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap()
                .stdout
                .as_slice()
        )
        .unwrap()
    );
}

fn remove(package: Package) {}

fn select(package: Package) {}

fn deselect(package: Package) {}

fn main() {
    let args = Args::parse();
    let mut cmd = command!();
    match validate_input(&args) {
        Ok(_) => (),
        Err(err) => {
            cmd.error(
                err,
                "Need at exactly one of --install, --remove, --select, or --deselect",
            )
            .exit();
        }
    }
    let package = match find_package(&args.package) {
        Ok(path) => path,
        Err(error_message) => cmd.error(ErrorKind::InvalidValue, error_message).exit(),
    };
    let contents = match fs::read_to_string(&path.join(PathBuf::from("Package.toml"))) {
        Ok(contents) => contents,
        Err(err) => {
            cmd.error(
                ErrorKind::Io,
                format!(
                    "Unable to open config file \"{}\" for package \"{}\" due to error \"{}\"",
                    path.to_string_lossy(),
                    args.package,
                    err.to_string()
                ),
            )
            .exit();
        }
    };
    if args.install {
        install(package);
    } else if args.remove {
        remove(package);
    } else if args.select {
        select(package);
    } else if args.deselect {
        deselect(package);
    } else {
        panic!("Input passed validation but did not contain a valid verb. This is a bug.");
    }
}
