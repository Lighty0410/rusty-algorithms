use schemars::{schema_for, JsonSchema};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, JsonSchema)]
struct Node {
    name: String,
    children: Option<Vec<Node>>,
}

impl Node {
    pub fn depth_first_search(&self, mut names: Vec<String>) -> Vec<String> {
        names.push(self.name.clone());

        for child in self.children.iter().flatten() {
            names = child.depth_first_search(names);
        }

        names
    }
}

mod tests {
    use crate::depth_first_search::Node;
    use schemars::schema_for;

    #[test]
    fn test_depth_first_search() {
        let schema = schema_for!(Node);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
