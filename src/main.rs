use {
  crate::token::Token,
  ariadne::{Label, Report, ReportKind, Source},
  chumsky::{input::InputRef, prelude::*},
  std::{
    env,
    fmt::{self, Display, Formatter},
    fs, process,
  },
};

mod lexer;
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

  let (tokens, errors) = lexer::lexer().parse(&src).into_output_errors();

  for error in &errors {
    let span = error.span().into_range();

    Report::build(ReportKind::Error, (path.as_str(), span.clone()))
      .with_message(error.to_string())
      .with_label(
        Label::new((path.as_str(), span)).with_message(error.to_string()),
      )
      .finish()
      .eprint((path.as_str(), Source::from(&src)))
      .unwrap();
  }

  if !errors.is_empty() {
    std::process::exit(1);
  }

  for (token, span) in tokens.unwrap() {
    println!("{span}: {token}");
  }
}
