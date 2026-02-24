use super::*;

#[derive(Debug)]
pub struct ParseError {
  pub message: String,
  pub span: Range<usize>,
}

pub(crate) fn parse(src: &str) -> Result<Graph<'_>, Vec<ParseError>> {
  let tokens = lexer::lex(src).into_result().map_err(|errors| {
    errors
      .into_iter()
      .map(|error| ParseError {
        message: error.to_string(),
        span: error.span().into_range(),
      })
      .collect::<Vec<_>>()
  })?;

  let eoi: Span = (src.len()..src.len()).into();

  parser()
    .parse(tokens.as_slice().map(eoi, |(t, s)| (t, s)))
    .into_result()
    .map_err(|errors| {
      errors
        .into_iter()
        .map(|error| ParseError {
          message: error.to_string(),
          span: error.span().into_range(),
        })
        .collect::<Vec<_>>()
    })
}

fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Graph<'src>, extra::Err<Rich<'tokens, Token<'src>, Span>>>
where
  I: ValueInput<'tokens, Token = Token<'src>, Span = Span>,
{
  let id = select! {
    Token::Ident(s) => Id::Ident(s),
    Token::String(s) => Id::String(s),
    Token::HtmlString(s) => Id::HtmlString(s),
    Token::Number(s) => Id::Number(s),
  };

  let port = just(Token::Colon)
    .ignore_then(id)
    .then(just(Token::Colon).ignore_then(id).or_not())
    .map(|(id, compass)| Port { compass, id });

  let node_id = id.then(port.or_not()).map(|(id, port)| NodeId { id, port });

  let attribute = id
    .then(just(Token::Equals).ignore_then(id).or_not())
    .then_ignore(just(Token::Semicolon).or(just(Token::Comma)).or_not())
    .map(|(key, value)| Attribute { key, value });

  let attribute_list = just(Token::OpenBracket)
    .ignore_then(attribute.repeated().collect::<Vec<_>>())
    .then_ignore(just(Token::CloseBracket))
    .repeated()
    .at_least(1)
    .collect::<Vec<Vec<Attribute>>>()
    .map(|lists| lists.into_iter().flatten().collect::<Vec<_>>());

  let edge_operation = select! {
    Token::Arrow => EdgeOperation::Arrow,
    Token::DashDash => EdgeOperation::DashDash,
  };

  let statement_list = recursive(|statement_list| {
    let subgraph = just(Token::Subgraph)
      .ignore_then(id.or_not())
      .or_not()
      .then(
        just(Token::OpenBrace)
          .ignore_then(statement_list)
          .then_ignore(just(Token::CloseBrace)),
      )
      .map(|(header, statements)| Subgraph {
        id: header.flatten(),
        statements,
      });

    let edge_target = node_id
      .clone()
      .map(EdgeTarget::NodeId)
      .or(subgraph.clone().map(EdgeTarget::Subgraph));

    let edge_rhs = edge_operation
      .then(edge_target.clone())
      .repeated()
      .at_least(1)
      .collect::<Vec<_>>();

    let edge_statement = edge_target
      .then(edge_rhs)
      .then(attribute_list.clone().or_not())
      .map(|((from, edges), attrs)| {
        Statement::Edge(EdgeStatement {
          attributes: attrs.unwrap_or_default(),
          edges,
          from,
        })
      });

    let attribute_target = select! {
      Token::Graph => AttributeTarget::Graph,
      Token::Node => AttributeTarget::Node,
      Token::Edge => AttributeTarget::Edge,
    };

    let attribute_statement = attribute_target
      .then(attribute_list.clone())
      .map(|(target, attrs)| {
        Statement::Attr(AttributeStatement {
          attributes: attrs,
          target,
        })
      });

    let assign = id
      .then_ignore(just(Token::Equals))
      .then(id)
      .map(|(key, value)| Statement::Assign(key, value));

    let node_statement =
      node_id.then(attribute_list.or_not()).map(|(id, attrs)| {
        Statement::Node(NodeStatement {
          attributes: attrs.unwrap_or_default(),
          id,
        })
      });

    let statement = choice((
      attribute_statement,
      edge_statement,
      assign,
      subgraph.map(Statement::Subgraph),
      node_statement,
    ));

    statement
      .then_ignore(just(Token::Semicolon).or_not())
      .repeated()
      .collect::<Vec<_>>()
  });

  let strict = just(Token::Strict).or_not().map(|s| s.is_some());

  let graph_kind = select! {
    Token::Graph => GraphKind::Graph,
    Token::Digraph => GraphKind::Digraph,
  };

  strict
    .then(graph_kind)
    .then(id.or_not())
    .then(
      just(Token::OpenBrace)
        .ignore_then(statement_list)
        .then_ignore(just(Token::CloseBrace)),
    )
    .then_ignore(end())
    .map(|(((strict, kind), id), statements)| Graph {
      id,
      kind,
      statements,
      strict,
    })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn anonymous_subgraph() {
    let ast = parse("digraph { { a } }");

    assert_matches!(
      &ast.statements[..],
      [Statement::Subgraph(Subgraph { id: None, statements })] if statements.len() == 1,
    );
  }

  #[test]
  fn assign_statement() {
    let ast = parse("digraph { label = \"hello\" }");

    assert_eq!(
      ast.statements,
      vec![Statement::Assign(Id::Ident("label"), Id::String("hello"),)],
    );
  }

  #[test]
  fn attr_statement() {
    let ast = parse("digraph { node [shape=circle] }");

    assert_eq!(
      ast.statements,
      vec![Statement::Attr(AttributeStatement {
        attributes: vec![Attribute {
          key: Id::Ident("shape"),
          value: Some(Id::Ident("circle")),
        }],
        target: AttributeTarget::Node,
      })],
    );
  }

  #[test]
  fn edge_chain() {
    let ast = parse("digraph { a -> b -> c }");

    assert_matches!(
      &ast.statements[..],
      [Statement::Edge(EdgeStatement { edges, .. })] if edges.len() == 2,
    );
  }

  #[test]
  fn edge_statement() {
    let ast = parse("digraph { a -> b }");

    assert_eq!(
      ast.statements,
      vec![Statement::Edge(EdgeStatement {
        attributes: vec![],
        edges: vec![(
          EdgeOperation::Arrow,
          EdgeTarget::NodeId(NodeId {
            id: Id::Ident("b"),
            port: None,
          }),
        )],
        from: EdgeTarget::NodeId(NodeId {
          id: Id::Ident("a"),
          port: None,
        }),
      })],
    );
  }

  #[test]
  fn empty_digraph() {
    let ast = parse("digraph {}");

    assert_eq!(
      ast,
      Graph {
        id: None,
        kind: GraphKind::Digraph,
        statements: vec![],
        strict: false,
      },
    );
  }

  #[test]
  fn empty_graph() {
    let ast = parse("graph {}");

    assert_eq!(
      ast,
      Graph {
        id: None,
        kind: GraphKind::Graph,
        statements: vec![],
        strict: false,
      },
    );
  }

  #[test]
  fn named_graph() {
    let ast = parse("digraph G {}");

    assert_eq!(ast.id, Some(Id::Ident("G")));
  }

  #[test]
  fn node_statement() {
    let ast = parse("digraph { a }");

    assert_eq!(
      ast.statements,
      vec![Statement::Node(NodeStatement {
        attributes: vec![],
        id: NodeId {
          id: Id::Ident("a"),
          port: None,
        },
      })],
    );
  }

  #[test]
  fn node_with_attrs() {
    let ast = parse("digraph { a [color=red, shape=circle] }");

    assert_eq!(
      ast.statements,
      vec![Statement::Node(NodeStatement {
        attributes: vec![
          Attribute {
            key: Id::Ident("color"),
            value: Some(Id::Ident("red")),
          },
          Attribute {
            key: Id::Ident("shape"),
            value: Some(Id::Ident("circle")),
          },
        ],
        id: NodeId {
          id: Id::Ident("a"),
          port: None,
        },
      })],
    );
  }

  fn parse(input: &str) -> Graph<'_> {
    super::parse(input).expect("parsing failed")
  }

  #[test]
  fn port() {
    let ast = parse("digraph { a:p1 -> b:p2 }");

    assert_matches!(
      &ast.statements[..],
      [Statement::Edge(EdgeStatement {
        from: EdgeTarget::NodeId(NodeId {
          port: Some(Port {
            compass: None,
            id: Id::Ident("p1")
          }),
          ..
        }),
        ..
      })],
    );
  }

  #[test]
  fn port_with_compass() {
    let ast = parse("digraph { a:p1:n -> b }");

    assert_matches!(
      &ast.statements[..],
      [Statement::Edge(EdgeStatement {
        from: EdgeTarget::NodeId(NodeId {
          port: Some(Port {
            compass: Some(Id::Ident("n")),
            id: Id::Ident("p1")
          }),
          ..
        }),
        ..
      })],
    );
  }

  #[test]
  fn semicolons() {
    let ast = parse("digraph { a; b; c; }");

    assert_eq!(ast.statements.len(), 3);
  }

  #[test]
  fn strict_graph() {
    let ast = parse("strict digraph {}");

    assert!(ast.strict);
  }

  #[test]
  fn subgraph() {
    let ast = parse("digraph { subgraph cluster_0 { a } }");

    assert_matches!(
      &ast.statements[..],
      [Statement::Subgraph(Subgraph { id: Some(Id::Ident("cluster_0")), statements })] if statements.len() == 1,
    );
  }

  #[test]
  fn undirected_edge() {
    let ast = parse("graph { a -- b }");

    assert_matches!(
      &ast.statements[..],
      [Statement::Edge(EdgeStatement { edges, .. })] if edges[0].0 == EdgeOperation::DashDash,
    );
  }
}
