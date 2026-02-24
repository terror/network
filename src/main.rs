use {
  ariadne::{Label, Report, ReportKind, Source},
  network::parse,
  std::{env, fs, process},
};

fn main() {
  let path = env::args().nth(1).unwrap_or_else(|| {
    eprintln!("usage: network <file>");
    process::exit(1);
  });

  let src = fs::read_to_string(&path).unwrap_or_else(|err| {
    eprintln!("{path}: {err}");
    process::exit(1);
  });

  let ast = parse(&src).unwrap_or_else(|parse_errors| {
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
