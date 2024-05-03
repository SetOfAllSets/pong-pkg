use std::path::PathBuf;
use std::process::Stdio;
use std::str;

use clap::error::ErrorKind;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    add: bool,
    #[arg(short, long, default_value_t = false)]
    remove: bool,
    #[arg(short, long, default_value_t = false)]
    select: bool,
    #[arg(short, long, default_value_t = false)]
    deselect: bool,
    #[arg(short, long)]
    package: String,
}

fn validate_input(args: &Args) {
    let mut cmd = command!();
    let mut count = 0;
    for arg in [args.add, args.remove, args.select, args.deselect].iter() {
        if *arg == true {
            count += 1;
        }
    }
    let error_kind: ErrorKind;
    if count < 1 {
        error_kind = ErrorKind::MissingRequiredArgument;
    } else if count > 1 {
        error_kind = ErrorKind::ArgumentConflict;
    } else {
        return;
    }
    cmd.error(
        error_kind,
        "Need at exactly one of --add, --remove, --select, or --deselect",
    )
    .exit();
    //MissingRequiredArgument
    //If this function exits then input has been validated
}

fn find_package(package: &String) -> Result<PathBuf, String> {
    //TODO: actually search for the package
    if package == "test-package" {
        return Result::Ok(PathBuf::from("./e"));
    }
    Result::Err("E".to_string())
}

fn add(package: &String) {
    println!("{}",str::from_utf8(std::process::Command::new("echo").arg(package).stdout(Stdio::piped()).spawn().unwrap().wait_with_output().unwrap().stdout.as_slice()).unwrap());
}

fn remove(package: &String) {
    std::process::Command::new("echo").arg(package);
}

fn select(package: &String) {
    std::process::Command::new("echo").arg(package);
}

fn deselect(package: &String) {
    std::process::Command::new("echo").arg(package);
}

fn main() {
    let args = Args::parse();
    validate_input(&args);
    match find_package(&args.package) {
        Ok(_) => (),
        Err(error_message) => {
            let mut cmd = command!();
            cmd.error(ErrorKind::InvalidValue, error_message).exit()
        }
    }
    if args.add {
        add(&args.package);
    } else if args.remove {
        remove(&args.package);
    } else if args.select {
        select(&args.package);
    } else if args.deselect {
        deselect(&args.package);
    } else {
        panic!("Input passed validation but did not contain a valid verb. This is a bug.");
    }
}
