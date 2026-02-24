use {
  crate::{
    ast::{
      Attribute, AttributeStatement, AttributeTarget, EdgeOperation,
      EdgeStatement, EdgeTarget, Graph, GraphKind, Id, NodeId, NodeStatement,
      Port, Statement, Subgraph,
    },
    lexer::Span,
    token::Token,
  },
  ariadne::{Label, Report, ReportKind, Source},
  chumsky::{
    input::{InputRef, ValueInput},
    prelude::*,
  },
  std::{
    env,
    fmt::{self, Display, Formatter},
    fs,
    ops::Range,
    process,
  },
};

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

fn main() {
  let path = env::args().nth(1).unwrap_or_else(|| {
    eprintln!("usage: network <file>");
    process::exit(1);
  });

  let src = fs::read_to_string(&path).unwrap_or_else(|err| {
    eprintln!("{path}: {err}");
    process::exit(1);
  });

  let ast = parser::parse(&src).unwrap_or_else(|parse_errors| {
    for error in &parse_errors {
      let span = error.span.clone();

      Report::build(ReportKind::Error, (path.as_str(), span.clone()))
        .with_message(&error.message)
        .with_label(
          Label::new((path.as_str(), span)).with_message(&error.message),
        )
        .finish()
        .eprint((path.as_str(), Source::from(&src)))
        .unwrap();
    }

    process::exit(1);
  });

  println!("{ast:#?}");
}
