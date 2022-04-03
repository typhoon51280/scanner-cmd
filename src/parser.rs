use nom::{
  IResult,
  Parser,
  bytes::complete::take_until1,
  character::complete::{u16, space0, anychar},
  multi::{many1, many_till},
  branch::alt,
};
use nom_supreme::{
  ParserExt,
  error::ErrorTree,
  tag::complete::tag,
};
use enigo::Key;

type Res<T, U> = IResult<T, U, ErrorTree<T>>;

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
  Text(String),
  KeyInput(Key),
  KeyUp(Key),
  KeyDown(Key),
}

fn key_fn(input: &str) -> Res<&str, Key> {
  alt((
    tag("1").context("Key::F1").value(Key::F1),
    tag("2").context("Key::F2").value(Key::F2),
    tag("3").context("Key::F3").value(Key::F3),
    tag("4").context("Key::F4").value(Key::F4),
    tag("5").context("Key::F5").value(Key::F5),
    tag("6").context("Key::F6").value(Key::F6),
    tag("7").context("Key::F7").value(Key::F7),
    tag("8").context("Key::F8").value(Key::F8),
    tag("9").context("Key::F9").value(Key::F9),
    tag("10").context("Key::F10").value(Key::F10),
    tag("11").context("Key::F11").value(Key::F11),
    tag("12").context("Key::F12").value(Key::F12),
  ))
  .preceded_by(tag("F"))
  .parse(input)
}

fn key_other(input: &str) -> Res<&str, Key> {
  alt((
    // SPACES
    tag("Return").context("Key::Return").value(Key::Return),
    tag("Delete").context("Key::Delete").value(Key::Delete),
    tag("Backspace").context("Key::Backspace").value(Key::Backspace),
    tag("Space").context("Key::Space").value(Key::Space),
    tag("Tab").context("Key::Tab").value(Key::Tab),
    // META
    tag("Alt").context("Key::Alt").value(Key::Alt),
    tag("CapsLock").context("Key::CapsLock").value(Key::CapsLock),
    tag("Control").context("Key::Control").value(Key::Control),
    tag("Escape").context("Key::Escape").value(Key::Escape),
    tag("Meta").context("Key::Meta").value(Key::Meta),
    tag("Option").context("Key::Option").value(Key::Option),
    tag("Shift").context("Key::Shift").value(Key::Shift),
    // MOVEMENT
    tag("Home").context("Key::Home").value(Key::Home),
    tag("End").context("Key::End").value(Key::End),
    tag("PageDown").context("Key::PageDown").value(Key::PageDown),
    tag("PageUp").context("Key::PageUp").value(Key::PageUp),
    tag("UpArrow").context("Key::UpArrow").value(Key::UpArrow),
    tag("DownArrow").context("Key::DownArrow").value(Key::DownArrow),
    tag("LeftArrow").context("Key::LeftArrow").value(Key::LeftArrow),
    tag("RightArrow").context("Key::RightArrow").value(Key::RightArrow),
  ))
  .parse(input)
}

fn key_raw(input: &str) -> Res<&str, Key> {
  u16
  .preceded_by(tag("Raw("))
  .terminated(tag(")"))
  .map(|result| Key::Raw(result))
  .parse(input)
}

fn key_layout(input: &str) -> Res<&str, Vec<Key>> {
  take_until1(")")
  .preceded_by(tag("Layout("))
  .terminated(tag(")"))
  .map(|s: &str| s.chars().map(|c| Key::Layout(c)).collect())
  .parse(input)
}

fn key_button(input: &str) -> Res<&str, Vec<Key>> {
  alt((
    alt((key_fn, key_other, key_raw)).map(|key| vec![key]),
    key_layout
  ))
  .preceded_by(tag("Key::"))
  .preceded_by(bracket_open)
  .terminated(bracket_close)
  .parse(input)
}

fn bracket_open(input: &str) -> Res<&str, &str> {
  tag("[[")
  .terminated(space0)
  .parse(input)
}

fn bracket_close(input: &str) -> Res<&str, &str> {
  tag("]]")
  .preceded_by(space0)
  .parse(input)
}

fn graph_open(input: &str) -> Res<&str, &str> {
  tag("{{")
  .terminated(space0)
  .parse(input)
}

fn graph_close(input: &str) -> Res<&str, &str> {
  tag("}}")
  .preceded_by(space0)
  .parse(input)
}

fn tag_close(input: &str) -> Res<&str, &str> {
  tag("/")
  .preceded_by(graph_open)
  .terminated(space0)
  .parse(input)
}

fn key_input_open(input: &str) -> Res<&str, &str> {
  tag("KeyInput")
  .preceded_by(graph_open)
  .terminated(graph_close)
  .parse(input)
}

fn key_input_close(input: &str) -> Res<&str, &str> {
  tag("KeyInput")
  .preceded_by(tag_close)
  .terminated(graph_close)
  .parse(input)
}

fn key_input(input: &str) -> Res<&str, Vec<Token>> {
  many1(key_button.delimited_by(space0))
  .preceded_by(key_input_open)
  .terminated(key_input_close)
  .map(|keys| keys.into_iter().flatten().map(|key| Token::KeyInput(key)).collect())
  .parse(input)
}

fn key_down_open(input: &str) -> Res<&str, &str> {
  tag("KeyDown")
  .preceded_by(graph_open)
  .terminated(graph_close)
  .parse(input)
}

fn key_down_close(input: &str) -> Res<&str, &str> {
  tag("KeyDown")
  .preceded_by(tag_close)
  .terminated(graph_close)
  .parse(input)
}

fn key_down(input: &str) -> Res<&str, Vec<Token>> {
  many1(key_button)
  .preceded_by(key_down_open)
  .terminated(key_down_close)
  .map(|keys| keys.into_iter().flatten().map(|key| Token::KeyDown(key)).collect())
  .parse(input)
}

fn key_up_open(input: &str) -> Res<&str, &str> {
  tag("KeyUp")
  .preceded_by(graph_open)
  .terminated(graph_close)
  .parse(input)
}

fn key_up_close(input: &str) -> Res<&str, &str> {
  tag("KeyUp")
  .preceded_by(tag_close)
  .terminated(graph_close)
  .parse(input)
}

fn key_up(input: &str) -> Res<&str, Vec<Token>> {
  many1(key_button)
  .preceded_by(key_up_open)
  .terminated(key_up_close)
  .map(|keys| keys.into_iter().flatten().map(|key| Token::KeyUp(key)).collect())
  .parse(input)
}

fn text_open(input: &str) -> Res<&str, &str> {
  tag("Text")
  .preceded_by(graph_open)
  .terminated(graph_close)
  .parse(input)
}

fn text_close(input: &str) -> Res<&str, &str> {
  tag("Text")
  .preceded_by(tag_close)
  .terminated(graph_close)
  .parse(input)
}

fn text(input: &str) -> Res<&str, Vec<Token>> {
  text_open
  .precedes(many_till(anychar, tag("{{").peek()))
  .terminated(text_close)
  .map(|(chars, _)| vec![Token::Text(chars.into_iter().collect::<String>())])
  .parse(input)
}

fn keyboard(input: &str) -> Res<&str, Vec<Token>> {
  many1(
    alt((
      key_input,
      key_down,
      key_up,
      text,
    )).delimited_by(space0)
  ).context("keyboard")
  .map(|tokens| tokens.into_iter().flatten().collect())
  .all_consuming()
  .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Key::<Return|Delete|Backspace|Space|Tab>
    fn test_key_input_space() {
      assert_eq!(keyboard("{{KeyInput}}[[Key::Return]]{{/KeyInput}}").unwrap(), ("", vec![Token::KeyInput(Key::Return)]));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Delete]]{{/KeyInput}}").unwrap(), ("", vec![Token::KeyInput(Key::Delete)]));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Backspace]]{{/KeyInput}}").unwrap(), ("", vec![Token::KeyInput(Key::Backspace)]));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Space]]{{/KeyInput}}").unwrap(), ("", vec![Token::KeyInput(Key::Space)]));
      assert_eq!(keyboard("{{KeyInput}}[[Key::Tab]]{{/KeyInput}}").unwrap(), ("", vec![Token::KeyInput(Key::Tab)]));
      assert_eq!(
        keyboard("\
          {{KeyInput}}\
          [[Key::Return]]\
          [[Key::Delete]]\
          [[Key::Backspace]]\
          [[Key::Space]]\
          [[Key::Tab]]\
          {{/KeyInput}}"
        ).unwrap(),
        ("",vec![
          Token::KeyInput(Key::Return),
          Token::KeyInput(Key::Delete),
          Token::KeyInput(Key::Backspace),
          Token::KeyInput(Key::Space),
          Token::KeyInput(Key::Tab)
        ])
      );
    }

    #[test]
    fn test_text() {
      assert_eq!(
        keyboard("{{Text}}hello world{{/Text}}").expect("Parser Error"),
        ("", vec![Token::Text(String::from("hello world"))])
      );
    }

    #[test]
    fn test_multi_spaces() {
      assert_eq!(
        keyboard("\
          {{ Text }}hello{{ / Text }} \
          {{ KeyInput  }} \
            [[Key::Layout( )  ]] \
          {{   / KeyInput }} \
          {{ Text }}world{{ / Text }}"
          ).expect("Parser Error"),
        ("", vec![
          Token::Text(String::from("hello")),
          Token::KeyInput(Key::Layout(' ')),
          Token::Text(String::from("world"))
        ])
      );
    }
}