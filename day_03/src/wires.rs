use std::collections::HashMap;

type Point = (i32, i32);
pub type Grid = HashMap<Point, u32>;

/// Lays out a wire on the grid, starting at (0,0) following the order of operations for the wire
/// Does not count a wire crossing itself twice
pub fn layout(grid: &mut Grid, wire: &str) {
    let operations = wire.split(",");
    let mut visited: Vec<Point> = Vec::new();
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;
    for operation in operations {
        let dir = &operation[0..1];
        let steps = operation[1..].parse::<usize>().unwrap();
        for _ in 0..steps {
            match dir {
                "U" => curr_y += 1,
                "D" => curr_y -= 1,
                "L" => curr_x -= 1,
                "R" => curr_x += 1,
                _ => panic!("Invalid direction given"),
            }
            visited.push((curr_x, curr_y));
        }
    }
    // We need to sort this and remove duplicate visited `Point`s,
    // so that we don't count a wire crossing itself twice.
    visited.sort();
    visited.dedup();
    for point in visited {
        let spot = grid.entry(point).or_insert(0);
        *spot += 1;
    }
}

/// Returns the closest point where `wire_count` wires cross from the origin, via Manhattan distance
pub fn crossing(grid: &Grid, wire_count: u32) -> u32 {
    let mut dist: u32 = 0;

    for key in grid.keys() {
        if grid[key] != wire_count {
            continue;
        }
        let this_dist = (key.0.abs() + key.1.abs()) as u32;
        if dist == 0 || this_dist < dist {
            dist = this_dist;
        }
    }
    dist
}

/// Lay out a wire, starting at (0,0) following the order of operations for the wire
/// Returns the grid of this with the amount of steps needed to reach each particular 'Point`
pub fn layout_with_distance(wire: &str) -> Grid {
    let mut grid = Grid::new();
    let mut distance: u32 = 0;
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;

    let operations = wire.split(",");
    for operation in operations {
        let dir = &operation[0..1];
        let steps = operation[1..].parse::<usize>().unwrap();
        for _ in 0..steps {
            match dir {
                "U" => curr_y += 1,
                "D" => curr_y -= 1,
                "L" => curr_x -= 1,
                "R" => curr_x += 1,
                _ => panic!("Invalid direction given"),
            }
            distance += 1;
            // Insert the distance travelled so far if we haven't yet hit this `Point`
            grid.entry((curr_x, curr_y)).or_insert(distance);
        }
    }
    grid
}

/// Given a set of wires (`Grid`s), return the shortest distance where they all intersect
pub fn crossing_by_wires(grids: &Vec<Grid>) -> u32 {
    let mut cross_grid = Grid::new();
    // First, layout the grid with all wires
    for grid in grids {
        for point in grid.keys() {
            let cross_point = cross_grid.entry(*point).or_insert(0);
            *cross_point += 1;
        }
    }

    // Next, find all points where all wires overlap, and check distance
    let mut distance: u32 = 0;
    let wire_count = grids.len() as u32;
    for key in cross_grid.keys() {
        if cross_grid[key] != wire_count {
            continue;
        }
        let this_distance = grids.iter().fold(0, |sum, g| sum + g[key]);
        if distance == 0 || this_distance < distance {
            distance = this_distance;
        }
    }
    distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_layout() {
        let mut g = Grid::new();
        let w = String::from("U3,R2,D2,L1");
        let mut res = Grid::new();
        res.insert((0, 1), 1);
        res.insert((0, 2), 1);
        res.insert((0, 3), 1);
        res.insert((1, 3), 1);
        res.insert((2, 3), 1);
        res.insert((2, 2), 1);
        res.insert((2, 1), 1);
        res.insert((1, 1), 1);
        layout(&mut g, &w);
        assert_eq!(g, res);
    }

    #[test]
    fn test_crossing() {
        let mut g = Grid::new();
        let w = String::from("U3,R2,D2,L1");
        layout(&mut g, &w);
        let w2 = String::from("R2,U4");
        layout(&mut g, &w2);
        assert_eq!(crossing(&g, 2), 3);
    }

    #[test]
    fn test_inputs() {
        let inputs1 = String::from(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        );
        let mut grid1 = Grid::new();
        for line in inputs1.split("\n") {
            layout(&mut grid1, &line);
        }
        assert_eq!(crossing(&grid1, 2), 159);

        let inputs2 = String::from(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        let mut grid2 = Grid::new();
        for line in inputs2.split("\n") {
            layout(&mut grid2, &line);
        }
        assert_eq!(crossing(&grid2, 2), 135);
    }

    #[test]
    fn test_layout_with_distance() {
        let mut res = Grid::new();
        res.insert((0, 1), 1);
        res.insert((0, 2), 2);
        res.insert((0, 3), 3);
        res.insert((1, 3), 4);
        res.insert((2, 3), 5);
        res.insert((2, 2), 6);
        res.insert((2, 1), 7);
        res.insert((1, 1), 8);
        let input = String::from("U3,R2,D2,L1");
        assert_eq!(layout_with_distance(&input), res);
    }

    #[test]
    fn test_crossing_by_wires() {
        let mut grids1 = Vec::new();
        let inputs1 = String::from(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        );
        for line in inputs1.split("\n") {
            grids1.push(layout_with_distance(&line));
        }
        assert_eq!(crossing_by_wires(&grids1), 610);

        let mut grids2 = Vec::new();
        let inputs2 = String::from(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        for line in inputs2.split("\n") {
            grids2.push(layout_with_distance(&line));
        }
        assert_eq!(crossing_by_wires(&grids2), 410);
    }
}
