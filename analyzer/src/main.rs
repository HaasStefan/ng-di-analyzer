
use ng_tree::NgTree;
pub mod ng_tree;

fn main() {
    let tree = NgTree::parse("../angular-workspace/projects/app/src/main.ts");
}
