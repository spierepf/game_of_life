use std::collections::HashSet;

#[derive(Eq, Copy, PartialEq, Hash, Clone)]
struct Cell {
    x: i64,
    y: i64,
}

impl Cell {
    fn neighbors(self: &Cell) -> HashSet<Cell> {
        let mut retval = HashSet::new();

        for x in -1..2 {
            for y in -1..2 {
                if x != 0 || y != 0 {
                    retval.insert(Cell { x: self.x + x, y: self.y + y });
                }
            }
        }

        return retval;
    }
}

#[test]
fn test_cell_neighbors() {
    let cell = Cell { x: 0, y: 0 };
    let neighbors = cell.neighbors();

    // A Cell has eight neighbors
    assert_eq!(8, neighbors.len());
    assert_eq!(true, neighbors.contains(&Cell { x: -1, y: -1 }));
    assert_eq!(true, neighbors.contains(&Cell { x: -1, y: 0 }));
    assert_eq!(true, neighbors.contains(&Cell { x: -1, y: 1 }));

    assert_eq!(true, neighbors.contains(&Cell { x: 0, y: -1 }));
    assert_eq!(true, neighbors.contains(&Cell { x: 0, y: 1 }));

    assert_eq!(true, neighbors.contains(&Cell { x: 1, y: -1 }));
    assert_eq!(true, neighbors.contains(&Cell { x: 1, y: 0 }));
    assert_eq!(true, neighbors.contains(&Cell { x: 1, y: 1 }));

    // A Cell is not its own neighbor
    assert_eq!(false, neighbors.contains(&cell));
}

fn neighborhood(alive: HashSet<Cell>) -> HashSet<Cell> {
    let mut retval = alive.clone();

    for c in alive {
        retval.extend(c.neighbors());
    }

    return retval;
}

#[test]
fn test_neighborhood() {
    let mut alive = HashSet::new();

    alive.insert(Cell { x: 0, y: 1 });
    assert_eq!(9, neighborhood(alive.clone()).len());

    alive.insert(Cell { x: 1, y: 1 });
    alive.insert(Cell { x: -1, y: 0 });
    alive.insert(Cell { x: 0, y: 0 });
    alive.insert(Cell { x: 0, y: -1 });

    assert_eq!(21, neighborhood(alive).len());
}

fn is_alive_in_next_generation(current_generation: HashSet<Cell>, cell: Cell) -> bool {
    let alive = current_generation.contains(&cell);
    let living_neighbors = current_generation.intersection(&cell.neighbors()).count();

    if alive && living_neighbors >= 2 && living_neighbors < 4 {
        return true;
    }

    if !alive && living_neighbors == 3 {
        return true;
    }

    return false;
}

#[test]
fn test_is_alive_in_next_generation() {
    let mut current_generation = HashSet::new();

    // A living cell with zero or one neighbor dies from isolation
    current_generation.insert(Cell { x: 0, y: 0 });
    assert_eq!(false, is_alive_in_next_generation(current_generation.clone(), Cell { x: 0, y: 0 }));
    current_generation.insert(Cell { x: 0, y: 1 });
    assert_eq!(false, is_alive_in_next_generation(current_generation.clone(), Cell { x: 0, y: 0 }));

    // A living cell with two or three neighbors survives to the next generation
    current_generation.insert(Cell { x: 1, y: 1 });
    assert_eq!(true, is_alive_in_next_generation(current_generation.clone(), Cell { x: 0, y: 0 }));
    current_generation.insert(Cell { x: -1, y: 0 });
    assert_eq!(true, is_alive_in_next_generation(current_generation.clone(), Cell { x: 0, y: 0 }));

    // A living cell with four or more neighbors dies from overcrowding
    current_generation.insert(Cell { x: 0, y: -1 });
    assert_eq!(false, is_alive_in_next_generation(current_generation.clone(), Cell { x: 0, y: 0 }));

    // A dead cell with precisely three neighbors is born into the next generation
    assert_eq!(true, is_alive_in_next_generation(current_generation.clone(), Cell { x: -1, y: 1 }));
}


fn main() {
    println!("Hello, world!");

    let mut current_generation = HashSet::new();

    current_generation.insert(Cell { x: 0, y: 1 });
    current_generation.insert(Cell { x: 1, y: 1 });
    current_generation.insert(Cell { x: -1, y: 0 });
    current_generation.insert(Cell { x: 0, y: 0 });
    current_generation.insert(Cell { x: 0, y: -1 });

    for i in 1..1104 {
        let neighborhood = neighborhood(current_generation.clone());

        let mut next_generation = HashSet::new();

        for cell in &neighborhood {
            if is_alive_in_next_generation(current_generation.clone(), *cell) {
                next_generation.insert(cell);
            }
        }

        current_generation.clear();
        current_generation.extend(next_generation);
        println!("{} {}", i, current_generation.clone().len())
    }
}
