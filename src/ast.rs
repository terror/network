#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Attr<'src> {
  pub(crate) key: Id<'src>,
  pub(crate) value: Option<Id<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AttrStmt<'src> {
  pub(crate) attrs: Vec<Attr<'src>>,
  pub(crate) target: AttrTarget,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AttrTarget {
  Edge,
  Graph,
  Node,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EdgeOp {
  Arrow,
  DashDash,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EdgeStmt<'src> {
  pub(crate) attrs: Vec<Attr<'src>>,
  pub(crate) edges: Vec<(EdgeOp, EdgeTarget<'src>)>,
  pub(crate) from: EdgeTarget<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EdgeTarget<'src> {
  NodeId(NodeId<'src>),
  Subgraph(Subgraph<'src>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Graph<'src> {
  pub(crate) id: Option<Id<'src>>,
  pub(crate) kind: GraphKind,
  pub(crate) stmts: Vec<Stmt<'src>>,
  pub(crate) strict: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GraphKind {
  Digraph,
  Graph,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Id<'src> {
  HtmlString(&'src str),
  Ident(&'src str),
  Number(&'src str),
  String(&'src str),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NodeId<'src> {
  pub(crate) id: Id<'src>,
  pub(crate) port: Option<Port<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NodeStmt<'src> {
  pub(crate) attrs: Vec<Attr<'src>>,
  pub(crate) id: NodeId<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Port<'src> {
  pub(crate) compass: Option<Id<'src>>,
  pub(crate) id: Id<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Stmt<'src> {
  Assign(Id<'src>, Id<'src>),
  Attr(AttrStmt<'src>),
  Edge(EdgeStmt<'src>),
  Node(NodeStmt<'src>),
  Subgraph(Subgraph<'src>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Subgraph<'src> {
  pub(crate) id: Option<Id<'src>>,
  pub(crate) stmts: Vec<Stmt<'src>>,
}
