use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Method {
    Backtrack,
    Prim,
    Kruskal,
}

struct Maze {
    width: usize,
    height: usize,
    passages: Vec<Vec<usize>>,
}

impl Maze {
    fn neighbors(&self, cell: usize) -> Vec<usize> {
        let (x, y) = (cell % self.width, cell / self.width);
        let mut neighbors = Vec::with_capacity(4);
        if x > 0 {
            neighbors.push(cell - 1);
        }
        if x + 1 < self.width {
            neighbors.push(cell + 1);
        }
        if y > 0 {
            neighbors.push(cell - self.width);
        }
        if y + 1 < self.height {
            neighbors.push(cell + self.width);
        }
        neighbors
    }

    fn join(&mut self, a: usize, b: usize) {
        self.passages[a].push(b);
        self.passages[b].push(a);
    }

    fn distances(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.passages.len()];
        distances[start] = 0;
        let mut queue = VecDeque::from([start]);
        while let Some(cell) = queue.pop_front() {
            for &next in &self.passages[cell] {
                if distances[next] == usize::MAX {
                    distances[next] = distances[cell] + 1;
                    queue.push_back(next);
                }
            }
        }
        distances
    }

    fn perimeter(&self) -> Vec<usize> {
        (0..self.passages.len())
            .filter(|&i| {
                i % self.width == 0
                    || i % self.width + 1 == self.width
                    || i < self.width
                    || i >= self.width * (self.height - 1)
            })
            .collect()
    }
}

fn root(parents: &mut [usize], mut cell: usize) -> usize {
    while parents[cell] != cell {
        parents[cell] = parents[parents[cell]];
        cell = parents[cell];
    }
    cell
}

fn carve(rng: &mut StdRng, width: usize, height: usize, method: Method) -> Maze {
    let mut maze = Maze {
        width,
        height,
        passages: vec![Vec::new(); width * height],
    };
    match method {
        Method::Backtrack => {
            let start = rng.random_range(0..width * height);
            let mut visited = vec![false; width * height];
            visited[start] = true;
            let mut stack = vec![start];
            while let Some(&cell) = stack.last() {
                let options: Vec<_> = maze
                    .neighbors(cell)
                    .into_iter()
                    .filter(|&i| !visited[i])
                    .collect();
                if options.is_empty() {
                    stack.pop();
                } else {
                    let next = options[rng.random_range(0..options.len())];
                    maze.join(cell, next);
                    visited[next] = true;
                    stack.push(next);
                }
            }
        }
        Method::Prim => {
            let start = rng.random_range(0..width * height);
            let mut visited = vec![false; width * height];
            visited[start] = true;
            let mut frontier: Vec<_> = maze
                .neighbors(start)
                .into_iter()
                .map(|next| (start, next))
                .collect();
            while !frontier.is_empty() {
                let index = rng.random_range(0..frontier.len());
                let (cell, next) = frontier.swap_remove(index);
                if visited[next] {
                    continue;
                }
                maze.join(cell, next);
                visited[next] = true;
                frontier.extend(
                    maze.neighbors(next)
                        .into_iter()
                        .filter(|&i| !visited[i])
                        .map(|i| (next, i)),
                );
            }
        }
        Method::Kruskal => {
            let mut parents: Vec<_> = (0..width * height).collect();
            let mut sizes = vec![1; width * height];
            let mut walls = Vec::new();
            for cell in 0..width * height {
                if cell % width + 1 < width {
                    walls.push((cell, cell + 1));
                }
                if cell / width + 1 < height {
                    walls.push((cell, cell + width));
                }
            }
            walls.shuffle(rng);
            for (a, b) in walls {
                let (mut ra, mut rb) = (root(&mut parents, a), root(&mut parents, b));
                if ra != rb {
                    if sizes[ra] < sizes[rb] {
                        std::mem::swap(&mut ra, &mut rb);
                    }
                    parents[rb] = ra;
                    sizes[ra] += sizes[rb];
                    maze.join(a, b);
                }
            }
        }
    }
    maze
}

fn entrances(maze: &Maze, rng: &mut StdRng) -> (usize, usize, usize) {
    let perimeter = maze.perimeter();
    if rng.random_bool(0.25) {
        let row = rng.random_range(0..maze.height);
        let start = row * maze.width;
        let end = rng.random_range(0..maze.height) * maze.width + maze.width - 1;
        return (start, end, maze.distances(start)[end]);
    }
    let mut best = (0, 0, 0);
    let mut ties = 0usize;
    for (i, &start) in perimeter.iter().enumerate() {
        let distances = maze.distances(start);
        for &end in &perimeter[i + 1..] {
            let distance = distances[end];
            if distance > best.2 {
                best = (start, end, distance);
                ties = 1;
            } else if distance == best.2 {
                ties += 1;
                if rng.random_range(0..ties) == 0 {
                    best = (start, end, distance);
                }
            }
        }
    }
    if rng.random_bool(0.5) {
        (best.1, best.0, best.2)
    } else {
        best
    }
}

fn opening(maze: &Maze, cell: usize, rng: &mut StdRng) -> (usize, usize) {
    let (x, y) = (cell % maze.width, cell / maze.width);
    let mut choices = Vec::new();
    if x == 0 {
        choices.push((0, 2 * y + 1));
    }
    if x + 1 == maze.width {
        choices.push((2 * maze.width, 2 * y + 1));
    }
    if y == 0 {
        choices.push((2 * x + 1, 0));
    }
    if y + 1 == maze.height {
        choices.push((2 * x + 1, 2 * maze.height));
    }
    choices[rng.random_range(0..choices.len())]
}

fn render(maze: &Maze, start: usize, end: usize, rng: &mut StdRng) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['#'; 2 * maze.width + 1]; 2 * maze.height + 1];
    for (cell, passages) in maze.passages.iter().enumerate() {
        let (x, y) = (2 * (cell % maze.width) + 1, 2 * (cell / maze.width) + 1);
        grid[y][x] = ' ';
        for &next in passages {
            let (nx, ny) = (2 * (next % maze.width) + 1, 2 * (next / maze.width) + 1);
            grid[(y + ny) / 2][(x + nx) / 2] = ' ';
        }
    }
    for (cell, marker) in [(start, 'S'), (end, 'E')] {
        grid[2 * (cell / maze.width) + 1][2 * (cell % maze.width) + 1] = marker;
        let (x, y) = opening(maze, cell, rng);
        grid[y][x] = ' ';
    }
    grid
}

pub fn generate(rng: &mut StdRng) -> String {
    let (width, height) = match rng.random_range(0..4) {
        0 => (rng.random_range(15..=20), rng.random_range(5..=7)),
        1 => (rng.random_range(5..=8), rng.random_range(10..=14)),
        2 => (rng.random_range(8..=12), rng.random_range(7..=11)),
        _ => (rng.random_range(3..=5), rng.random_range(12..=16)),
    };
    let method = [Method::Backtrack, Method::Prim, Method::Kruskal][rng.random_range(0..3)];
    let maze = carve(rng, width, height, method);
    let (start, end, distance) = entrances(&maze, rng);
    let dead_ends = maze
        .passages
        .iter()
        .enumerate()
        .filter(|(i, passages)| *i != start && *i != end && passages.len() == 1)
        .count();
    let image = render(&maze, start, end, rng)
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    format!("{image}\nS to E in {distance} steps, {dead_ends} dead ends")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn all_methods_make_reciprocal_connected_trees() {
        let mut rng = StdRng::seed_from_u64(845);
        for method in [Method::Backtrack, Method::Prim, Method::Kruskal] {
            for (width, height) in [(1, 1), (1, 20), (19, 1), (3, 16), (20, 7), (10, 10)] {
                for _ in 0..16 {
                    let maze = carve(&mut rng, width, height, method);
                    assert_eq!(
                        maze.passages.iter().map(Vec::len).sum::<usize>(),
                        2 * (width * height - 1)
                    );
                    assert!(!maze.distances(0).contains(&usize::MAX));
                    for (cell, passages) in maze.passages.iter().enumerate() {
                        for &next in passages {
                            assert!(maze.neighbors(cell).contains(&next));
                            assert!(maze.passages[next].contains(&cell));
                            assert_eq!(passages.iter().filter(|&&n| n == next).count(), 1);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn rendered_passages_preserve_the_tree_and_two_border_openings() {
        let mut rng = StdRng::seed_from_u64(2651);
        for method in [Method::Backtrack, Method::Prim, Method::Kruskal] {
            for _ in 0..48 {
                let maze = carve(&mut rng, 11, 8, method);
                let (start, end, distance) = entrances(&maze, &mut rng);
                assert_ne!(start, end);
                assert!(maze.perimeter().contains(&start));
                assert!(maze.perimeter().contains(&end));
                assert_eq!(maze.distances(start)[end], distance);
                let grid = render(&maze, start, end, &mut rng);
                let border_openings = grid
                    .iter()
                    .enumerate()
                    .map(|(y, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|&(x, c)| {
                                (x == 0 || y == 0 || x == row.len() - 1 || y == grid.len() - 1)
                                    && *c == ' '
                            })
                            .count()
                    })
                    .sum::<usize>();
                assert_eq!(border_openings, 2);
                for cell in 0..maze.passages.len() {
                    let (x, y) = (2 * (cell % maze.width) + 1, 2 * (cell / maze.width) + 1);
                    for next in maze.neighbors(cell) {
                        let (nx, ny) = (2 * (next % maze.width) + 1, 2 * (next / maze.width) + 1);
                        assert_eq!(
                            grid[(y + ny) / 2][(x + nx) / 2] == ' ',
                            maze.passages[cell].contains(&next)
                        );
                    }
                }
            }
        }
    }
}
