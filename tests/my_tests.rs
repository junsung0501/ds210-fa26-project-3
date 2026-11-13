//! Your tests of the board. Checkpoint 1 asks for at least three, on `score`.
//!
//!     cargo test --test my_tests
//!
//! Build a position with `Board::from_rows`, and assert what `score()` should say.
//! Think about what could go wrong in a `score` that passes a few easy cases: a
//! direction it forgets, an edge it runs off, a run it counts twice or not at all.

use tictactoe::board::Board;

#[test]
fn example_a_row_for_x() {
    let board = Board::from_rows(&["XXX", "OO.", "..."]);
    assert_eq!(board.score(), 1);
}

// Your tests go below. Give each one a name that says what it checks.


fn mine_board(rows: &[&str]) -> tictactoe::board::Board {
    tictactoe::board::Board::from_rows(rows)
}

#[test]
fn mine_moves_shrink_after_apply_and_come_back_after_undo() {
    let mut board = mine_board(&["...", "...", "..."]);
    board.apply_move((1, 1));
    assert_eq!(board.moves().len(), 8);
    assert!(!board.moves().contains(&(1, 1)));
    board.undo();
    assert_eq!(board.moves().len(), 9);
    assert!(board.moves().contains(&(1, 1)));
}

#[test]
fn mine_winning_move_ends_the_game_and_undo_reopens_it() {
    let mut board = mine_board(&["XX.", "OO.", "..."]);
    assert!(!board.is_over());
    board.apply_move((0, 2));
    assert_eq!(board.score(), 1);
    assert!(board.is_over());
    board.undo();
    assert_eq!(board.score(), 0);
    assert!(!board.is_over());
}

#[test]
fn mine_two_runs_that_share_a_corner_both_count() {
    let board = mine_board(&["XXX", "X.O", "XO."]);
    assert_eq!(board.score(), 2);
}

#[test]
fn mine_an_x_run_and_an_o_run_cancel_out_on_5x5() {
    let board = mine_board(&["XXX..", ".....", "OOO..", ".....", "....."]);
    assert_eq!(board.score(), 0);
    assert!(!board.is_over());
}
