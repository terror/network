use {
  ast::{
    Attribute, AttributeStatement, AttributeTarget, EdgeOperation,
    EdgeStatement, EdgeTarget, Graph, GraphKind, Id, NodeId, NodeStatement,
    Port, Statement, Subgraph,
  },
  chumsky::{
    input::{InputRef, ValueInput},
    prelude::*,
  },
  lexer::Span,
  std::{
    fmt::{self, Display, Formatter},
    ops::Range,
  },
  token::Token,
};

pub use parser::ParseError;

#[macro_export]
macro_rules! assert_matches {
  ($expression:expr, $( $pattern:pat_param )|+ $( if $guard:expr )? $(,)?) => {
    match $expression {
      $( $pattern )|+ $( if $guard )? => {}
      left => panic!(
        "assertion failed: (left ~= right)\n  left: `{:?}`\n right: `{}`",
        left,
        stringify!($($pattern)|+ $(if $guard)?)
      ),
    }
  }
}

mod ast;
mod lexer;
mod parser;
mod token;

/// Parse Graphviz DOT source into a syntax tree.
///
/// This is the crate's primary public API. It tokenizes the input and then
/// parses it as a single DOT graph.
///
/// The returned [`Graph`] borrows all string slices directly from `src`, so
/// the source string must outlive the parsed value.
///
/// On failure, this function returns all lexer and parser errors it can
/// recover, each with:
///
/// - `message`: a human-readable description of the problem
/// - `span`: a byte range in the original source where the problem occurred
///
/// # Examples
///
/// ```
/// let graph = network::parse("digraph { foo -> bar }");
/// assert!(graph.is_ok());
/// ```
///
/// ```
/// let error = network::parse("digraph {").unwrap_err();
/// assert!(!error.is_empty());
/// ```
pub fn parse(src: &str) -> Result<Graph<'_>, Vec<ParseError>> {
  parser::parse(src)
}
