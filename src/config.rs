use regex::Regex;
use serde::Deserialize;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;

use crate::errors::{Error, Result};

#[derive(Debug, Deserialize)]
struct RawCommand {
    command: String,
    rules: Vec<RawRule>,
}

#[derive(Debug, Deserialize)]
struct RawRule {
    regex: String,
    color: String,
}

pub struct Rule {
    pub regex: Regex,
    pub color_code: u8,
}

impl TryFrom<RawRule> for Rule {
    type Error = Error;

    fn try_from(raw: RawRule) -> Result<Self> {
        let regex = Regex::new(&format!("({})", &raw.regex))?;
        match color_code_for_color_name(&raw.color) {
            Some(color_code) => {
                Ok(Rule{regex: regex, color_code: color_code})
            },
            None => {
                Err(Error::from(format!("{} is not a known color", &raw.color)))
            }
        }
    }
}

pub fn rules_for_cli_command(cli_command: &str) -> Result<Option<Vec<Rule>>> {
    let home = std::env::var("HOME")?;
    let filename = format!("{}/.config/kleur/config.yml", home);
    let config = load_config(&filename)?; // TODO: allow missing file, don't allow yaml errors.

    for entry in config {
        if cli_command.starts_with(&entry.command) {
            let mut rules = Vec::new();

            for raw in entry.rules {
                let rule = Rule::try_from(raw)?;
                rules.push(rule);
            }

            return Ok(Some(rules))
        }
    }

    return Ok(None)
}

fn load_config(filename: &String) -> Result<Vec<RawCommand>> {
    let file = File::open(filename)?;

    let reader = BufReader::new(&file);
    let commands:Vec<RawCommand> = serde_yaml::from_reader(reader)?;
    return Ok(commands)
}

fn color_code_for_color_name(name: &str) -> Option<u8> {
    match name {
        "bold" => Some(1),  // Yes, bold is a color..
        "dim" => Some(2),
        "underlined" => Some(4),
        "blink" => Some(5),
        "inverted" => Some(6),
        "black" => Some(30),
        "red" => Some(31),
        "green" => Some(32),
        "yellow" => Some(33),
        "blue" => Some(34),
        "magenta" => Some(35),
        "cyan" => Some(36),
        "light gray" => Some(37),
        "dark gray" => Some(90),
        "light red" => Some(91),
        "light green" => Some(92),
        "light yellow" => Some(93),
        "light blue" => Some(94),
        "light magenta" => Some(95),
        "light cyan" => Some(96),
        "white" => Some(97),
        _ => None,
    }
}
