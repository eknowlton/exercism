pub mod graph {
    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node<'a> {
                message: &'a str,
                attrs: Vec<super::attr::Attr>,
            }

            impl<'a> Node<'a> {
                pub fn new(message: &'a str) -> Self {
                    Node {
                        message,
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(&'a mut self, attrs: &'a Vec<super::attr::Attr>) -> &'a mut Self {
                    self.attrs.extend(attrs.to_vec());
                    self
                }
            }

        }

        pub mod edge {
            #[derive(Clone)]
            pub struct Edge<'a> {
                to: &'a str,
                from: &'a str,
            }

            impl<'a> Edge<'a> {
                pub fn new(to: &'a str, from: &'a str) -> Self {
                    Edge { to, from }
                }
            }

        }
        pub mod attr {

            #[derive(Clone, PartialEq, Debug)]
            pub struct Attr;

        }

    }

    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
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

        pub fn with_nodes(&'a mut self, nodes: &'a Vec<graph_items::node::Node>) -> &'a mut Self {
            self.nodes.extend(nodes.to_vec());
            self
        }
    }
}
