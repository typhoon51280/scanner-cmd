use nom::{
  IResult,
  Parser,
  character::complete::{u16, space0, anychar},
  multi::{many1, many_till},
  branch::alt,
};
use nom_supreme::{
  ParserExt,
  error::ErrorTree,
  tag::complete::tag,
  final_parser::{final_parser, Location},
};
use enigo::Key;

type Res<T, U> = IResult<T, U, ErrorTree<T>>;

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
  Text(String),
  KeyClick(Key),
  KeyUp(Key),
  KeyDown(Key),
}

fn key_fn(input: &str) -> Res<&str, Key> {
  alt((
    tag("1").value(Key::F1),
    tag("2").value(Key::F2),
    tag("3").value(Key::F3),
    tag("4").value(Key::F4),
    tag("5").value(Key::F5),
    tag("6").value(Key::F6),
    tag("7").value(Key::F7),
    tag("8").value(Key::F8),
    tag("9").value(Key::F9),
    tag("10").value(Key::F10),
    tag("11").value(Key::F11),
    tag("12").value(Key::F12),
  ))
  .preceded_by(tag("F"))
  .parse(input)
}

fn key_other(input: &str) -> Res<&str, Key> {
  alt((
    // SPACES
    tag("Return").value(Key::Return),
    tag("Delete").value(Key::Delete),
    tag("Backspace").value(Key::Backspace),
    tag("Space").value(Key::Space),
    tag("Tab").value(Key::Tab),
    // META
    tag("Alt").value(Key::Alt),
    tag("CapsLock").value(Key::CapsLock),
    tag("Control").value(Key::Control),
    tag("Escape").value(Key::Escape),
    tag("Meta").value(Key::Meta),
    tag("Option").value(Key::Option),
    tag("Shift").value(Key::Shift),
    // MOVEMENT
    tag("Home").value(Key::Home),
    tag("End").value(Key::End),
    tag("PageDown").value(Key::PageDown),
    tag("PageUp").value(Key::PageUp),
    tag("UpArrow").value(Key::UpArrow),
    tag("DownArrow").value(Key::DownArrow),
    tag("LeftArrow").value(Key::LeftArrow),
    tag("RightArrow").value(Key::RightArrow),
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
  many_till(anychar.verify(|c| !c.is_uppercase()), tag(")").peek())
  .preceded_by(tag("Layout("))
  .terminated(tag(")"))
  .map(|(chars, _)| chars.into_iter().map(|c| Key::Layout(c)).collect())
  .parse(input)
}

fn key_button(input: &str) -> Res<&str, Vec<Key>> {
  alt((
    alt((
      key_fn.context("[[Key::F<n>]]"),
      key_other.context("[[Key::<Special>]]"),
      key_raw.context("[[Key::Raw(<u8>)]]"),
    )).map(|key| vec![key]),
    key_layout.context("[[Key::Layout(<chars>)]]")
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

fn key_click_open(input: &str) -> Res<&str, &str> {
  tag("KeyClick")
  .preceded_by(graph_open)
  .terminated(graph_close)
  .parse(input)
}

fn key_click_close(input: &str) -> Res<&str, &str> {
  tag("KeyClick")
  .preceded_by(tag_close)
  .terminated(graph_close)
  .parse(input)
}

fn key_click(input: &str) -> Res<&str, Vec<Token>> {
  many1(key_button.delimited_by(space0))
  .preceded_by(key_click_open)
  .terminated(key_click_close)
  .map(|keys| keys.into_iter().flatten().map(|key| Token::KeyClick(key)).collect())
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
      key_click.context("{{KeyClick}}"),
      key_down.context("{{KeyDown}}"),
      key_up.context("{{KeyUp}}"),
      text.context("{{Text}}"),
    ))
    .delimited_by(space0)
  )
  .map(|tokens| tokens.into_iter().flatten().collect())
  .parse(input)
}

pub fn parse(input: &str) -> Result<Vec<Token>, ErrorTree<Location>> {
  final_parser(keyboard)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Key::<Return|Delete|Backspace|Space|Tab>
    fn test_key_input_space() {
      assert_eq!(parse("{{KeyClick}}[[Key::Return]]{{/KeyClick}}").unwrap(), vec![Token::KeyClick(Key::Return)]);
      assert_eq!(parse("{{KeyClick}}[[Key::Delete]]{{/KeyClick}}").unwrap(), vec![Token::KeyClick(Key::Delete)]);
      assert_eq!(parse("{{KeyClick}}[[Key::Backspace]]{{/KeyClick}}").unwrap(), vec![Token::KeyClick(Key::Backspace)]);
      assert_eq!(parse("{{KeyClick}}[[Key::Space]]{{/KeyClick}}").unwrap(), vec![Token::KeyClick(Key::Space)]);
      assert_eq!(parse("{{KeyClick}}[[Key::Tab]]{{/KeyClick}}").unwrap(), vec![Token::KeyClick(Key::Tab)]);
      assert_eq!(
        parse("\
          {{KeyClick}}\
          [[Key::Return]]\
          [[Key::Delete]]\
          [[Key::Backspace]]\
          [[Key::Space]]\
          [[Key::Tab]]\
          {{/KeyClick}}"
        ).unwrap(),
        vec![
          Token::KeyClick(Key::Return),
          Token::KeyClick(Key::Delete),
          Token::KeyClick(Key::Backspace),
          Token::KeyClick(Key::Space),
          Token::KeyClick(Key::Tab)
        ]
      );
    }

    #[test]
    fn test_text() {
      assert_eq!(
        parse("{{Text}}hello world{{/Text}}").expect("Parser Error"),
        vec![Token::Text(String::from("hello world"))]
      );
    }

    #[test]
    fn test_multi_spaces() {
      assert_eq!(
        parse("\
          {{ Text }}hello{{ / Text }} \
          {{ KeyClick  }} \
            [[Key::Layout( )  ]] \
          {{   / KeyClick }} \
          {{ Text }}world{{ / Text }}"
          ).expect("Parser Error"),
        vec![
          Token::Text(String::from("hello")),
          Token::KeyClick(Key::Layout(' ')),
          Token::Text(String::from("world"))
        ]
      );
    }
}