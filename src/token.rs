use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token<'src> {
  Arrow,
  CloseBrace,
  CloseBracket,
  Colon,
  Comma,
  DashDash,
  Digraph,
  Edge,
  Equals,
  Graph,
  HtmlString(&'src str),
  Ident(&'src str),
  Node,
  Number(&'src str),
  OpenBrace,
  OpenBracket,
  Semicolon,
  Strict,
  String(&'src str),
  Subgraph,
}

impl Display for Token<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Token::Arrow => write!(f, "->"),
      Token::CloseBrace => write!(f, "}}"),
      Token::CloseBracket => write!(f, "]"),
      Token::Colon => write!(f, ":"),
      Token::Comma => write!(f, ","),
      Token::DashDash => write!(f, "--"),
      Token::Digraph => write!(f, "digraph"),
      Token::Edge => write!(f, "edge"),
      Token::Equals => write!(f, "="),
      Token::Graph => write!(f, "graph"),
      Token::HtmlString(s) => write!(f, "<{s}>"),
      Token::Ident(s) | Token::Number(s) => {
        write!(f, "{s}")
      }
      Token::Node => write!(f, "node"),
      Token::OpenBrace => write!(f, "{{"),
      Token::OpenBracket => write!(f, "["),
      Token::Semicolon => write!(f, ";"),
      Token::Strict => write!(f, "strict"),
      Token::String(s) => write!(f, "\"{s}\""),
      Token::Subgraph => write!(f, "subgraph"),
    }
  }
}
