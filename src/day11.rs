use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use itertools::Itertools;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Fuel {
    Promethium,
    Cobalt,
    Curium,
    Ruthenium,
    Plutonium,
    Hydrogen,
    Lithium,
    Elerium,
    Dilithium,
}

impl fmt::Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fuel::Promethium => write!(f, "Pr"),
            Fuel::Cobalt => write!(f, "Co"),
            Fuel::Curium => write!(f, "Cu"),
            Fuel::Ruthenium => write!(f, "R"),
            Fuel::Plutonium => write!(f, "Pl"),
            Fuel::Hydrogen => write!(f, "H"),
            Fuel::Lithium => write!(f, "L"),
            Fuel::Elerium => write!(f, "E"),
            Fuel::Dilithium => write!(f, "D"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Item {
    Generator(Fuel),
    Microchip(Fuel),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Item::Generator(ref fuel) => write!(f, "{}G", fuel),
            Item::Microchip(ref fuel) => write!(f, "{}M", fuel),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Move {
    One(Item),
    Two(Item, Item),
}

#[derive(Clone, Debug)]
struct State {
    elevator: usize,
    floors: Vec<HashSet<Item>>,
    step: i32,
}

impl State {
    fn is_valid(&self) -> bool {
        for floor in &self.floors {
            let mut microchips: HashSet<&Fuel> = HashSet::new();
            let mut generators: HashSet<&Fuel> = HashSet::new();

            for item in floor {
                match *item {
                    Item::Microchip(ref f) => microchips.insert(f),
                    Item::Generator(ref f) => generators.insert(f),
                };
            }

            let lonely_chips = microchips.difference(&generators)
                .collect::<HashSet<_>>();

            for microchip in lonely_chips {
                let mut dangers = generators.clone();
                dangers.remove(microchip);
                if !dangers.is_empty() {
                    return false
                }
            }
        }
        true
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E{}: ", self.elevator + 1)?;
        for (i, floor) in self.floors.iter().enumerate() {
            write!(f, "[")?;
            let mut items = floor.iter().collect::<Vec<&Item>>();
            items.sort();
            for (j, item) in items.iter().enumerate() {
                write!(f, "{}", item)?;
                if j < floor.len() - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "]")?;
            if i < self.floors.len() - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct StateKey {
    elevator: usize,
    floor_pairs: Vec<VecDeque<usize>>,
}

impl From<State> for StateKey {
    fn from(state: State) -> Self {
        // Use a VecDeque to ensure we always have [Microchip, Generator] order using push_front
        // and push_back
        let mut seen: HashMap<Fuel, VecDeque<usize>> = HashMap::new();

        for (i, floor) in state.floors.iter().enumerate() {
            for item in floor {
                match *item {
                    Item::Microchip(ref f) => {
                        let pair = seen.entry(f.clone())
                            .or_insert_with(|| VecDeque::with_capacity(2));
                        pair.push_front(i)
                    },
                    Item::Generator(ref f) => {
                        let pair = seen.entry(f.clone())
                            .or_insert_with(|| VecDeque::with_capacity(2));
                        pair.push_back(i)
                    },
                };
            }
        }

        let mut floor_pairs = seen.values().cloned().collect::<Vec<_>>();
        floor_pairs.sort();

        StateKey {
            elevator: state.elevator,
            floor_pairs,
        }
    }
}



fn find_min_moves(initial: State) -> Option<i32> {
    let mut visited: HashSet<StateKey> = HashSet::new();
    let mut queue = VecDeque::new();
    let top_floor = initial.floors.len() - 1;
    queue.push_back(initial);

    while let Some(state) = queue.pop_front() {
        let possible_moves = state.floors[state.elevator].iter()
            .cloned()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| Move::Two(a, b))
            .chain(state.floors[state.elevator].iter().cloned().map(Move::One))
            .collect::<Vec<_>>();

        let directions: Vec<i32> = match state.elevator {
            0 => vec![1],
            n if n == top_floor => vec![-1],
            _ => vec![-1, 1],
        };

        for direction in directions {
            for move_ in possible_moves.clone() {
                let cur_floor = state.elevator;
                let next_floor = ((cur_floor as i32) + direction) as usize;

                let mut new_state = state.clone();
                new_state.elevator = next_floor;
                new_state.step += 1;

                match move_ {
                    Move::One(a) => {
                        new_state.floors[cur_floor].remove(&a);
                        new_state.floors[next_floor].insert(a);
                    },
                    Move::Two(a, b) => {
                        new_state.floors[cur_floor].remove(&a);
                        new_state.floors[cur_floor].remove(&b);
                        new_state.floors[next_floor].insert(a);
                        new_state.floors[next_floor].insert(b);
                    }
                }

                if new_state.is_valid() {
                    if new_state.floors[0].is_empty() && new_state.floors[1].is_empty() && new_state.floors[2].is_empty() {
                        return Some(new_state.step)
                    }

                    if visited.insert(new_state.clone().into()) {
                        queue.push_back(new_state)
                    }
                }
            }
        }
    }
    None
}

pub fn solve() {
    let state1 = State {
        elevator: 0,
        floors: vec![
            hashset! {
                Item::Generator(Fuel::Promethium),
                Item::Microchip(Fuel::Promethium),
            },
            hashset! {
                Item::Generator(Fuel::Cobalt),
                Item::Generator(Fuel::Curium),
                Item::Generator(Fuel::Ruthenium),
                Item::Generator(Fuel::Plutonium),
            },
            hashset! {
                Item::Microchip(Fuel::Cobalt),
                Item::Microchip(Fuel::Curium),
                Item::Microchip(Fuel::Ruthenium),
                Item::Microchip(Fuel::Plutonium),
            },
            hashset! {},
        ],
        step: 0,
    };

    let state2 = State {
        elevator: 0,
        floors: vec![
            hashset! {
                Item::Generator(Fuel::Promethium),
                Item::Microchip(Fuel::Promethium),
                Item::Generator(Fuel::Elerium),
                Item::Microchip(Fuel::Elerium),
                Item::Generator(Fuel::Dilithium),
                Item::Microchip(Fuel::Dilithium),
            },
            hashset! {
                Item::Generator(Fuel::Cobalt),
                Item::Generator(Fuel::Curium),
                Item::Generator(Fuel::Ruthenium),
                Item::Generator(Fuel::Plutonium),
            },
            hashset! {
                Item::Microchip(Fuel::Cobalt),
                Item::Microchip(Fuel::Curium),
                Item::Microchip(Fuel::Ruthenium),
                Item::Microchip(Fuel::Plutonium),
            },
            hashset! {},
        ],
        step: 0,
    };

    println!("Part 1: {}", find_min_moves(state1).unwrap());
    println!("Part 2: {}", find_min_moves(state2).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let state = State {
            elevator: 0,
            floors: vec![
                hashset! {
                Item::Microchip(Fuel::Hydrogen),
                Item::Microchip(Fuel::Lithium),
            },
                hashset! {
                Item::Generator(Fuel::Hydrogen),
            },
                hashset! {
                Item::Generator(Fuel::Lithium),
            },
                hashset! {},
            ],
            step: 0,
        };

        let num =  find_min_moves(state).unwrap();
        assert_eq!(num, 11);
    }
}
