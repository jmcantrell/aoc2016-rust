use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};

use super::{ComponentKind, Floor};

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    moves: usize,
    elevator: usize,
    floors: Vec<Floor>,
}

impl State {
    fn new(floors: Vec<Floor>) -> Self {
        Self {
            floors,
            moves: 0,
            elevator: 0,
        }
    }

    fn current_floor(&self) -> &Floor {
        &self.floors[self.elevator]
    }

    fn next_states(&self) -> impl Iterator<Item = State> + '_ {
        let possible_moves = self
            .current_floor()
            .iter()
            .combinations(2)
            .chain(self.current_floor().iter().combinations(1));

        possible_moves.flat_map(move |components| {
            [-1, 1]
                .into_iter()
                .filter_map(|direction| {
                    self.elevator
                        .checked_add_signed(direction)
                        .filter(|&i| i < self.floors.len())
                })
                .filter_map(move |next_elevator| {
                    let mut next_floors = self.floors.clone();

                    next_floors[self.elevator].move_out(components.clone());
                    next_floors[next_elevator]
                        .move_in(components.iter().map(|&&component| component));

                    (next_floors[self.elevator].is_safe() && next_floors[next_elevator].is_safe())
                        .then_some(Self {
                            moves: self.moves + 1,
                            elevator: next_elevator,
                            floors: next_floors,
                        })
                })
        })
    }

    fn key(&self) -> (usize, Vec<Vec<(ComponentKind, usize)>>) {
        let counts_by_floor = self
            .floors
            .iter()
            .map(|floor| {
                let mut counts: Vec<(_, usize)> = floor
                    .iter()
                    .fold(HashMap::new(), |mut counts, component| {
                        *counts.entry(component.kind).or_default() += 1;
                        counts
                    })
                    .into_iter()
                    .collect();

                counts.sort_by_key(|&(_, count)| count);

                counts
            })
            .collect();

        (self.elevator, counts_by_floor)
    }

    fn all_components_at_top(&self) -> bool {
        self.floors
            .iter()
            .take(self.floors.len() - 1)
            .all(|floor| floor.is_empty())
    }
}

pub fn min_moves_to_top(floors: Vec<Floor>) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([State::new(floors)]);

    while let Some(state) = queue.pop_front() {
        if state.all_components_at_top() {
            return Some(state.moves);
        }

        for next_state in state.next_states() {
            if seen.insert(next_state.key()) {
                queue.push_back(next_state)
            }
        }
    }

    None
}
