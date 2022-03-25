use clap::{App, Arg};
use std::{thread, time::Duration};
use std::fs;
use enigo::{Enigo, KeyboardControllable};

fn main() {
    let matches = App::new("Keyboard Emulator")
        .version("1.0.0")
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
        .get_matches();
    let delay = matches.value_of("delay").unwrap().parse::<u64>().unwrap_or(0);
    let parse_flag = matches.is_present("parse");
    let file = matches.value_of("file").unwrap_or("");
    let content: String;
    if !file.is_empty() {
        content = fs::read_to_string(file).expect("error reading 'file' contents");
    } else {
        content = matches.value_of("code").unwrap().to_string();
    }
    let command = Some(content.trim_matches(|c| c == '\'')).expect("error trim code");
    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_secs(delay));
    if parse_flag {
        let command_unicode = command
            .replace("{CR}", "\r")
            .replace("{LF}", "\n");
        enigo.key_sequence_parse(command_unicode.as_str());
    } else {
        enigo.key_sequence(command);
    }
}
