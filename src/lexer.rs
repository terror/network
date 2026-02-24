use super::*;

pub(crate) type Span = SimpleSpan<usize>;
pub(crate) type Spanned<'src> = (Token<'src>, Span);

pub(crate) fn lex(src: &str) -> ParseResult<Vec<Spanned<'_>>, Rich<'_, char>> {
  lexer().parse(src)
}

fn lexer<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<'src>>, extra::Err<Rich<'src, char>>>
{
  let digits = text::digits(10).to_slice();

  let number = just('-')
    .or_not()
    .then(
      just('.').then(digits).ignored().or(
        digits
          .then(just('.').then(digits.or_not()).or_not())
          .ignored(),
      ),
    )
    .to_slice()
    .map(Token::Number);

  let escape = just('\\').then(any());

  let string = just('"')
    .then(none_of("\"\\").ignored().or(escape.ignored()).repeated())
    .then(just('"'))
    .to_slice()
    .map(|string: &str| Token::String(&string[1..string.len() - 1]));

  let html_string = custom(|input: &mut InputRef<'src, '_, &'src str, _>| {
    let before = input.cursor();

    match input.next() {
      Some('<') => {}
      found => {
        let span = input.span_since(&before);

        return Err(Rich::custom(
          span,
          format!(
            "expected '<', found {}",
            found.map_or("end of input".into(), |c: char| format!("'{c}'"))
          ),
        ));
      }
    }

    let mut depth = 1u32;

    while depth > 0 {
      match input.next() {
        Some('<') => depth += 1,
        Some('>') => depth -= 1,
        Some(_) => {}
        None => {
          return Err(Rich::custom(
            input.span_since(&before),
            "unclosed HTML string",
          ));
        }
      }
    }

    let slice = input.slice_since(&before..);

    Ok(Token::HtmlString(&slice[1..slice.len() - 1]))
  });

  let ident = text::ascii::ident().map(|string: &str| match string {
    _ if string.eq_ignore_ascii_case("strict") => Token::Strict,
    _ if string.eq_ignore_ascii_case("graph") => Token::Graph,
    _ if string.eq_ignore_ascii_case("digraph") => Token::Digraph,
    _ if string.eq_ignore_ascii_case("node") => Token::Node,
    _ if string.eq_ignore_ascii_case("edge") => Token::Edge,
    _ if string.eq_ignore_ascii_case("subgraph") => Token::Subgraph,
    _ => Token::Ident(string),
  });

  let arrow = just("->").to(Token::Arrow);
  let dashdash = just("--").to(Token::DashDash);

  let punct = choice((
    just('{').to(Token::OpenBrace),
    just('}').to(Token::CloseBrace),
    just('[').to(Token::OpenBracket),
    just(']').to(Token::CloseBracket),
    just(';').to(Token::Semicolon),
    just(',').to(Token::Comma),
    just('=').to(Token::Equals),
    just(':').to(Token::Colon),
  ));

  let line_comment = just("//")
    .then(any().and_is(just('\n').not()).repeated())
    .ignored()
    .padded();

  let block_comment = just("/*")
    .then(any().and_is(just("*/").not()).repeated())
    .then(just("*/"))
    .ignored()
    .padded();

  let comment = line_comment.or(block_comment).repeated();

  let token =
    choice((arrow, dashdash, number, string, html_string, ident, punct));

  comment
    .ignore_then(
      token
        .map_with(|tok, e| (tok, e.span()))
        .padded()
        .padded_by(comment)
        .repeated()
        .collect(),
    )
    .then_ignore(end())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn comments() {
    assert_eq!(lex("// foo\nbar"), vec![Token::Ident("bar")]);
    assert_eq!(lex("/* foo */ bar"), vec![Token::Ident("bar")]);
    assert_eq!(lex("/* foo\nbar */ baz"), vec![Token::Ident("baz")]);
  }

  #[test]
  fn edge_operators() {
    assert_eq!(lex_one("->"), Token::Arrow);
    assert_eq!(lex_one("--"), Token::DashDash);
  }

  #[test]
  fn empty() {
    assert_eq!(lex(""), Vec::<Token>::new());
  }

  #[test]
  fn error_on_invalid() {
    assert!(super::lex("@").into_result().is_err());
  }

  #[test]
  fn graph() {
    assert_eq!(
      lex("digraph { a -> b }"),
      vec![
        Token::Digraph,
        Token::OpenBrace,
        Token::Ident("a"),
        Token::Arrow,
        Token::Ident("b"),
        Token::CloseBrace,
      ],
    );
  }

  #[test]
  fn html_strings() {
    #[track_caller]
    fn case<'a>(input: &'a str, expected: &'a str) {
      assert_eq!(lex_one(input), Token::HtmlString(expected));
    }

    case("<>", "");
    case("<foo>", "foo");
    case("<<b>foo</b>>", "<b>foo</b>");
    case(
      "<<table><tr><td>foo</td></tr></table>>",
      "<table><tr><td>foo</td></tr></table>",
    );
  }

  #[test]
  fn identifiers() {
    #[track_caller]
    fn case<'a>(input: &'a str, expected: &'a str) {
      assert_eq!(lex_one(input), Token::Ident(expected));
    }

    case("foo", "foo");
    case("_bar", "_bar");
    case("a1", "a1");
    case("node_name", "node_name");
  }

  #[test]
  fn keywords() {
    #[track_caller]
    fn case(input: &str, expected: Token<'_>) {
      assert_eq!(lex_one(input), expected);
    }

    case("strict", Token::Strict);
    case("graph", Token::Graph);
    case("digraph", Token::Digraph);
    case("node", Token::Node);
    case("edge", Token::Edge);
    case("subgraph", Token::Subgraph);
  }

  #[test]
  fn keywords_case_insensitive() {
    #[track_caller]
    fn case(input: &str, expected: Token<'_>) {
      assert_eq!(lex_one(input), expected);
    }

    case("STRICT", Token::Strict);
    case("Graph", Token::Graph);
    case("DIGRAPH", Token::Digraph);
    case("NODE", Token::Node);
    case("Edge", Token::Edge);
    case("SUBGRAPH", Token::Subgraph);
  }

  fn lex(input: &str) -> Vec<Token<'_>> {
    lexer()
      .parse(input)
      .into_result()
      .unwrap()
      .into_iter()
      .map(|(tok, _)| tok)
      .collect()
  }

  fn lex_one(input: &str) -> Token<'_> {
    let tokens = lex(input);
    assert_eq!(tokens.len(), 1);
    tokens.into_iter().next().unwrap()
  }

  #[test]
  fn numbers() {
    #[track_caller]
    fn case<'a>(input: &'a str, expected: &'a str) {
      assert_eq!(lex_one(input), Token::Number(expected));
    }

    case("0", "0");
    case("42", "42");
    case("3.14", "3.14");
    case(".5", ".5");
    case("-1", "-1");
    case("-3.14", "-3.14");
    case("-.5", "-.5");
    case("1.", "1.");
  }

  #[test]
  fn punctuation() {
    assert_eq!(
      lex("{ } [ ] ; , = :"),
      vec![
        Token::OpenBrace,
        Token::CloseBrace,
        Token::OpenBracket,
        Token::CloseBracket,
        Token::Semicolon,
        Token::Comma,
        Token::Equals,
        Token::Colon,
      ],
    );
  }

  #[test]
  fn strings() {
    #[track_caller]
    fn case<'a>(input: &'a str, expected: &'a str) {
      assert_eq!(lex_one(input), Token::String(expected));
    }

    case(r#""""#, "");
    case(r#""foo""#, "foo");
    case(r#""foo bar""#, "foo bar");
    case(r#""foo\"bar""#, r#"foo\"bar"#);
    case(r#""foo\\bar""#, r"foo\\bar");
  }
}
