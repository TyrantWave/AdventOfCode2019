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

    let distance = wires::crossing(&grid, wire_count);
    println!("Shortest distance for given inputs: {}", distance);
}
