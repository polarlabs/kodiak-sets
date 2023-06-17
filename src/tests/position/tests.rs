#[cfg(test)]
mod tests {
    use crate::position::{Min, Position};

    #[test]
    fn test_new_position_5_1() {
        let pos = Position::new(5, 1);

        assert_eq!(
            pos,
            Position {
                numerator: 5,
                denominator: 1
            }
        );
    }

    #[test]
    fn test_new_position_1_1000() {
        let pos = Position::new(1, 1000);

        assert_eq!(
            pos,
            Position {
                numerator: 1,
                denominator: 1000
            }
        );
    }

    #[test]
    fn test_new_position_1000_1() {
        let pos = Position::new(1000, 1);

        assert_eq!(
            pos,
            Position {
                numerator: 1000,
                denominator: 1
            }
        );
    }

    #[test]
    fn test_new_position_1_0() {
        let pos = Position::new(1, 0);

        assert_eq!(
            pos,
            Position {
                numerator: 1,
                denominator: 1
            }
        );
    }

    #[test]
    fn test_new_position_1_max_with_overflow() {
        let pos = Position::new(1, u64::MAX.wrapping_add(1));

        assert_eq!(
            pos,
            Position {
                numerator: 1,
                denominator: 1
            }
        );
    }

    #[test]
    fn test_n1d0() {
        let pos = Position::n1d0();

        assert_eq!(
            pos,
            Position {
                numerator: 1,
                denominator: 0
            }
        );
    }

    #[test]
    fn test_mid_min_1_1() {
        let pos = Position::new(1, 1);

        assert_eq!(
            Position::mid(Position::MIN, pos),
            Position {
                numerator: 1,
                denominator: 2
            }
        );
    }

    #[test]
    fn test_mid_1_1_2_1() {
        let lhs = Position::new(1, 1);
        let rhs = Position::new(2, 1);

        assert_eq!(
            Position::mid(lhs, rhs),
            Position {
                numerator: 3,
                denominator: 2
            }
        );
        assert_eq!(
            Position::mid(rhs, lhs),
            Position {
                numerator: 3,
                denominator: 2
            }
        );
    }

    #[test]
    fn test_add_assign_n1d1_n1d0() {
        let mut pos = Position::new(1, 1);
        pos += Position::n1d0();

        assert_eq!(
            pos,
            Position {
                numerator: 2,
                denominator: 1
            }
        );
    }

    #[test]
    fn test_partial_eq_n1d1_n2d1() {
        let pos1 = Position::new(1, 1);
        let pos2 = Position::new(2, 1);

        assert_eq!(pos1 == pos2, false);
        assert_eq!(pos1 != pos2, true);
    }

    #[test]
    fn test_partial_eq_n5d1_n5d1() {
        let pos1 = Position::new(5, 1);
        let pos2 = Position::new(5, 1);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_eq_n2d1_n4d2() {
        let pos1 = Position::new(2, 1);
        let pos2 = Position::new(4, 2);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_eq_n4d2_n2d1() {
        let pos1 = Position::new(4, 2);
        let pos2 = Position::new(2, 1);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_eq_n8d2_n16d4() {
        let pos1 = Position::new(8, 2);
        let pos2 = Position::new(16, 4);

        assert_eq!(pos1 == pos2, true);
        assert_eq!(pos1 != pos2, false);
    }

    #[test]
    fn test_partial_ord_n1d1_n2d1() {
        let pos1 = Position::new(1, 1);
        let pos2 = Position::new(2, 1);

        assert_eq!(pos1 >= pos2, false);
        assert_eq!(pos1 <= pos2, true);
    }

    #[test]
    fn test_debug() {
        let pos1 = Position::new(1, 1);

        println!("{:?}", pos1);
    }
}
