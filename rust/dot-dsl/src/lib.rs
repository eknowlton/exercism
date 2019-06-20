pub mod graph {
    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node<'a>(&'a str);

            impl<'a> Node<'a> {
                pub fn new(content: &'a str) -> Self {
                    Node(content)
                }
            }

        }

        pub mod edge {
            #[derive(Clone)]
            pub struct Edge;

        }
        pub mod attr {

            #[derive(Clone)]
            pub struct Attr;

        }

    }

    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Vec<graph_items::attr::Attr>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: vec![],
            }
        }

        pub fn with_nodes(&self, nodes: &'a Vec<graph_items::node::Node>) -> Self {
            Graph {
                attrs: self.attrs.to_vec(),
                edges: self.edges.to_vec(),
                nodes: [&self.nodes[..], &nodes[..]].concat(),
            }
        }
    }
}
