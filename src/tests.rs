#[cfg(test)]

mod create_solve_board {
    use crate::{gen_board, solve_puzzle, valid_board};
    fn run_solve_puzzle_test(dim: usize, test_name: &str) {
        let board = gen_board(dim);
        let mut solved = board.clone();
        solve_puzzle(&mut solved);
        assert!(
            valid_board(&solved),
            "Invalid board solved for {}",
            test_name
        );

        for row in 0..dim {
            for col in 0..dim {
                if board.board[row][col] != 0 {
                    assert!(
                        board.board[row][col] == solved.board[row][col],
                        "Board solve failed for {} at ({}, {})",
                        test_name,
                        row,
                        col
                    );
                }
            }
        }
    }

    #[test]
    fn solve_puzzle_test_1() {
        run_solve_puzzle_test(9, "test no. 1")
    }

    #[test]
    fn solve_puzzle_test_2() {
        run_solve_puzzle_test(9, "test no. 2")
    }

    #[test]
    fn solve_puzzle_test_3() {
        run_solve_puzzle_test(9, "test no. 3")
    }

    #[test]
    fn solve_puzzle_test_4() {
        run_solve_puzzle_test(9, "test no. 4")
    }

    #[test]
    fn solve_puzzle_test_5() {
        run_solve_puzzle_test(9, "test no. 5")
    }

    #[test]
    fn solve_puzzle_test_6() {
        run_solve_puzzle_test(9, "test no. 6")
    }

    #[test]
    fn solve_puzzle_test_7() {
        run_solve_puzzle_test(9, "test no. 7")
    }

    #[test]
    fn solve_puzzle_test_8() {
        run_solve_puzzle_test(9, "test no. 8")
    }

    #[test]
    fn solve_puzzle_test_9() {
        run_solve_puzzle_test(9, "test no. 9")
    }

    #[test]
    fn solve_puzzle_test_10() {
        run_solve_puzzle_test(9, "test no. 10")
    }

    #[test]
    fn solve_puzzle_test_11() {
        run_solve_puzzle_test(9, "test no. 11")
    }

    #[test]
    fn solve_puzzle_test_12() {
        run_solve_puzzle_test(9, "test no. 12")
    }

    #[test]
    fn solve_puzzle_test_13() {
        run_solve_puzzle_test(9, "test no. 13")
    }

    #[test]
    fn solve_puzzle_test_14() {
        run_solve_puzzle_test(9, "test no. 14")
    }

    #[test]
    fn solve_puzzle_test_15() {
        run_solve_puzzle_test(9, "test no. 15")
    }

    #[test]
    fn solve_puzzle_test_16() {
        run_solve_puzzle_test(9, "test no. 16")
    }

    #[test]
    fn solve_puzzle_test_17() {
        run_solve_puzzle_test(9, "test no. 17")
    }

    #[test]
    fn solve_puzzle_test_18() {
        run_solve_puzzle_test(9, "test no. 18")
    }

    #[test]
    fn solve_puzzle_test_19() {
        run_solve_puzzle_test(9, "test no. 19")
    }

    #[test]
    fn solve_puzzle_test_20() {
        run_solve_puzzle_test(9, "test no. 20")
    }
}

mod gen_board {
    use crate::{Sudoku, SudokuParseError};
    use std::str::FromStr;
    #[test]
    fn test_valid_input_9x9() {
        let input = "
            -------------------
            |1 2 3|4 5 6|7 8 9|
            |4 5 6|7 8 9|1 2 3|
            |7 8 9|1 2 3|4 5 6|
            -------------------
            |2 3 4|5 6 7|8 9 1|
            |5 6 7|8 9 1|2 3 4|
            |8 9 1|2 3 4|5 6 7|
            -------------------
            |3 4 5|6 7 8|9 1 2|
            |6 7 8|9 1 2|3 4 5|
            |9 1 2|3 4 5|6 7 8|
            -------------------
        ";
        let sudoku = Sudoku::from_str(input).unwrap();
        let actual_board: Vec<Vec<u8>> = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 5, 6, 7, 8, 9, 1, 2, 3],
            vec![7, 8, 9, 1, 2, 3, 4, 5, 6],
            vec![2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![5, 6, 7, 8, 9, 1, 2, 3, 4],
            vec![8, 9, 1, 2, 3, 4, 5, 6, 7],
            vec![3, 4, 5, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 8, 9, 1, 2, 3, 4, 5],
            vec![9, 1, 2, 3, 4, 5, 6, 7, 8],
        ];

        assert_eq!(sudoku.board, actual_board);
    }

    #[test]
    fn test_invalid_input_missing_values() {
        let input = "
            -------------------
            |1 2 3|4 5 6|7 8 9|
            |4 5 6|7 8 9|1 2 3|
            |7 8 9|1 2 3|4 5 6|
            -------------------
            |2 3 4|5 6 7|8 9 1|
            |5 6 7|8 9 1|2 3 4|
            |8 9 1|2  4|5 6 7|
            -------------------
            |3 4 5|6 7 8|9 1 2|
            |6 7 8|9 1 2|3 4 5|
            |9 1 2|3 4 5|6 7 8|
            -------------------
        ";
        let result = Sudoku::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(SudokuParseError::InvalidFormat));
    }

    #[test]
    fn test_invalid_input_non_numeric() {
        let input = "
            -------------------
            |b 2 3|4 5 6|7 8 9|
            |4 5 6|7 8 9|1 2 3|
            |7 8 9|1 2 3|4 5 6|
            -------------------
            |2 3 4|5 6 7|8 9 1|
            |5 6 7|8 9 1|2 3 4|
            |8 9 1|2 3 4|5 6 7|
            -------------------
            |3 4 5|6 7 8|9 1 2|
            |6 7 8|9 1 2|3 4 a|
            |9 1 2|3 4 5|6 7 5|
            -------------------
        ";
        let result = Sudoku::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(SudokuParseError::InvalidCharacter));
    }

    #[test]
    fn test_invalid_input_missing_row() {
        let input = "
            -------------------
            |1 2 3|4 5 6|7 8 9|
            |4 5 6|7 8 9|1 2 3|
            -------------------
            |2 3 4|5 6 7|8 9 1|
            |5 6 7|8 9 1|2 3 4|
            |8 9 1|2 3 4|5 6 7|
            -------------------
            |3 4 5|6 7 8|9 1 2|
            |6 7 8|9 1 2|3 4 5|
            |9 1 2|3 4 5|6 7 8|
            -------------------
        ";
        let result = Sudoku::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(SudokuParseError::InvalidFormat));
    }
}
