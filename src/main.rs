#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate os_pipe;

use std::env;
use std::process::Command;
use std::io::{BufRead, BufReader};
use regex::Captures;
use os_pipe::pipe;

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
            println!("WARNING: Kleur found no rules for this invocation");
            rules = Vec::new();
        }
        Ok(Some(found)) => {
            rules = found;
        }
    }

    let(reader, writer) = pipe()?;
    let mut child = Command::new(&args[1]);
    child.args(&args[2..]);
    child.stdout(writer.try_clone()?);
    child.stderr(writer);

    let mut handle = child.spawn()
        .expect("Failed to execute command");
    drop(child);

    let reader = BufReader::new(reader);
    if 0 == rules.len() {
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
    handle.wait()?;

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
