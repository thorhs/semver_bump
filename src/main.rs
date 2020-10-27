use std::env;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Command {
    Major,
    Minor,
    Patch,
}

fn usage(message: &str) -> ! {
    println!("{}", message);
    println!();
    println!("Usage: semver_bump <major|minor|patch> <version|-> [-b build_string] [-p pre-release_version]");
    println!("input version must confirm to SemVer 2.0, with an optional v prefix");
    println!("input version must contain major, minor and patch parts.");
    println!(
        "input version can optionally start with v, if it does, the output also starts with a v"
    );
    println!("input version can optionally have a -pre_release postfix");
    println!("input version can optionally have a +build_string postfix");
    println!("input version can optionally be -, causing semver_bump to read version from stdin");
    std::process::exit(1);
}

fn main() {
    let mut args = env::args().skip(1);

    let command = match args.next().as_ref().map(String::as_str) {
        Some("major") => Command::Major,
        Some("minor") => Command::Minor,
        Some("patch") => Command::Patch,
        Some(cmd) => usage(&format!("Unknown command: {}", cmd)),
        _ => usage("Missing command"),
    };

    let mut input_string = args
        .next()
        .unwrap_or_else(|| usage("Missing input version"));

    let stdin = io::stdin();
    input_string = if input_string == "-".to_owned() {
        if let Some(instr) = stdin.lock().lines().next() {
            match instr {
                Ok(instr2) => instr2,
                Err(e) => usage(&format!(
                    "Error reading version from stdin: {}",
                    e.to_string()
                )),
            }
        } else {
            usage("Error reading version from stdin: no data available");
        }
    } else {
        input_string
    };

    let use_v = if input_string.chars().nth(0) == Some('v') {
        input_string = input_string.chars().skip(1).collect();
        "v"
    } else {
        ""
    };

    let mut build_string: Option<String> = None;
    let mut pre_release_version: Option<String> = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_ref() {
            "-b" => {
                if let Some(bs) = iter.next() {
                    build_string = Some(bs);
                } else {
                    usage("Missing build_string after -b");
                }
            }
            "-p" => {
                if let Some(prv) = iter.next() {
                    pre_release_version = Some(prv);
                } else {
                    usage("Missing pre-release_version after -p");
                }
            }
            _ => usage(&format!("Invalid flag/option: {}", arg)),
        }
    }

    let mut parsed_version = match semver::Version::parse(&input_string) {
        Ok(v) => v,
        Err(e) => usage(&format!(
            "Unable to parse version: '{}', {}",
            input_string,
            e.to_string()
        )),
    };

    let mut version = match command {
        Command::Major => {
            parsed_version.increment_major();
            parsed_version
        }
        Command::Minor => {
            parsed_version.increment_minor();
            parsed_version
        }
        Command::Patch => {
            parsed_version.increment_patch();
            parsed_version
        }
    };

    if let Some(bs) = build_string {
        version.build = vec![semver::Identifier::AlphaNumeric(bs)];
    }

    if let Some(prv) = pre_release_version {
        version.pre = vec![semver::Identifier::AlphaNumeric(prv)];
    }

    println!("{}{}", use_v, version.to_string());
}
