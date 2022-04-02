use nom::{
  IResult,
  error::{VerboseError, context},
  bytes::complete::{tag, take_until1},
  character::complete::{u16, space0},
  sequence::{delimited, preceded, terminated},
  multi::{many1},
  combinator::{value, map},
  branch::{alt},
};
use enigo::Key;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(PartialEq,Debug)]
enum Token {
  Unicode(String),
  KeyInput(Key),
  KeyUp(Key),
  KeyDown(Key),
}

// struct KeyboardInputs {
//   tokens: Vec<Token>
// }

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
  context("Key::(<chars>)",
    delimited(
      tag("("),
      map(take_until1(")"), |s: &str| s.chars().map(|c| Key::Layout(c)).collect()),
      tag(")")
    )
  )(input)
}

fn bracket_open(input: &str) -> Res<&str, &str> {
  context("[[",
    preceded(
      space0,
      tag("[["),
    )
  )(input)
}

fn bracket_close(input: &str) -> Res<&str, &str> {
  context("]]",
  terminated(
      tag("]]"),
      space0
    )
  )(input)
}

fn graph_open(input: &str) -> Res<&str, &str> {
  context("graph open",
    preceded(
      space0,
      tag("{{"),
    )
  )(input)
}

fn graph_close(input: &str) -> Res<&str, &str> {
  context("graph close",
    terminated(
      tag("}}"),
      space0
    )
  )(input)
}

fn key_prefix(input: &str) -> Res<&str, &str> {
  context("Key::",
    preceded(
      bracket_open,
      tag("Key::")
    )
  )(input)
}

fn key_button(input: &str) -> Res<&str, Vec<Key>> {
  context("[[Key::???]]",
  delimited(
    key_prefix,
      alt((
        map(
          alt((key_space, key_meta, key_arrow, key_move, key_fn, key_raw)),
          |key| vec![key]
        ),
        key_layout
      )),
      bracket_close
    )
  )(input)
}

fn key_input_open(input: &str) -> Res<&str, &str> {
  context("key input open",
    delimited(
      graph_open,
        tag("KeyInput"),
        graph_close
    )
  )(input)
}

fn key_input_close(input: &str) -> Res<&str, &str> {
  context("key input close",
    delimited(
      graph_open,
        tag("/KeyInput"),
        graph_close
    )
  )(input)
}

fn key_input(input: &str) -> Res<&str, Vec<Token>> {
  context("key input",
    delimited(
      key_input_open,
      alt((
        map(many1(key_button), |keys| keys.into_iter().flatten().map(|key| Token::KeyInput(key)).collect()),
      )),
      key_input_close
    )
  )(input)
}

fn key_down_open(input: &str) -> Res<&str, &str> {
  context("key down open",
    delimited(
      graph_open,
        tag("KeyDown"),
        graph_close
    )
  )(input)
}

fn key_down_close(input: &str) -> Res<&str, &str> {
  context("key down close",
    delimited(
      graph_open,
        tag("/KeyDown"),
        graph_close
    )
  )(input)
}

fn key_down(input: &str) -> Res<&str, Vec<Token>> {
  context("key down",
    delimited(
      key_down_open,
      map(key_button, |keys| keys.into_iter().map(|key| Token::KeyDown(key)).collect()),
      key_down_close
    )
  )(input)
}

fn key_up_open(input: &str) -> Res<&str, &str> {
  context("key up open",
    delimited(
      graph_open,
        tag("KeyUp"),
        graph_close
    )
  )(input)
}

fn key_up_close(input: &str) -> Res<&str, &str> {
  context("key up close",
    delimited(
      graph_open,
        tag("/KeyUp"),
        graph_close
    )
  )(input)
}

fn key_up(input: &str) -> Res<&str, Vec<Token>> {
  context("key up",
    delimited(
      key_up_open,
      map(key_button, |keys| keys.into_iter().map(|key| Token::KeyUp(key)).collect()),
      key_up_close
    )
  )(input)
}

fn text(input: &str) -> Res<&str, Vec<Token>> {
  context("text",
    delimited(
      tag("{{Text}}"),
      map(
        take_until1("{{/Text}}"),
        |chars: &str| vec![Token::Unicode(chars.to_string())]
      ),
      tag("{{/Text}}")
    )
  )(input)
}

fn keyboard(input: &str) -> Res<&str, Vec<Token>> {
  context("mutiple keyboard inputs",
    map(
      many1(
        alt((
          key_input,
          key_down,
          key_up,
          text
        )),
      ),
      |tokens| tokens.into_iter().flatten().collect()
    )
  )(input)
}

/**
fn verbose_error() {
  let data = "\
  {{Text}}hello world{{/Text}}";
  let result = keyboard(data);
  println!("parsed: {:?}", result);
  match result.borrow() {
    Err(Err::Error(e)) | Err(Err::Failure(e)) => {
      println!("verbose errors:\n{}", convert_error(data, e.to_owned()));
    }
    _ => {}
  }
  assert_eq!(result, Ok(("", vec![
    Token::Unicode(String::from("hello world"))
  ])));
}
**/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Key::<Return|Delete|Backspace|Space|Tab>
    fn key_input_space() {
      assert_eq!(keyboard("{{KeyInput}}[[Key::Return]]{{/KeyInput}}"), Ok(("", vec![Token::KeyInput(Key::Return)])));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Delete]]{{/KeyInput}}"), Ok(("", vec![Token::KeyInput(Key::Delete)])));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Backspace]]{{/KeyInput}}"), Ok(("", vec![Token::KeyInput(Key::Backspace)])));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Space]]{{/KeyInput}}"), Ok(("", vec![Token::KeyInput(Key::Space)])));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Tab]]{{/KeyInput}}"), Ok(("", vec![Token::KeyInput(Key::Tab)])));
      assert_eq!(
        keyboard("\
          {{KeyInput}}\
          [[Key::Return]]\
          [[Key::Delete]]\
          [[Key::Backspace]]\
          [[Key::Space]]\
          [[Key::Tab]]\
          {{/KeyInput}}"
        ),
        Ok(("",vec![
          Token::KeyInput(Key::Return),
          Token::KeyInput(Key::Delete),
          Token::KeyInput(Key::Backspace),
          Token::KeyInput(Key::Space),
          Token::KeyInput(Key::Tab)
        ]))
      );
    }

    #[test]
    fn text() {
      assert_eq!(keyboard("{{Text}}hello world{{/Text}}"), Ok(("", vec![Token::Unicode(String::from("hello world"))])));
    }

    #[test]
    fn mix_text_keylayout() {
      assert_eq!(
        keyboard("\
          {{Text}}hello{{/Text}} \
          {{KeyInput}} \
            [[Key::( )]] \
          {{/KeyInput}} \
          {{Text}}world{{/Text}}"
        ),
        Ok(("", vec![
          Token::Unicode(String::from("hello")),
          Token::KeyInput(Key::Layout(' ')),
          Token::Unicode(String::from("world"))
        ]))
      );
    }
}