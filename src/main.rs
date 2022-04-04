use clap::{Command, Arg};
use std::{thread, fs, time};
use enigo::{Enigo, KeyboardControllable};
use parser::{Token, parse};

mod parser;

fn main() {
    let matches = Command::new("Keyboard Emulator")
        .version("1.0.1")
        .author("Humanoid Typhoon <typhoon51280@users.noreply.github.com>")
        .about("Keyboard Input Simulator")
        .arg(
            Arg::new("code")
                .short('c')
                .long("code")
                .takes_value(true)
                .required(true)
                .forbid_empty_values(true)
                .conflicts_with("file")
                .help("Inline Text Input (single quote): 'ipselorumdixit'"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .required(true)
                .forbid_empty_values(true)
                .conflicts_with("code")
                .help("Filename Text Input"),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .takes_value(true)
                .required(false)
                .forbid_empty_values(true)
                .default_missing_value("0")
                .help("Delay before input simulation start"),
        )
        .arg(
            Arg::new("parse")
                .short('p')
                .long("parse")
                .takes_value(false)
                .required(false)
                .help("Parse Mode enabled"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .takes_value(false)
                .required(false)
                .help("Debug Mode"),
        )
        .get_matches();
    let delay = matches.value_of("delay").unwrap_or("0").parse::<u64>().unwrap_or(0);
    let parse_flag = matches.is_present("parse");
    let debug_flag = matches.is_present("debug");
    let file = matches.value_of("file").unwrap_or("");
    let content: String;
    if !file.is_empty() {
        content = fs::read_to_string(file).expect("error reading 'file' contents");
    } else {
        content = matches.value_of("code").unwrap().to_string();
    }
    let command = Some(content.trim_matches(|c| c == '\'')).expect("error trim code");
    let mut enigo = Enigo::new();
    thread::sleep(time::Duration::from_secs(delay));
    if parse_flag {
        let tokens = match parse(command) {
            Ok(tokens) => {
                if debug_flag {
                    println!("{:#?}", tokens);
                }
                tokens
            },
            Err(error) => {
                if debug_flag { 
                    panic!("{:#?}", error)
                } else {
                    panic!("Parsing Error")
                }
            }
        };
        for token in tokens {
            match token {
                Token::Text(text) =>  { enigo.key_sequence(text.as_str()) }
                Token::KeyClick(key) => { enigo.key_click(key) }
                Token::KeyDown(key) => { enigo.key_down(key) }
                Token::KeyUp(key) => { enigo.key_down(key) }
            }
        }
    } else {
        enigo.key_sequence(command);
    }
}
