use nodes;
use std::collections::HashMap;

pub mod decorators;
pub mod nodes;

/**
 * 
 * Program is the main entry point for the Angular program.
 * Contains a flat hash table for all the nodes without for toring 
 * It uses root node to reflect the hierarchy of the nodes.
 */
pub struct Program {
    pub main_ts_path: String,
    pub root_node: nodes::RootNode,
    components: HashMap<String, nodes::NodeType::Component>,
    directives: HashMap<String, nodes::NodeType::Directive>,
    pipes: HashMap<String, nodes::NodeType::Pipe>,
    services: HashMap<String, nodes::NodeType::Service>,
    ng_modules: HashMap<String, nodes::NodeType::NgModule>,
    routes: HashMap<String, nodes::NodeType::Routes>,
    injection_tokens: HashMap<String, nodes::NodeType::InjectionToken>
}
