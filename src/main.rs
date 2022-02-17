use clap::{App, Arg};
use enigo::{Enigo, KeyboardControllable};
use std::{thread, time::Duration};

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
                .help("Qr Code"),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .takes_value(true)
                .help("Delay before input simulation start"),
        )
        .get_matches();
    let code = matches.value_of("code").unwrap_or("");
    let delay = matches.value_of("delay").unwrap_or("0").parse::<u64>().unwrap_or(0);
    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_secs(delay));
    enigo.key_sequence(Some(code.trim_matches(|c| c == '\'')).unwrap_or(""));
}
