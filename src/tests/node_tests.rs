#[cfg(test)]
mod tests {
    use crate::sequence::{Node, Pos};

    pub fn setup_node_empty() -> Node<String> {
        let node: Node<String> = Node::new_empty(Pos::default());
        node
    }

    #[test]
    fn test_new_node_empty() {
        let node = setup_node_empty();

        assert_eq!(node.element_as_ref(), None);
    }

    #[test]
    fn test_num() {
        let node = Node::new(Pos::default(), "A".to_string());

        assert_eq!(node.num(), 1);
    }

    #[test]
    fn test_denom() {
        let node = Node::new(Pos::default(), "A".to_string());

        assert_eq!(node.denom(), 1);
    }

    #[test]
    fn test_element_as_ref_some_t() {
        let node = Node::new(Pos::default(), "A".to_string());

        assert_eq!(node.element_as_ref(), Some(&"A".to_string()));
    }

    #[test]
    fn test_element_as_ref_none() {
        let node: Node<String> = Node::new_empty(Pos::default());

        assert_eq!(node.element_as_ref(), None);
    }

    #[test]
    fn test_element_as_mut_some_t() {
        let mut node = Node::new(Pos::default(), "A".to_string());

        assert_eq!(node.element_as_mut(), Some(&mut "A".to_string()));
    }

    #[test]
    fn test_element_as_mut_none() {
        let mut node: Node<String> = Node::new_empty(Pos::default());

        assert_eq!(node.element_as_mut(), None);
    }
}
