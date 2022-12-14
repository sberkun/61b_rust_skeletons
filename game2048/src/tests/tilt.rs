use super::utils::{board, board_from_list, test_tilt, test_tilt_no_changes};
use crate::game::{Direction, MovingTile};

#[test]
#[rustfmt::skip::macros(board)]
fn test_triple_merge_1() {
    let before = board![
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [4, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 0, 0, 0, 2),
        MovingTile::new(0, 1, 0, 0, 2),
        MovingTile::new(0, 2, 0, 1, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_triple_merge_2() {
    let before = board![
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 0, 0, 0],
        [4, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 2, 0, 3, 2),
        MovingTile::new(0, 1, 0, 3, 2),
        MovingTile::new(0, 0, 0, 2, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_quadruple_merge() {
    let before = board![
        [0, 0, 0, 2],
        [0, 0, 0, 2],
        [0, 0, 0, 2],
        [0, 0, 0, 2]
    ];
    let after = board![
        [0, 0, 0, 4],
        [0, 0, 0, 4],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(3, 0, 3, 0, 2),
        MovingTile::new(3, 1, 3, 0, 2),
        MovingTile::new(3, 2, 3, 1, 2),
        MovingTile::new(3, 3, 3, 1, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 8);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_single_merge_up() {
    let before = board![
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 0, 0, 0],
        [4, 0, 0, 0]
    ];
    let after = board![
        [4, 0, 0, 0],
        [4, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 0, 0, 0, 2),
        MovingTile::new(0, 1, 0, 0, 2),
        MovingTile::new(0, 3, 0, 1, 4),
    ];
    test_tilt(&before, Direction::North, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_single_merge_south() {
    let before = board![
        [4, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [4, 0, 0, 0],
        [4, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 3, 0, 3, 2),
        MovingTile::new(0, 2, 0, 3, 2),
        MovingTile::new(0, 0, 0, 2, 4),
    ];
    test_tilt(&before, Direction::South, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_single_merge_east() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [4, 0, 2, 2]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 4, 4]
    ];
    let moves = vec![
        MovingTile::new(3, 3, 3, 3, 2),
        MovingTile::new(2, 3, 3, 3, 2),
        MovingTile::new(0, 3, 2, 3, 4),
    ];
    test_tilt(&before, Direction::East, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_single_merge_west() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 2, 0, 4]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [4, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 3, 0, 3, 2),
        MovingTile::new(1, 3, 0, 3, 2),
        MovingTile::new(3, 3, 1, 3, 4),
    ];
    test_tilt(&before, Direction::West, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_no_move() {
    let before = board![
        [2, 0, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    test_tilt_no_changes(&before, Direction::North);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_no_merge() {
    let before = board![
        [0, 0, 4, 0],
        [0, 0, 0, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 4, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 0, 2, 0, 4),
        MovingTile::new(3, 1, 3, 0, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_adjacent_no_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 4, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 1, 2, 0, 4),
        MovingTile::new(2, 2, 2, 1, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_non_adjacent_no_merge_1() {
    let before = board![
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 0, 2, 0, 4),
        MovingTile::new(2, 2, 2, 1, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_move_up_non_adjacent_no_merge_2() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 1, 2, 0, 4),
        MovingTile::new(2, 3, 2, 1, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_adjacent_merge() {
    let before = board![
        [0, 0, 2, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 0, 2, 0, 2),
        MovingTile::new(2, 1, 2, 0, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_non_adjacent_merge() {
    let before = board![
        [0, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 0, 2, 0, 2),
        MovingTile::new(2, 3, 2, 0, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_adjacent_merge_move() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 1, 2, 0, 2),
        MovingTile::new(2, 2, 2, 0, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_no_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 4],
        [0, 0, 2, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 4],
        [0, 0, 0, 2]
    ];
    let moves = vec![
        MovingTile::new(3, 2, 3, 2, 4),
        MovingTile::new(2, 3, 3, 3, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_adjacent_no_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 4, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 4],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 2, 3, 2, 4),
        MovingTile::new(1, 2, 2, 2, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_non_adjacent_no_merge_1() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 0, 4],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 4],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(3, 2, 3, 2, 4),
        MovingTile::new(1, 2, 2, 2, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_non_adjacent_no_merge_2() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 0, 4, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 4],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 2, 3, 2, 4),
        MovingTile::new(0, 2, 2, 2, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_adjacent_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 2],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 4],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(3, 2, 3, 2, 2),
        MovingTile::new(2, 2, 3, 2, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_non_adjacent_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 0, 0, 2],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 4],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(3, 2, 3, 2, 2),
        MovingTile::new(0, 2, 3, 2, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_adjacent_merge_move() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 2, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 4],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 2, 3, 2, 2),
        MovingTile::new(1, 2, 3, 2, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_right_non_adjacent_merge_move() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 0, 2, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 4],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 2, 3, 2, 2),
        MovingTile::new(0, 2, 3, 2, 2),
    ];
    test_tilt(&before, Direction::East, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_down_no_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 4, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 3, 1, 3, 4),
        MovingTile::new(0, 2, 0, 3, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_down_adjacent_no_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 4, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 2, 1, 3, 4),
        MovingTile::new(1, 1, 1, 2, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_down_non_adjacent_no_merge_1() {
    let before = board![
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 4, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 3, 1, 3, 4),
        MovingTile::new(1, 1, 1, 2, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_down_adjacent_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 2, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 3, 1, 3, 2),
        MovingTile::new(1, 2, 1, 3, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_down_non_adjacent_merge() {
    let before = board![
        [0, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 3, 1, 3, 2),
        MovingTile::new(1, 0, 1, 3, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_down_adjacent_merge_move() {
    let before = board![
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 2, 1, 3, 2),
        MovingTile::new(1, 1, 1, 3, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_down_non_adjacent_merge_move() {
    let before = board![
        [0, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 4, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 2, 1, 3, 2),
        MovingTile::new(1, 0, 1, 3, 2),
    ];
    test_tilt(&before, Direction::South, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_no_merge() {
    let before = board![
        [4, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [4, 0, 0, 0],
        [2, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 1, 0, 1, 2),
        MovingTile::new(0, 0, 0, 0, 4),
    ];
    test_tilt(&before, Direction::West, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_adjacent_no_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 4, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [4, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 1, 0, 1, 4),
        MovingTile::new(2, 1, 1, 1, 2),
    ];
    test_tilt(&before, Direction::West, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_non_adjacent_no_merge_1() {
    let before = board![
        [0, 0, 0, 0],
        [4, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [4, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 1, 0, 1, 4),
        MovingTile::new(2, 1, 1, 1, 2),
    ];
    test_tilt(&before, Direction::West, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_adjacent_merge() {
    let before = board![
        [0, 0, 0, 0],
        [2, 2, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [4, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 1, 0, 1, 2),
        MovingTile::new(1, 1, 0, 1, 2),
    ];
    test_tilt(&before, Direction::West, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_non_adjacent_merge() {
    let before = board![
        [0, 0, 0, 0],
        [2, 0, 0, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [4, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 1, 0, 1, 2),
        MovingTile::new(3, 1, 0, 1, 2),
    ];
    test_tilt(&before, Direction::West, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_adjacent_merge_move() {
    let before = board![
        [0, 0, 0, 0],
        [0, 2, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [4, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 1, 0, 1, 2),
        MovingTile::new(2, 1, 0, 1, 2),
    ];
    test_tilt(&before, Direction::West, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_non_adjacent_merge_move() {
    let before = board![
        [0, 0, 0, 0],
        [0, 2, 0, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 0, 0],
        [4, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(1, 1, 0, 1, 2),
        MovingTile::new(3, 1, 0, 1, 2),
    ];
    test_tilt(&before, Direction::West, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_many_merges_up() {
    let before = board![
        [0, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 0, 8]
    ];
    let after = board![
        [16, 16, 16, 16],
        [8, 16, 8, 16],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 1, 0, 0, 8),
        MovingTile::new(0, 2, 0, 0, 8),
        MovingTile::new(0, 3, 0, 1, 8),
        MovingTile::new(1, 0, 1, 0, 8),
        MovingTile::new(1, 1, 1, 0, 8),
        MovingTile::new(1, 2, 1, 1, 8),
        MovingTile::new(1, 3, 1, 1, 8),
        MovingTile::new(2, 0, 2, 0, 8),
        MovingTile::new(2, 1, 2, 0, 8),
        MovingTile::new(2, 2, 2, 1, 8),
        MovingTile::new(3, 0, 3, 0, 8),
        MovingTile::new(3, 1, 3, 0, 8),
        MovingTile::new(3, 2, 3, 1, 8),
        MovingTile::new(3, 3, 3, 1, 8),
    ];
    test_tilt(&before, Direction::North, &after, moves, 96);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_many_merges_down() {
    let before = board![
        [0, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 0, 8]
    ];
    let after = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [8, 16, 8, 16],
        [16, 16, 16, 16]
    ];
    let moves = vec![
        MovingTile::new(3, 3, 3, 3, 8),
        MovingTile::new(3, 2, 3, 3, 8),
        MovingTile::new(3, 1, 3, 2, 8),
        MovingTile::new(3, 0, 3, 2, 8),
        MovingTile::new(2, 2, 2, 3, 8),
        MovingTile::new(2, 1, 2, 3, 8),
        MovingTile::new(2, 0, 2, 2, 8),
        MovingTile::new(1, 3, 1, 3, 8),
        MovingTile::new(1, 2, 1, 3, 8),
        MovingTile::new(1, 1, 1, 2, 8),
        MovingTile::new(1, 0, 1, 2, 8),
        MovingTile::new(0, 3, 0, 3, 8),
        MovingTile::new(0, 2, 0, 3, 8),
        MovingTile::new(0, 1, 0, 2, 8),
    ];
    test_tilt(&before, Direction::South, &after, moves, 96);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_many_merges_right() {
    let before = board![
        [0, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 0, 8]
    ];
    let after = board![
        [0, 0, 8, 16],
        [0, 0, 16, 16],
        [0, 0, 16, 16],
        [0, 0, 8, 16]
    ];
    let moves = vec![
        MovingTile::new(3, 0, 3, 0, 8),
        MovingTile::new(2, 0, 3, 0, 8),
        MovingTile::new(1, 0, 2, 0, 8),
        MovingTile::new(3, 1, 3, 1, 8),
        MovingTile::new(2, 1, 3, 1, 8),
        MovingTile::new(1, 1, 2, 1, 8),
        MovingTile::new(0, 1, 2, 1, 8),
        MovingTile::new(3, 2, 3, 2, 8),
        MovingTile::new(2, 2, 3, 2, 8),
        MovingTile::new(1, 2, 2, 2, 8),
        MovingTile::new(0, 2, 2, 2, 8),
        MovingTile::new(3, 3, 3, 3, 8),
        MovingTile::new(1, 3, 3, 3, 8),
        MovingTile::new(0, 3, 2, 3, 8),
    ];
    test_tilt(&before, Direction::East, &after, moves, 96);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_many_merges_left() {
    let before = board![
        [0, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 8, 8],
        [8, 8, 0, 8]
    ];
    let after = board![
        [16, 8, 0, 0],
        [16, 16, 0, 0],
        [16, 16, 0, 0],
        [16, 8, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(0, 3, 0, 3, 8),
        MovingTile::new(1, 3, 0, 3, 8),
        MovingTile::new(3, 3, 1, 3, 8),
        MovingTile::new(0, 2, 0, 2, 8),
        MovingTile::new(1, 2, 0, 2, 8),
        MovingTile::new(2, 2, 1, 2, 8),
        MovingTile::new(3, 2, 1, 2, 8),
        MovingTile::new(0, 1, 0, 1, 8),
        MovingTile::new(1, 1, 0, 1, 8),
        MovingTile::new(2, 1, 1, 1, 8),
        MovingTile::new(3, 1, 1, 1, 8),
        MovingTile::new(1, 0, 0, 0, 8),
        MovingTile::new(2, 0, 0, 0, 8),
        MovingTile::new(3, 0, 1, 0, 8),
    ];
    test_tilt(&before, Direction::West, &after, moves, 96);
}
