#![feature(unboxed_closures)]

extern crate tcod;

use tcod::DijkstraPath;

fn create_path() -> DijkstraPath<'static> {
    let chess_board: [[isize; 8]; 8] = [
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
    let can_move = move |&mut: from: (isize, isize), to: (isize, isize)| -> f32 {
        let (fx, fy) = from;
        let (tx, ty) = to;
        if chess_board[fy as usize][fx as usize] == chess_board[ty as usize][tx as usize] {
            1.0
        } else {
            0.0
        }
    };
    DijkstraPath::new_from_callback(8, 8, can_move, 1.0)
}

fn walk_from(path: &mut DijkstraPath, origin: (isize, isize)) {
    path.find(origin);
    path.reverse();
    println!("Starting from: {:?}", origin);
    for pos in path.walk() {
        println!("Walking to: {:?}", pos);
    }
    println!("Arrived at the destination!\n");
}


fn main() {
    let mut path = create_path();
    let destination = (0, 0);
    path.compute_grid(destination);

    // Let's find multiple paths leading to (0, 0)
    walk_from(&mut path, (4, 6));
    walk_from(&mut path, (0, 6));
    walk_from(&mut path, (0, 0));
    walk_from(&mut path, (6, 2));
}
