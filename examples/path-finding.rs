extern crate tcod;

use tcod::AStarWithCallback;


fn main() {
    let chess_board: [[int, ..8], ..8] = [
        [1, 0, 1, 0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
    ];
    // Movement like in Checkers: you can only move to the square of the same colour
    let can_move = |from_x: int, from_y: int, to_x: int, to_y: int| -> f32 {
        if chess_board[from_x as uint][from_y as uint] == chess_board[to_x as uint][to_y as uint] {
            1.0
        } else {
            0.0
        }
    };
    let mut path = AStarWithCallback::new(4, 4, can_move, 1.0);
    path.find(0, 0, 1, 1);
    path.find(0, 0, 1, 0);
    assert_eq!(path.find(0, 0, 0, 1), false);
}
