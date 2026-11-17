use std::collections::VecDeque;

pub const MAP: usize = 16;
pub type Cell = (usize, usize);
pub const START: Cell = (2, 2);
pub const EXIT: Cell = (13, 13);
pub const LEVEL_COUNT: usize = 5;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Floor,
    Wall,
}
pub type Map = [[Tile; MAP]; MAP];

pub struct Level {
    pub name: &'static str,
    pub subtitle: &'static str,
    pub theme: usize,
    pub map: Map,
    pub enemies: Vec<Cell>,
    pub coins: Vec<Cell>,
}

pub fn level(index: usize) -> Level {
    assert!(index < LEVEL_COUNT);
    let (name, subtitle, theme) = [
        ("BARRIO VERDE", "La fuga empieza entre jardines.", 0),
        (
            "EL CALLEJON",
            "Encuentra el camino entre las barricadas.",
            1,
        ),
        ("DISTRITO OCRE", "Las patrullas ya saben que vienes.", 2),
        ("EL LABERINTO", "Cada esquina es una emboscada.", 0),
        (
            "ULTIMA FRONTERA",
            "Derrota al capitan y alcanza la libertad.",
            2,
        ),
    ][index];
    let mut map = [[Tile::Floor; MAP]; MAP];
    for (y, row) in map.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            let border = x == 0 || y == 0 || x == MAP - 1 || y == MAP - 1;
            let obstacle = match index {
                0 => {
                    ((4..=6).contains(&x) && y == 5)
                        || (x == 10 && (8..=11).contains(&y))
                        || (y == 11 && (3..=5).contains(&x))
                }
                1 => {
                    (x == 5 && (2..=11).contains(&y) && y != 7)
                        || (x == 10 && (4..=13).contains(&y) && y != 10)
                }
                2 => {
                    ((4..=6).contains(&x) && (4..=6).contains(&y))
                        || ((9..=11).contains(&x) && (9..=11).contains(&y))
                        || (y == 4 && (10..=12).contains(&x))
                }
                3 => {
                    ((y == 4 || y == 10) && (2..=12).contains(&x) && x != 7)
                        || (y == 7 && (4..=13).contains(&x) && x != 10)
                }
                _ => {
                    ((x == 4 || x == 11) && (4..=11).contains(&y) && y != 7)
                        || ((y == 4 || y == 11) && (4..=11).contains(&x) && x != 7 && x != 8)
                }
            };
            if border || obstacle {
                *tile = Tile::Wall;
            }
        }
    }
    let spawns = [(8, 3), (12, 7), (5, 12), (13, 11), (8, 9), (3, 9), (12, 3)];
    let enemies = spawns[..3 + index].to_vec();
    let coins = vec![(3, 3), (7, 6), (12, 2), (2, 12), (9, 13), (13, 13)];
    for &(x, y) in enemies.iter().chain(&coins).chain([START, EXIT].iter()) {
        map[y][x] = Tile::Floor;
    }
    Level {
        name,
        subtitle,
        theme,
        map,
        enemies,
        coins,
    }
}

pub fn dist(a: Cell, b: Cell) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn neighbor(pos: Cell, dx: i32, dy: i32) -> Option<Cell> {
    let (x, y) = (pos.0 as i32 + dx, pos.1 as i32 + dy);
    (x >= 0 && y >= 0 && x < MAP as i32 && y < MAP as i32).then_some((x as usize, y as usize))
}

pub fn bfs(map: &Map, start: Cell, goal: Cell) -> Vec<Cell> {
    if start.0 >= MAP
        || start.1 >= MAP
        || goal.0 >= MAP
        || goal.1 >= MAP
        || map[start.1][start.0] == Tile::Wall
        || map[goal.1][goal.0] == Tile::Wall
    {
        return vec![];
    }
    let mut queue = VecDeque::from([start]);
    let mut seen = [[false; MAP]; MAP];
    let mut parent = [[None; MAP]; MAP];
    seen[start.1][start.0] = true;
    while let Some(cell) = queue.pop_front() {
        if cell == goal {
            let mut path = vec![];
            let mut current = goal;
            while current != start {
                path.push(current);
                current = parent[current.1][current.0].unwrap();
            }
            path.reverse();
            return path;
        }
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            if let Some((x, y)) = neighbor(cell, dx, dy)
                && !seen[y][x]
                && map[y][x] == Tile::Floor
            {
                seen[y][x] = true;
                parent[y][x] = Some(cell);
                queue.push_back((x, y));
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_level_has_reachable_objectives_and_floor_tiles() {
        for index in 0..LEVEL_COUNT {
            let l = level(index);
            for (y, row) in l.map.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if *tile == Tile::Floor && (x, y) != START {
                        let path = bfs(&l.map, START, (x, y));
                        assert!(!path.is_empty(), "Level {index}: unreachable tile {x},{y}");
                        let mut prev = START;
                        for step in path {
                            assert_eq!(dist(prev, step), 1);
                            prev = step;
                        }
                    }
                }
            }
            assert_eq!(l.enemies.len(), 3 + index);
            for pos in l.enemies.iter().chain(&l.coins).chain([EXIT].iter()) {
                assert_eq!(l.map[pos.1][pos.0], Tile::Floor);
                assert_ne!(*pos, START);
            }
        }
    }
    #[test]
    fn pathfinding_rejects_walls_and_outside_coordinates() {
        let map = level(0).map;
        assert!(bfs(&map, START, (0, 0)).is_empty());
        assert!(bfs(&map, START, (MAP, MAP)).is_empty());
        assert!(bfs(&map, (MAP, MAP), START).is_empty());
        assert_eq!(neighbor((0, 0), -1, 0), None);
    }
}
