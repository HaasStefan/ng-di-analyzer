
pub enum NodeType {
    Component(NodeMetadata),
    Directive(NodeMetadata),
    Pipe(NodeMetadata),
    Service(NodeMetadata),
    NgModule(NodeMetadata),
    Routes(NodeMetadata),
    InjectionToken(NodeMetadata),
}

pub struct NodeMetadata {
  pub identifier: String,
  pub children: Vec<NodeType>,
}

pub struct Program {
    pub nodes: Vec<NodeType>,
}

impl Program {
    pub fn new() -> Self {
        Program { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: NodeType) {
        self.nodes.push(node);
    }

    pub fn get_nodes(&self) -> &Vec<NodeType> {
        &self.nodes
    }
}

impl NodeMetadata {
    pub fn new(identifier: String) -> Self {
        NodeMetadata {
            identifier,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: NodeType) {
        self.children.push(child);
    }

    pub fn get_children(&self) -> &Vec<NodeType> {
        &self.children
    }
}