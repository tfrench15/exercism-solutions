pub mod graph {
    use std::collections::HashMap;
        
    #[derive(Debug, Clone, Default)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes(self, n: &Vec<graph_items::node::Node>) -> Graph {
            Graph {
                nodes: n.to_vec(),
                edges: self.edges,
                attrs: self.attrs
            }
        }

        pub fn with_edges(self, e: &Vec<graph_items::edge::Edge>) -> Graph {
            Graph { 
                nodes: self.nodes,
                edges: e.to_vec(),
                attrs: self.attrs
            }
        }

        pub fn with_attrs(self, a: &[(&str, &str)]) -> Graph {
            let mut map: HashMap<String, String> = self.attrs;
            map.insert(a[0].0.to_string(), a[0].1.to_string());

            Graph {
                nodes: self.nodes,
                edges: self.edges,
                attrs: map
            }
        }

        pub fn get_node(&self, node: &graph_items::node::Node) -> graph_items::node::Node {
            for elem in self.nodes {
                if elem == node {
                    return elem
                }
            }
        }

        pub fn get_attr(&self, key: &str) -> String {
            match self.attrs.get(key) {
                Some(value) => { return value.to_string() },
                None => { return String::new() }
            }
        }
    }

    pub mod graph_items {
        pub mod node {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                id: String
            }

            impl Node {
                pub fn new(s: &str) -> Node {
                    Node { id: s.to_string() }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Node {
                    Node { id: self.id }
                }
            }
        }

        pub mod edge {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                head: String,
                tail: String
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Edge {
                    Edge {
                        head: a.to_string(),
                        tail: b.to_string()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Edge {
                    Edge {
                        head: self.head,
                        tail: self.tail
                    }
                }
            }
        }
    }
}
