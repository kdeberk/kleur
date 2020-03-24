#[macro_use]
extern crate error_chain;
extern crate regex;

use std::env;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use regex::Captures;

mod config;
mod errors;

use crate::errors::{Error, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if 1 == args.len() {
        return Err(Error::from("No command was specified"))
    }

    let rules: Vec<config::Rule>;
    match config::rules_for_cli_command(&args[1..].join(" ")) {
        Err(x) => {
            return Err(x);
        },
        Ok(None) => {
            rules = Vec::new();
        }
        Ok(Some(found)) => {
            rules = found;
        }
    }

    let stdout = Command::new(&args[1])
        .args(&args[2..])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::from("Failed to execute command"))?;
    let reader = BufReader::new(stdout);

    if 0 == rules.len() {
        println!("WARNING: Kleur found no rules for this invocation");
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line))
    } else {
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", colorize_line(&line, &rules)));
    }

    Ok(())
}

fn colorize_line<'a>(line: &'a str, rules: &Vec<config::Rule>) -> String {
    let mut line = line.to_owned();

    for rule in rules {
        line = rule.regex.replace(&line, |caps: &Captures| {
            format!("\x1b[{}m{}\x1b[0m", rule.color_code, &caps[0])
        }).into_owned();
    }

    line
}
