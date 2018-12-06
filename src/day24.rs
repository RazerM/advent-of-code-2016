#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

use itertools::Itertools;
use maplit::hashmap;
use permutohedron::Heap;
use std::char;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Block {
    Wall,
    Path,
    Number(i32),
}

type Pos = (i32, i32);

struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl Display for Grid<Block> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let grid = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|block| char::from(*block))
                    .collect::<String>()
            }).collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", grid)
    }
}

impl<T> Grid<T> {
    fn get(&self, pos: Pos) -> &T {
        let (x, y) = pos;
        &self.grid[y as usize][x as usize]
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        let (x, y) = pos;
        x >= 0 && y >= 0 && x < self.grid[0].len() as i32 && y < self.grid.len() as i32
    }
}

impl Grid<Block> {
    fn is_wall(&self, pos: Pos) -> bool {
        match self.get(pos) {
            Block::Wall => true,
            _ => false,
        }
    }

    fn neighbours(&self, pos: Pos) -> Vec<Pos> {
        let (x, y) = pos;
        let neighbours = [(x + 1, y), (x, y - 1), (x - 1, y), (x, y + 1)];

        neighbours
            .iter()
            .cloned()
            .filter(|p| self.in_bounds(*p))
            .filter(|p| !self.is_wall(*p))
            .collect::<Vec<Pos>>()
    }
}

impl From<Block> for char {
    fn from(block: Block) -> Self {
        match block {
            Block::Wall => '#',
            Block::Path => '.',
            Block::Number(n) => char::from_digit(n as u32, 10).unwrap(),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", char::from(*self))
    }
}

struct PriorityQueue<V: Debug, P: Debug + Ord = i32> {
    heap: BinaryHeap<PriorityItem<V, P>>,
}

impl<V: Debug, P: Debug + Ord> PriorityQueue<V, P> {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn put(&mut self, item: V, priority: P) {
        self.heap.push(PriorityItem::new(item, priority));
    }

    fn get(&mut self) -> Option<V> {
        self.heap.pop().map(|pi| pi.value)
    }
}

#[derive(Debug)]
struct PriorityItem<V: Debug, P: Debug + Ord> {
    value: V,
    priority: P,
}

impl<V: Debug, P: Debug + Ord> PriorityItem<V, P> {
    fn new(value: V, priority: P) -> Self {
        PriorityItem { value, priority }
    }
}

impl<V: Debug, P: Debug + Ord> Eq for PriorityItem<V, P> {}

impl<V: Debug, P: Debug + Ord> PartialEq for PriorityItem<V, P> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<V: Debug, P: Debug + Ord> PartialOrd for PriorityItem<V, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V: Debug, P: Debug + Ord> Ord for PriorityItem<V, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        // make this a min heap
        match self.priority.cmp(&other.priority) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

fn heuristic(a: Pos, b: Pos) -> i32 {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn a_star_search(grid: &Grid<Block>, start: Pos, goal: Pos) -> Vec<Pos> {
    let mut frontier = PriorityQueue::new();
    frontier.put(start, 0);
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while let Some(current) = frontier.get() {
        if current == goal {
            break;
        }

        for neighbour in grid.neighbours(current) {
            // We don't have different costs on the grid, use 1
            let new_cost = cost_so_far[&current] + 1;
            if !cost_so_far.contains_key(&neighbour) || new_cost < cost_so_far[&neighbour] {
                cost_so_far.insert(neighbour, new_cost);
                let priority = new_cost + heuristic(goal, neighbour);
                frontier.put(neighbour, priority);
                came_from.insert(neighbour, Some(current));
            }
        }
    }

    let mut current = goal;

    // construct a vector of positions to move to
    let mut path = vec![current];

    while let Some(prev) = came_from[&current] {
        // we don't want the start position included in the path
        if prev == start {
            break;
        }
        current = prev;
        path.push(current);
    }
    path.reverse();
    path
}

pub(crate) fn solve() {
    let stdin = io::stdin();
    let rows = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Block::Wall,
                    '.' => Block::Path,
                    c @ '0'..='9' => Block::Number(c.to_digit(10).unwrap() as i32),
                    c => panic!(format!("Unexpected character: {}", c)),
                }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let mut number_positions: HashMap<i32, Pos> = HashMap::new();

    for (y, row) in rows.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if let Block::Number(n) = block {
                number_positions.insert(*n, (x as i32, y as i32));
            }
        }
    }

    let grid = Grid { grid: rows };

    // create a map of position pairs to the number of steps between them.
    let mut pair_steps = HashMap::new();
    number_positions
        .keys()
        .tuple_combinations::<(_, _)>()
        .for_each(|(a, b)| {
            let path = a_star_search(&grid, number_positions[&a], number_positions[&b]);
            let distance = path.len();
            pair_steps.insert((*a, *b), distance);
            pair_steps.insert((*b, *a), distance);
        });

    let mut numbers = number_positions
        .keys()
        .filter(|num| **num != 0)
        .collect::<Vec<_>>();

    let mut part1_steps = Vec::new();
    let mut part2_steps = Vec::new();

    // generate each permutation of nodes to visit after zero.
    let heap = Heap::new(&mut numbers);
    for data in heap {
        // First get the number of steps from 0 to the first number.
        let mut steps = pair_steps[&(0, *data[0])];

        let mut iter = data.windows(2);
        while let Some([a, b]) = iter.next() {
            let distance = pair_steps[&(**a, **b)];
            steps += distance;
        }
        let part2_extra = pair_steps[&(**data.last().unwrap(), 0)];
        part1_steps.push(steps);
        part2_steps.push(steps + part2_extra)
    }

    println!("Part 1: {}", part1_steps.iter().min().unwrap());
    println!("Part 2: {}", part2_steps.iter().min().unwrap());
}
