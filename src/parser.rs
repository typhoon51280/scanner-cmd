use nom::{
  IResult,
  error::{VerboseError, context},
  bytes::complete::{tag},
  character::complete::{u16, anychar},
  sequence::{delimited, preceded},
  multi::{many1, many0},
  combinator::{value, map},
  branch::{alt},
};
use enigo::Key;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

enum Token {
  Unicode(String),
  KeyClick(Key),
  KeyUp(Key),
  KeyDown(Key),
}

struct KeyboardInputs {
  tokens: Vec<Token>
}

fn key_fn(input: &str) -> Res<&str, Key> {
  context("F1-F12",
    preceded(
      tag("F"),
      alt((
        value(Key::F1, tag("1")),
        value(Key::F2, tag("2")),
        value(Key::F3, tag("3")),
        value(Key::F4, tag("4")),
        value(Key::F5, tag("5")),
        value(Key::F6, tag("6")),
        value(Key::F7, tag("7")),
        value(Key::F8, tag("8")),
        value(Key::F9, tag("9")),
        value(Key::F10, tag("10")),
        value(Key::F11, tag("11")),
        value(Key::F12, tag("12")),
      ))
    )
  )(input)
}

fn key_space(input: &str) -> Res<&str, Key> {
  context("Key::<Return|Delete|Backspace|Space|Tab>",
    alt((
        value(Key::Return, tag("Return")),
        value(Key::Delete, tag("Delete")),
        value(Key::Backspace, tag("Backspace")),
        value(Key::Space, tag("Space")),
        value(Key::Tab, tag("Tab")),
    ))
  )(input)
}

fn key_meta(input: &str) -> Res<&str, Key> {
  context("Key::<Alt|CapsLock|Control|Escape|Meta|Option|Shift>",
    alt((
      value(Key::Alt, tag("Alt")),
      value(Key::CapsLock, tag("CapsLock")),
      value(Key::Control, tag("Control")),
      value(Key::Escape, tag("Escape")),
      value(Key::Meta, tag("Meta")),
      value(Key::Option, tag("Option")),
      value(Key::Shift, tag("Shift")),
    ))
  )(input)
}

fn key_move(input: &str) -> Res<&str, Key> {
  context("Key::<Home|End|PageDown|PageUp>",
    alt((
      value(Key::Home, tag("Home")),
      value(Key::End, tag("End")),
      value(Key::PageDown, tag("PageDown")),
      value(Key::PageUp, tag("PageUp")),
    ))
  )(input)
}

fn key_arrow(input: &str) -> Res<&str, Key> {
  context("Key::<UpArrow|DownArrow|LeftArrow|RightArrow>",
    alt((
      value(Key::UpArrow, tag("UpArrow")),
      value(Key::DownArrow, tag("DownArrow")),
      value(Key::LeftArrow, tag("LeftArrow")),
      value(Key::RightArrow, tag("RightArrow")),
    ))
  )(input)
}

fn key_raw(input: &str) -> Res<&str, Key> {
  context("Key::Raw::(<u16>)",
    map(
        preceded(
          tag("Raw::"),
          u16
        ),
        |result| Key::Raw(result)
      )
  )(input)
}

fn key_layout(input: &str) -> Res<&str, Vec<Key>> {
  context("Layout::(<chars>)",
    delimited(
      tag("Layout::("),
      many1(map(anychar, |c| Key::Layout(c))),
      tag(")")
    )
  )(input)
}

fn key_button(input: &str) -> Res<&str, Vec<Key>> {
  context("[[Key::???]]",
  delimited(
    tag("[[Key::"),
      alt((
        map(
          alt((key_space, key_meta, key_arrow, key_move, key_fn, key_raw)),
          |key| vec![key]
        ),
        key_layout
      )),
    tag("]]")
    )
  )(input)
}

fn key_input(input: &str) -> Res<&str, Vec<Token>> {
  context("",
    delimited(
      tag("{{KeyInput}}"),
      map(many1(key_button), |keys| keys.into_iter().flatten().map(|key| Token::KeyClick(key)).collect()),
      tag("{{/KeyInput}}")
    )
  )(input)
}

fn key_down(input: &str) -> Res<&str, Vec<Token>> {
  context("",
    delimited(
      tag("{{KeyDown}}"),
      map(key_button, |keys| keys.into_iter().map(|key| Token::KeyDown(key)).collect()),
      tag("{{/KeyDown}}")
    )
  )(input)
}

fn key_up(input: &str) -> Res<&str, Vec<Token>> {
  context("",
    delimited(
      tag("{{KeyUp}}"),
      map(key_button, |keys| keys.into_iter().map(|key| Token::KeyUp(key)).collect()),
      tag("{{/KeyUp}}")
    )
  )(input)
}

fn key_sequence(input: &str) -> Res<&str, Vec<Token>> {
  context("",
    map(
      many1(
        alt((
          alt((key_input, key_down, key_up)),
          map(many0(anychar), |chars| vec![Token::Unicode(chars.into_iter().collect())])
        ))
      ),
      |tokens| tokens.into_iter().flatten().collect()
    )
  )(input)
}
