#[cfg(test)]
mod tests {
    use crate::sequence::{Node, Pos};

    // Helpers to setup test
    pub fn setup_node() -> Node<String> {
        Node::new(Pos::default(), "A".to_string())
    }

    pub fn setup_node_empty() -> Node<String> {
        Node::new_empty(Pos::default())
    }

    #[test]
    fn test_new() {
        let node = setup_node();

        assert_eq!(node.pos(), (node.num(), node.denom()));
        assert_eq!(node.element(), Some("A".to_string()));
    }

    #[test]
    fn test_new_empty() {
        let node = setup_node_empty();

        assert_eq!(node.pos(), (node.num(), node.denom()));
        assert_eq!(node.element_as_ref(), None);
    }

    #[test]
    fn test_position() {
        let node = setup_node();

        assert_eq!(node.position(), Pos::default());
    }

    #[test]
    fn test_pos() {
        let node = setup_node();

        assert_eq!(node.pos(), (node.num(), node.denom()));
    }

    #[test]
    fn test_num() {
        let node = setup_node();

        assert_eq!(node.num(), Pos::default().num);
    }

    #[test]
    fn test_denom() {
        let node = setup_node();

        assert_eq!(node.denom(), Pos::default().denom);
    }

    #[test]
    fn test_element_as_ref_some() {
        let node = setup_node();

        assert_eq!(node.element_as_ref(), Some(&"A".to_string()));
    }

    #[test]
    fn test_element_as_ref_none() {
        let node = setup_node_empty();

        assert_eq!(node.element_as_ref(), None);
    }

    #[test]
    fn test_element_as_mut_some() {
        let mut node = setup_node();

        assert_eq!(node.element_as_mut(), Some(&mut "A".to_string()));
    }

    #[test]
    fn test_element_as_mut_none() {
        let mut node = setup_node_empty();

        assert_eq!(node.element_as_mut(), None);
    }

    #[test]
    fn test_is_none() {
        let node = setup_node_empty();

        assert_eq!(node.is_none(), true);
        assert_eq!(node.is_some(), false);
    }

    #[test]
    fn test_is_some() {
        let node = setup_node();

        assert_eq!(node.is_none(), false);
        assert_eq!(node.is_some(), true);
    }

    #[test]
    fn test_set_node() {
        let mut node = setup_node();

        assert_eq!(node.element_as_ref(), Some(&"A".to_string()));

        node.set("B".to_string());

        assert_eq!(node.element_as_ref(), Some(&"B".to_string()));
    }

    #[test]
    fn test_set_node_empty() {
        let mut node = setup_node_empty();

        assert_eq!(node.element_as_ref(), None);

        node.set("B".to_string());

        assert_eq!(node.element_as_ref(), Some(&"B".to_string()));
    }
}
