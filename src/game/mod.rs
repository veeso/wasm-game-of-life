mod cell;
mod world;

use wasm_bindgen::prelude::*;

pub use cell::Cell;
pub use world::World;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
/// The game of life instance
pub struct Game {
    turn: u32,
    world: World,
}

#[wasm_bindgen]
impl Game {
    /// Instantiates a new `Game`
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            turn: 0,
            world: World::new(width, height),
        }
    }

    /// Play turn
    pub fn tick(&mut self) {
        self.turn += 1;
        // iter alive cells
        let mut changes = Vec::new();
        // iterate current alive cells
        for cell in self.world.alive_cells() {
            // get cell neighbour
            let (row, column) = self.world.row_and_column(cell);
            let neighbours = self.neighbours_to_cell(self.world.neighbours(row, column));
            let alive_neighbours = neighbours.iter().filter(|x| x.is_alive()).count();
            if !(2..=3).contains(&alive_neighbours) {
                changes.push((Cell::Dead, row, column));
            }
        }
        // iterate current dead cells
        for cell in self.world.dead_cells() {
            let (row, column) = self.world.row_and_column(cell);
            let neighbours = self.neighbours_to_cell(self.world.neighbours(row, column));
            let alive_neighbours = neighbours.iter().filter(|x| x.is_alive()).count();
            if alive_neighbours == 3 {
                changes.push((Cell::Alive, row, column));
            }
        }
        // put alive and dead
        for (state, row, column) in changes.into_iter() {
            self.world.write_cell(state, row, column);
        }
    }

    /// Render world
    pub fn render(&self) -> String {
        self.world.to_string()
    }

    /// Convert neighbours to cells
    fn neighbours_to_cell(&self, neighbours: [usize; 8]) -> Vec<Cell> {
        neighbours
            .iter()
            .map(|x| {
                let (row, col) = self.world.row_and_column(*x);
                self.world.get_cell(row, col).unwrap()
            })
            .collect()
    }
}
