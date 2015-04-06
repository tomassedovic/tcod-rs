extern crate tcod;

use tcod::pathfinding::AStar;

fn create_path() -> AStar<'static> {
    let chess_board: [[i32; 8]; 8] = [
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
    let can_move = move |from: (i32, i32), to: (i32, i32)| -> f32 {
        let (fx, fy) = from;
        let (tx, ty) = to;
        if chess_board[fy as usize][fx as usize] == chess_board[ty as usize][tx as usize] {
            1.0
        } else {
            0.0
        }
    };
    AStar::new_from_callback(8, 8, can_move, 1.0)
}

fn main() {
    let mut path = create_path();
    assert_eq!(path.find((0, 0), (1, 1)), true);
    assert_eq!(path.len(), 1);
    assert_eq!(path.is_empty(), false);
    assert_eq!(path.find((0, 0), (0, 1)), false);
    assert_eq!(path.len(), 0);
    assert_eq!(path.is_empty(), true);

    assert_eq!(path.find((0, 0), (0, 6)), true);
    assert_eq!(path.len(), 6);
    assert_eq!(path.origin(), (0, 0));
    assert_eq!(path.destination(), (0, 6));

    // Explore the path:
    assert_eq!(path.get(0), Some((1, 1)));
    assert_eq!(path.get(1), Some((0, 2)));
    assert_eq!(path.get(2), Some((1, 3)));
    assert_eq!(path.get(3), Some((0, 4)));
    assert_eq!(path.get(4), Some((1, 5)));
    assert_eq!(path.get(5), Some((0, 6)));

    // Make sure we don't segfault on invalid index
    assert_eq!(path.get(-1), None);
    assert_eq!(path.get(6), None);
    assert_eq!(path.get(7), None);


    // Walk the path (consuming it):
    for pos in path.walk() {
        println!("Walking to {:?}", pos);
    }

    assert_eq!(path.len(), 0);
    assert_eq!(path.is_empty(), true);
    // Note: origin has moved to the destination:
    assert_eq!(path.origin(), (0, 6));
    assert_eq!(path.destination(), (0, 6));
    assert_eq!(path.get(0), None);
}
