#[derive(Clone, Debug, PartialEq)]
pub struct Attribute<'src> {
  pub key: Id<'src>,
  pub value: Option<Id<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeStatement<'src> {
  pub attributes: Vec<Attribute<'src>>,
  pub target: AttributeTarget,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeTarget {
  Edge,
  Graph,
  Node,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeOperation {
  Arrow,
  DashDash,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeStatement<'src> {
  pub attributes: Vec<Attribute<'src>>,
  pub edges: Vec<(EdgeOperation, EdgeTarget<'src>)>,
  pub from: EdgeTarget<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeTarget<'src> {
  NodeId(NodeId<'src>),
  Subgraph(Subgraph<'src>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Graph<'src> {
  pub id: Option<Id<'src>>,
  pub kind: GraphKind,
  pub statements: Vec<Statement<'src>>,
  pub strict: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GraphKind {
  Digraph,
  Graph,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Id<'src> {
  HtmlString(&'src str),
  Ident(&'src str),
  Number(&'src str),
  String(&'src str),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeId<'src> {
  pub id: Id<'src>,
  pub port: Option<Port<'src>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeStatement<'src> {
  pub attributes: Vec<Attribute<'src>>,
  pub id: NodeId<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Port<'src> {
  pub compass: Option<Id<'src>>,
  pub id: Id<'src>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'src> {
  Assign(Id<'src>, Id<'src>),
  Attr(AttributeStatement<'src>),
  Edge(EdgeStatement<'src>),
  Node(NodeStatement<'src>),
  Subgraph(Subgraph<'src>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subgraph<'src> {
  pub id: Option<Id<'src>>,
  pub statements: Vec<Statement<'src>>,
}
