#[cfg(test)]
mod tests {
    use crate::sequence::{Min, Pos};

    #[test]
    fn test_new_position_5_1() {
        let pos = Pos::new(5, 1);

        assert_eq!(pos, Pos { num: 5, denom: 1 });
    }

    #[test]
    fn test_new_position_1_1000() {
        let pos = Pos::new(1, 1000);

        assert_eq!(pos, Pos { num: 1, denom: 1000 });
    }

    #[test]
    fn test_new_position_1000_1() {
        let pos = Pos::new(1000, 1);

        assert_eq!(pos, Pos { num: 1000, denom: 1 });
    }

    #[test]
    fn test_new_position_1_0() {
        // denom < DENOM_MIN
        let pos = Pos::new(1, 0);

        assert_eq!(pos, Pos { num: 1, denom: 1 });
    }

    #[test]
    fn test_new_position_1_max_with_overflow() {
        let pos = Pos::new(1, u64::MAX.wrapping_add(1));

        assert_eq!(pos, Pos { num: 1, denom: 1 });
    }

    #[test]
    fn test_new_position_max_with_overflow_1() {
        let pos = Pos::new(u64::MAX.wrapping_add(1), 1);

        assert_eq!(pos, Pos { num: 0, denom: 1 });
    }

    #[test]
    fn test_n1d0() {
        let pos = Pos::n1d0();

        assert_eq!(pos, Pos { num: 1, denom: 0 });
    }

    #[test]
    fn test_mid_1_1_2_1() {
        let lhs = Pos::new(1, 1);
        let rhs = Pos::new(2, 1);

        assert_eq!(Pos::mid(lhs, rhs), Pos { num: 3, denom: 2 });
        assert_eq!(Pos::mid(rhs, lhs), Pos { num: 3, denom: 2 });
    }

    #[test]
    fn test_mid_min_1_1() {
        let pos = Pos::new(1, 1);

        let mid = Pos::mid(Pos::MIN, pos);
        assert_eq!(mid, Pos { num: 1, denom: 2 });
    }

    #[test]
    fn test_add_n1d1_n1d0() {
        let pos = Pos::new(1, 1);

        let add = pos + Pos::n1d0();
        assert_eq!(add, Pos { num: 2, denom: 1 });
    }

    #[test]
    fn test_add_n27d5_n110d1() {
        let pos1 = Pos::new(27, 5);
        let pos2 = Pos::new(110, 1);

        assert_eq!(
            pos1 + pos2,
            Pos {
                num: 27 + 110,
                denom: 5 + 1
            }
        );
    }

    #[test]
    fn test_add_assign_n1d1_n1d0() {
        let mut pos = Pos::new(1, 1);
        pos += Pos::n1d0();

        assert_eq!(pos, Pos { num: 2, denom: 1 });
    }

    #[test]
    fn test_add_assign_n27d5_n110d1() {
        let mut pos = Pos::new(27, 5);
        pos += Pos::new(110, 1);

        assert_eq!(
            pos,
            Pos {
                num: 27 + 110,
                denom: 5 + 1
            }
        );
    }

    #[test]
    fn test_partial_eq_n5d1_n5d1() {
        let pos1 = Pos::new(5, 1);
        let pos2 = Pos::new(5, 1);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_eq_n1d1_n2d1() {
        let pos1 = Pos::new(1, 1);
        let pos2 = Pos::new(2, 1);

        assert_eq!(pos1 == pos2, false);
        assert_eq!(pos1 != pos2, true);
    }

    #[test]
    fn test_partial_eq_n4d2_n2d1() {
        let pos1 = Pos::new(4, 2);
        let pos2 = Pos::new(2, 1);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_eq_n2d1_n4d2() {
        let pos1 = Pos::new(2, 1);
        let pos2 = Pos::new(4, 2);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_eq_n8d2_n16d4() {
        let pos1 = Pos::new(8, 2);
        let pos2 = Pos::new(16, 4);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_ord_n1d1_n2d1() {
        let pos1 = Pos::new(1, 1);
        let pos2 = Pos::new(2, 1);

        assert_eq!(pos1 >= pos2, false);
        assert_eq!(pos1 <= pos2, true);
    }

    #[test]
    fn test_default() {
        let pos = Pos::default();

        assert_eq!(pos, Pos::new(1, 1));
    }
}
