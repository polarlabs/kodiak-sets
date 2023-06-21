#[cfg(test)]
mod tests {
    use crate::position::Position;
    use crate::tests::sequence::*;
    use crate::Sequence;

    #[test]
    fn test_new_sequence_empty() {
        let seq = setup_seq_empty();

        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_new_sequence_3() {
        let seq = setup_seq_abc();

        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_len() {
        let seq = setup_seq_abc();

        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_position_from() {
        let seq = setup_seq_abc();

        assert_eq!(seq.position_from(1), Position::new(2, 1));
    }

    #[test]
    fn test_insert_into_empty_sequence() {
        let mut seq = setup_seq_empty();
        seq.insert(0, "A".to_string());

        assert_eq!(seq.len(), 1);
        assert_eq!(seq.first(), Some(&"A".to_string()));
        assert_eq!(seq.last(), Some(&"A".to_string()));
    }

    #[test]
    fn test_insert_into_non_empty_sequence_index_gt_len() {
        let mut seq = setup_seq_abc();
        seq.insert(8, "H".to_string());

        assert_eq!(seq.len(), 4);
        assert_eq!(seq.get(3), Some(&"H".to_string()));
        assert_eq!(seq.last(), Some(&"H".to_string()));
    }

    #[test]
    fn test_insert_into_non_empty_sequence_index_0() {
        let mut seq = setup_seq_abc();
        seq.insert(0, "_H".to_string());

        assert_eq!(seq.len(), 4);
        assert_eq!(seq.get(0), Some(&"_H".to_string()));
        assert_eq!(seq.first(), Some(&"_H".to_string()));
    }

    #[test]
    fn test_insert_into_non_empty_sequence_index_1() {
        let mut seq = setup_seq_abc();
        seq.insert(1, "AA".to_string());

        assert_eq!(seq.len(), 4);
        assert_eq!(seq.get(1), Some(&"AA".to_string()));
    }

    #[test]
    fn test_insert_at_pos_into_non_empty_sequence() {
        let mut seq = setup_seq_abc();
        seq.insert_at(Position::new(4, 1), "D".to_string());

        assert_eq!(seq.len(), 4);
        assert_eq!(seq.get(3), Some(&"D".to_string()));
    }

    #[test]
    fn test_insert_at_pos_into_non_empty_sequence_at_existing_middle_pos() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        seq.insert_at(Position::new(2, 1), "BB".to_string());

        assert_eq!(seq.len(), len);
        assert_eq!(seq.get(1), Some(&"BB".to_string()));
    }

    #[test]
    fn test_insert_at_pos_into_non_empty_sequence_at_existing_end_pos() {
        let mut seq = setup_seq_abc();
        seq.insert_at(Position::new(3, 1), "D".to_string());

        assert_eq!(seq.len(), 3);
        assert_eq!(seq.get(2), Some(&"D".to_string()));

        seq.insert_at(Position::new(3, 1), "E".to_string());
        assert_eq!(seq.len(), 3);
        assert_eq!(seq.get(2), Some(&"E".to_string()));
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
    fn test_next() {
        let mut seq = setup_seq_abc();
        let mut n = 0;

        while n <= seq.len() {
            assert_eq!(seq.current_index, n);
            seq.non_iterating_next();
            // todo: compare output
            n += 1;
        }

        assert_eq!(seq.current_index, 0);
    }

    #[test]
    fn test_remove_r_none() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove(len), None);
        assert_eq!(seq.len(), len);
    }

    #[test]
    fn test_remove_r_some() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove(1), Some("B".to_string()));
        assert_eq!(seq.len(), len - 1);
    }

    #[test]
    fn test_remove_at_r_none() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove_at(Position::new((len + 1) as u64, 1)), None);
        assert_eq!(seq.len(), len);
    }

    #[test]
    fn test_remove_at_r_some() {
        let mut seq = setup_seq_abc();
        let len = seq.len();

        assert_eq!(seq.remove_at(Position::new(len as u64, 1)), Some("C".to_string()));
        assert_eq!(seq.len(), len - 1);
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

        assert_eq!(seq[0], "A".to_string());
    }

    #[test]
    fn test_trait_impl_index_mut() {
        let mut seq = setup_seq_abc();
        seq[0] = "_H".to_string();

        assert_eq!(seq[0], "_H".to_string());
    }

    #[test]
    fn test_trait_impl_clone() {
        let seq = setup_seq_abc();
        let seq2 = seq.clone();

        assert_eq!(seq.len(), seq2.len());
        assert_eq!(seq, seq2);
    }
}
