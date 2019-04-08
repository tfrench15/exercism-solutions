pub mod graph {
    use std::collections::HashMap;
        
    #[derive(Debug, Clone, Default, PartialEq)]
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

        pub fn get_node(self, node: &str) -> Result<graph_items::node::Node, ()> {
            for elem in self.nodes {
                if elem.id == node {
                    return Ok(graph_items::node::Node {
                        id: elem.id,
                        attrs: self.attrs
                    })
                }
            }
            panic!("node must be stored")
        }

        pub fn get_attr(&self, key: &str) -> Option<&str> {
            match self.attrs.get(key) {
                Some(value) => { Some(&value) },
                None => { None }
            }
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub id: String,
                pub attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(s: &str) -> Node {
                    Node { 
                        id: s.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Node {
                    let mut map: HashMap<String, String> = self.attrs;
                    map.insert(attrs[0].0.to_string(), attrs[0].1.to_string());

                    Node { 
                        id: self.id,
                        attrs: map
                    }
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    match self.attrs.get(attr) {
                        Some(v) => { return Some(&v) },
                        None => { None }
                    }
                }

                pub fn expect(&self, msg: &str) -> String {
                    msg.to_string()
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                head: String,
                tail: String,
                attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Edge {
                    Edge {
                        head: a.to_string(),
                        tail: b.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Edge {
                    let mut map: HashMap<String, String> = self.attrs;
                    map.insert(attrs[0].0.to_string(), attrs[0].1.to_string());

                    Edge {
                        head: self.head,
                        tail: self.tail,
                        attrs: map
                    }
                }
            }
        }
    }
}
