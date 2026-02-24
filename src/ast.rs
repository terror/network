#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Attribute<'src> {
  pub(crate) key: Id<'src>,
  pub(crate) value: Option<Id<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AttributeStatement<'src> {
  pub(crate) attributes: Vec<Attribute<'src>>,
  pub(crate) target: AttributeTarget,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AttributeTarget {
  Edge,
  Graph,
  Node,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EdgeOperation {
  Arrow,
  DashDash,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EdgeStatement<'src> {
  pub(crate) attributes: Vec<Attribute<'src>>,
  pub(crate) edges: Vec<(EdgeOperation, EdgeTarget<'src>)>,
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
  pub(crate) statements: Vec<Statement<'src>>,
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
pub(crate) struct NodeStatement<'src> {
  pub(crate) attributes: Vec<Attribute<'src>>,
  pub(crate) id: NodeId<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Port<'src> {
  pub(crate) compass: Option<Id<'src>>,
  pub(crate) id: Id<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Statement<'src> {
  Assign(Id<'src>, Id<'src>),
  Attr(AttributeStatement<'src>),
  Edge(EdgeStatement<'src>),
  Node(NodeStatement<'src>),
  Subgraph(Subgraph<'src>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Subgraph<'src> {
  pub(crate) id: Option<Id<'src>>,
  pub(crate) statements: Vec<Statement<'src>>,
}
