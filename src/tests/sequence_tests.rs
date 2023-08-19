#[cfg(test)]
mod tests {
    use crate::sequence::{Node, Pos, Sequence};

    pub fn setup_seq_empty() -> Sequence<String> {
        let seq: Sequence<String> = Sequence::new();
        seq
    }

    pub fn setup_seq_abc() -> Sequence<String> {
        let mut seq: Sequence<String> = Sequence::new();

        seq.push("A".to_string());
        seq.push("B".to_string());
        seq.push("C".to_string());

        seq
    }

    pub fn setup_seq_123() -> Sequence<i32> {
        let mut seq: Sequence<i32> = Sequence::new();

        seq.push(1);
        seq.push(2);
        seq.push(3);

        seq
    }

    #[test]
    fn test_new_seq_empty() {
        let seq = setup_seq_empty();

        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_new_seq_3() {
        let seq = setup_seq_abc();

        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_with_capacity() {
        let capacity = 4;
        let seq: Sequence<String> = Sequence::with_capacity(capacity);

        assert_eq!(seq.len(), 0);
        assert_eq!(seq.capacity(), capacity)
    }

    #[test]
    fn test_first_none_seq_empty() {
        let seq = setup_seq_empty();

        assert_eq!(seq.first(), None);
    }

    #[test]
    fn test_first_some() {
        let seq = setup_seq_abc();

        assert_eq!(seq.first(), Some(&"A".to_string()));
    }

    #[test]
    fn test_first_some_after_first_element_has_been_removed() {
        let mut seq = setup_seq_abc();
        let _node = seq.remove(0);

        assert_eq!(seq.first(), Some(&"B".to_string()));
    }

    #[test]
    fn test_first_none_after_all_nodes_have_been_removed() {
        let mut seq = setup_seq_abc();
        let _node = seq.remove(0);
        let _node = seq.remove(0);
        let _node = seq.remove(0);

        assert_eq!(seq.first(), None);
    }

    #[test]
    fn test_last_none() {
        let seq = setup_seq_empty();

        assert_eq!(seq.last(), None);
    }

    #[test]
    fn test_last_none_after_all_nodes_have_been_removed() {
        let mut seq = setup_seq_abc();
        let _node = seq.remove(0);
        let _node = seq.remove(0);
        let _node = seq.remove(0);

        assert_eq!(seq.last(), None);
    }

    #[test]
    fn test_last_some() {
        let seq = setup_seq_abc();

        assert_eq!(seq.last(), Some(&"C".to_string()));
    }

    #[test]
    fn test_last_some_after_last_node_has_been_removed() {
        let mut seq = setup_seq_abc();
        let _node = seq.remove(2);

        assert_eq!(seq.last(), Some(&"B".to_string()));
    }

    #[test]
    fn test_get_some() {
        let seq = setup_seq_abc();

        assert_eq!(seq.get(1), Some(&"B".to_string()));
    }

    #[test]
    fn test_get_none_node_empty() {
        let mut seq = setup_seq_abc();
        seq.remove(1);

        assert_eq!(seq.get(1), Some(&"C".to_string()));
    }

    #[test]
    fn test_get_none_index_out_of_bounds() {
        let seq = setup_seq_abc();

        assert_eq!(seq.get(3), None);
    }

    #[test]
    fn test_get_mut_some() {
        let mut seq = setup_seq_abc();

        assert_eq!(seq.get_mut(1), Some(&mut "B".to_string()));
    }

    #[test]
    fn test_get_mut_none_node_empty() {
        let mut seq = setup_seq_abc();
        seq.remove(1);

        assert_eq!(seq.get_mut(1), Some(&mut "C".to_string()));
    }

    #[test]
    fn test_get_mut_none_index_out_of_bounds() {
        let mut seq = setup_seq_abc();

        assert_eq!(seq.get_mut(3), None);
    }

    #[test]
    fn test_len_seq_new() {
        let seq = setup_seq_abc();
        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_len_seq_insert_element() {
        let mut seq = setup_seq_abc();
        seq.insert(10, "A".to_string());
        assert_eq!(seq.len(), 4);
    }

    #[test]
    fn test_len_seq_insert_element_at() {
        let mut seq = setup_seq_abc();
        seq.insert_at(Pos::default(), "A".to_string());
        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_len_seq_remove_element() {
        let mut seq = setup_seq_abc();

        seq.remove(1);
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_len_seq_remove_element_at() {
        let mut seq = setup_seq_abc();
        seq.remove_at(Pos::default());
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_len_seq_remove_element_insert_element() {
        let mut seq = setup_seq_abc();
        seq.remove(1);
        assert_eq!(seq.len(), 2);

        seq.insert(1, "A".to_string());
        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_len_seq_push_element() {
        let mut seq = setup_seq_abc();
        seq.push("A".to_string());
        assert_eq!(seq.len(), 4);
    }

    #[test]
    fn test_insert_index_eq_len_empty_seq() {
        let mut seq = setup_seq_empty();
        let len = seq.len();

        seq.insert(0, "A".to_string());

        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.first(), Some(&"A".to_string()));
        assert_eq!(seq.last(), Some(&"A".to_string()));
    }

    #[test]
    fn test_insert_index_gt_len_empty_seq() {
        let mut seq = setup_seq_empty();
        let len = seq.len();

        seq.insert(100, "A".to_string());

        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.first(), Some(&"A".to_string()));
        assert_eq!(seq.last(), Some(&"A".to_string()));
    }

    #[test]
    fn test_insert_index_eq_len_non_empty_seq() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert(len, "H".to_string());

        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.get(len), Some(&"H".to_string()));
        assert_eq!(seq.last(), Some(&"H".to_string()));
    }

    #[test]
    fn test_insert_index_gt_len_non_empty_seq() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert(len + 100, "H".to_string());

        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.get(len), Some(&"H".to_string()));
        assert_eq!(seq.last(), Some(&"H".to_string()));
    }

    #[test]
    fn test_insert_index_lt_len_node_is_some_index_eq_0() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert(0, "H".to_string());
        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.get(0), Some(&"H".to_string()));
        assert_eq!(seq.first(), Some(&"H".to_string()));
    }

    #[test]
    fn test_insert_index_lt_len_node_is_some_index_ne_0() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert(1, "H".to_string());

        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.get(1), Some(&"H".to_string()));
    }

    #[test]
    fn test_insert_index_lt_len_node_is_none_index_ne_0() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.remove(1);
        assert_eq!(seq.len(), len - 1);

        seq.insert(1, "H".to_string());
        assert_eq!(seq.len(), len);
        assert_eq!(seq.get(1), Some(&"H".to_string()));
    }

    #[test]
    fn test_insert_at_empty_seq() {
        let mut seq = setup_seq_empty();
        let len = seq.len();

        seq.insert_at(Pos::new(4, 1), "D".to_string());
        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.first(), Some(&"D".to_string()));
        assert_eq!(seq.last(), Some(&"D".to_string()));
    }

    #[test]
    fn test_insert_at_non_existing_pos_to_end_of_seq() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert_at(Pos::new(4, 1), "D".to_string());
        assert_eq!(seq.len(), len + 1);
        assert_eq!(seq.last(), Some(&"D".to_string()));
    }

    #[test]
    fn test_insert_at_at_existing_but_vacant_pos() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        let _node = seq.remove(1);

        seq.insert_at(Pos::new(2, 1), "BB".to_string());
        assert_eq!(seq.len(), len);
        assert_eq!(seq.get(1), Some(&"BB".to_string()));
    }

    #[test]
    fn test_insert_at_at_existing_pos_to_mid_of_seq() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert_at(Pos::new(2, 1), "BB".to_string());
        assert_eq!(seq.len(), len);
        assert_eq!(seq.get(1), Some(&"BB".to_string()));
    }

    #[test]
    fn test_insert_at_at_existing_pos_to_end_of_seq() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert_at(Pos::new(3, 1), "D".to_string());
        assert_eq!(seq.len(), len);
        assert_eq!(seq.get(2), Some(&"D".to_string()));

        seq.insert_at(Pos::new(3, 1), "E".to_string());
        assert_eq!(seq.len(), len);
        assert_eq!(seq.get(2), Some(&"E".to_string()));
    }

    #[test]
    fn test_position_from_index_eq_len() {
        let seq = setup_seq_abc();

        assert_eq!(seq.position_from(3), None);
    }

    #[test]
    fn test_position_from_index_gt_len() {
        let seq = setup_seq_abc();

        assert_eq!(seq.position_from(5), None);
    }

    #[test]
    fn test_position_from_index_lt_len() {
        let seq = setup_seq_abc();

        assert_eq!(seq.position_from(1), Some(Pos::new(2, 1)));
    }

    #[test]
    fn test_pos_from_index_eq_len() {
        let seq = setup_seq_abc();

        assert_eq!(seq.pos_from(3), None);
    }

    #[test]
    fn test_pos_from_index_gt_len() {
        let seq = setup_seq_abc();

        assert_eq!(seq.pos_from(5), None);
    }

    #[test]
    fn test_pos_from_index_lt_len() {
        let seq = setup_seq_abc();

        assert_eq!(seq.pos_from(1), Some((2, 1)));
    }

    #[test]
    fn test_is_empty_true() {
        let seq = setup_seq_empty();

        assert_eq!(seq.is_empty(), true);
    }

    #[test]
    fn test_is_empty_false() {
        let seq = setup_seq_abc();

        assert_eq!(seq.is_empty(), false);
    }

    #[test]
    fn test_into_iter_next_seq_abc() {
        let seq = setup_seq_abc();

        for node in &seq {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_into_iter_next_seq_123() {
        let seq = setup_seq_123();

        for node in &seq {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_into_iter_owned_seq_abc() {
        let seq = setup_seq_abc();

        for node in seq {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_vec() {
        let mut seq = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut i = 0;

        while i < seq.len() {
            seq.push("None".to_string());
            let node = seq.swap_remove(i);
            println!("{:?}", node);
            i += 1;
        }
    }

    #[test]
    fn test_into_iter_owned_seq_123() {
        let mut seq = setup_seq_123();
        seq[1] = Node::new_empty(Pos::default());

        println!("{}", seq.len());

        for node in seq {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_into_iter_mut_next_seq_123() {
        let mut seq = setup_seq_abc();

        for node in &mut seq {
            node.set("D".to_string());
        }

        println!("{:?}", seq);
    }

    #[test]
    fn test_remove_index_eq_len() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove(3), None);
        assert_eq!(seq.len(), len);
    }

    #[test]
    fn test_remove_index_gt_len() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove(5), None);
        assert_eq!(seq.len(), len);
    }

    #[test]
    fn test_remove_index_two_times() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        let _node = seq.remove(1);

        assert_eq!(seq.remove(1), Some("C".to_string()));
        assert_eq!(seq.len(), len - 2);
    }

    #[test]
    fn test_remove_element_some() {
        // Remove from the front of sequence
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove(0), Some("A".to_string()));
        assert_eq!(seq.len(), len - 1);

        // Remove from the middle of sequence
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove(1), Some("B".to_string()));
        assert_eq!(seq.len(), len - 1);

        // Remove from the rear of sequence
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove(2), Some("C".to_string()));
        assert_eq!(seq.len(), len - 1);
    }

    #[test]
    fn test_remove_element_from_a_vacant_pos_followed_by_only_vacant_pos() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.push("D".to_string());
        seq.push("E".to_string());
        let _node = seq.remove(4);
        let _node = seq.remove(3);
        let _node = seq.remove(2);

        assert_eq!(seq.remove(2), None);
        assert_eq!(seq.len(), len - 1);
    }

    #[test]
    fn test_remove_at_none() {
        // Remove from empty sequence
        let mut seq = setup_seq_empty();
        let len = seq.len();

        assert_eq!(seq.remove_at(Pos::new(1, 1)), None);
        assert_eq!(seq.len(), len);

        // Remove an non-existing node
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove_at(Pos::new((len + 1) as u64, 1)), None);
        assert_eq!(seq.len(), len);
    }

    #[test]
    fn test_remove_at_some() {
        // Remove from the front of sequence
        let mut seq = setup_seq_abc();
        assert_eq!(seq.remove_at(Pos::new(1, 1)), Some("A".to_string()));
        assert_eq!(seq.len(), 2);

        // Remove from the middle of sequence
        let mut seq = setup_seq_abc();
        assert_eq!(seq.remove_at(Pos::new(2, 1)), Some("B".to_string()));
        assert_eq!(seq.len(), 2);

        // Remove from the rear of sequence
        let mut seq = setup_seq_abc();
        let len = seq.len();
        assert_eq!(seq.remove_at(Pos::new(len as u64, 1)), Some("C".to_string()));
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_trait_impl_default() {
        let seq1: Sequence<String> = Sequence::default();
        let seq2: Sequence<String> = Sequence::new();

        assert_eq!(seq1, seq2);
    }

    #[test]
    fn test_trait_impl_index() {
        let seq = setup_seq_abc();

        assert_eq!(seq[0].element_as_ref(), Some(&"A".to_string()));
    }

    #[test]
    fn test_pos_from() {
        let seq = setup_seq_abc();

        assert_eq!(seq.pos_from(0), Some((1, 1)));
    }

    #[test]
    fn test_trait_impl_index_mut() {
        let mut seq = setup_seq_abc();
        seq[0].set("H".to_string());

        assert_eq!(seq[0].element_as_ref(), Some(&"H".to_string()));
    }

    #[test]
    fn test_trait_impl_clone() {
        let seq = setup_seq_abc();
        let seq2 = seq.clone();

        assert_eq!(seq.len(), seq2.len());
        assert_eq!(seq, seq2);
    }

    #[test]
    fn test_trait_impl_clone_incl_element_none() {
        let mut seq = setup_seq_abc();
        seq.remove(1);
        let seq2 = seq.clone();

        assert_eq!(seq.len(), seq2.len());
        assert_eq!(seq, seq2);
    }

    #[test]
    fn test_trait_impl_into_iterator() {
        let mut seq = setup_seq_abc();
        seq.remove(1);

        for node in seq {
            assert_eq!(node.element.is_some(), true);
        }
    }
}
