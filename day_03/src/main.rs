use std::fs;

mod wires;

fn main() {
    let file = fs::read_to_string("src/inputs").unwrap();

    let mut grid = wires::Grid::new();
    let mut wire_count = 0;
    for line in file.split("\n") {
        wires::layout(&mut grid, &line);
        wire_count += 1;
    }

    let manhattan_distance = wires::crossing(&grid, wire_count);
    println!(
        "Shortest Manhattan distance for given inputs: {}",
        manhattan_distance
    );

    let mut grids = Vec::new();
    for line in file.split("\n") {
        grids.push(wires::layout_with_distance(&line));
    }
    let travelled_distance = wires::crossing_by_wires(&grids);
    println!(
        "Shortest travelled distance for given inputs: {}",
        travelled_distance
    );
}
