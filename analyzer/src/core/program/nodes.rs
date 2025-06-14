
pub enum NodeType {
    Component(Node),
    Directive(Node),
    Pipe(Node),
    Service(Node),
    NgModule(Node),
    Routes(Node),
    InjectionToken(Node),
}

pub struct RootNode {
    pub children: Vec<NodeDependency>
}

pub struct NodeDependency {
  pub identifier: String,
  pub children: Vec<NodeDependency>,
}

pub struct Node {
  pub injections: Vec<String>, // TODO: define Injection type
  pub file_path: String,
  pub providers: Option<Vec<String>>, // TODO: define Provider type
  pub viewProviders: Option<Vec<String>>,  
}

impl NodeDependency {
    pub fn new(identifier: String) -> Self {
        NodeDependency {
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