pub mod graph {
    use std::collections::HashMap;

    use graph_items::node::Node;
    use graph_items::edge::Edge;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes.iter() {
                self.nodes.push(node.clone())
            }
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges.iter() {
                self.edges.push(edge.clone())
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (name, value) in attrs.iter() {
                self.attrs.insert(name.to_string(), value.to_string());
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().filter(|node| node.name == name).nth(0).map(|node| node.clone())
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node { name: name.to_string(), attrs: HashMap::new() }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (name, value) in attrs.iter() {
                        self.attrs.insert(name.to_string(), value.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|value| value.as_str())
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge{ from: from.to_string(), to: to.to_string(), attrs: HashMap::new() }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (name, value) in attrs.iter() {
                        self.attrs.insert(name.to_string(), value.to_string());
                    }
                    self
                }
            }
        }
    }
}
