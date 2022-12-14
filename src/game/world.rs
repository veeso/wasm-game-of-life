//! # World

use super::Cell;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
/// The container for the world.
/// The world as implemented is to be considered "a globe", so edges are communicating
pub struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl World {
    /// Instantiates a new `World`
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    /// Returns an iterator over the indexes of alive cells
    pub fn alive_cells(&self) -> impl Iterator<Item = usize> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_i, cell)| cell.is_alive())
            .map(|(i, _cell)| i)
    }

    /// Returns an iterator over the indexes of dead cells
    pub fn dead_cells(&self) -> impl Iterator<Item = usize> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_i, cell)| !cell.is_alive())
            .map(|(i, _cell)| i)
    }

    /// Get cell at provided position
    pub fn get_cell(&self, row: u32, col: u32) -> Option<Cell> {
        self.cells.get(self.index(row, col)).copied()
    }

    /// Update value of a cell at provided position
    pub fn write_cell(&mut self, cell: Cell, row: u32, col: u32) {
        let index = self.index(row, col);
        if let Some(world_cell) = self.cells.get_mut(index) {
            *world_cell = cell;
        }
    }

    /// Given a position, returns the INDEX of the cell's neighbours
    pub fn neighbours(&self, row: u32, col: u32) -> [usize; 8] {
        let upper_row = row.checked_sub(1).unwrap_or_else(|| self.last_row());
        let lower_row = if row == self.last_row() { 0 } else { row + 1 };
        let column_at_left = col.checked_sub(1).unwrap_or_else(|| self.last_col());
        let column_at_right = if col == self.last_col() { 0 } else { col + 1 };

        [
            self.index(upper_row, column_at_left),
            self.index(upper_row, col),
            self.index(upper_row, column_at_right),
            self.index(row, column_at_left),
            self.index(row, column_at_right),
            self.index(lower_row, column_at_left),
            self.index(lower_row, col),
            self.index(lower_row, column_at_right),
        ]
    }

    /// Calculate row and column from a given index
    pub fn row_and_column(&self, index: usize) -> (u32, u32) {
        (index as u32 / self.width, index as u32 % self.width)
    }

    /// Get index for cell in a matrix view
    fn index(&self, row: u32, col: u32) -> usize {
        ((row * self.width) + col) as usize
    }

    /// Returns the row at the lowest bound
    fn last_row(&self) -> u32 {
        self.height - 1
    }

    /// Returns the column further to the right in the world
    fn last_col(&self) -> u32 {
        self.width - 1
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_create_new_world() {
        let world = World::new(16, 16);
        assert_eq!(world.width, 16);
        assert_eq!(world.height, 16);
        assert_eq!(world.cells.len(), 256);
    }

    #[test]
    fn should_get_cell() {
        let mut world = World::new(16, 16);
        world.cells[18] = Cell::Alive;
        assert_eq!(world.get_cell(1, 2).unwrap(), Cell::Alive);
        assert_eq!(world.get_cell(20, 1), None);
    }

    #[test]
    fn should_write_cell() {
        let mut world = World::new(16, 16);
        world.write_cell(Cell::Alive, 2, 7);
        assert_eq!(world.get_cell(2, 7).unwrap(), Cell::Alive);
    }

    #[test]
    fn should_iter_alive_cells() {
        let mut world = World::new(16, 16);
        world.write_cell(Cell::Alive, 2, 7);
        world.write_cell(Cell::Alive, 1, 2);
        assert_eq!(world.alive_cells().count(), 2);
    }

    #[test]
    fn should_iter_dead_cells() {
        let mut world = World::new(16, 16);
        world.write_cell(Cell::Alive, 2, 7);
        world.write_cell(Cell::Alive, 1, 2);
        assert_eq!(world.dead_cells().count(), 254);
    }

    #[test]
    fn should_get_last_row_and_columns() {
        let world = World::new(16, 20);
        assert_eq!(world.last_col(), 15);
        assert_eq!(world.last_row(), 19);
    }

    #[test]
    fn should_get_neighbours() {
        let world = World::new(16, 16);
        assert_eq!(
            world.neighbours(2, 2),
            [
                world.index(1, 1),
                world.index(1, 2),
                world.index(1, 3),
                world.index(2, 1),
                world.index(2, 3),
                world.index(3, 1),
                world.index(3, 2),
                world.index(3, 3),
            ]
        );
    }

    #[test]
    fn should_get_rows_and_columns_from_index() {
        let world = World::new(16, 16);
        assert_eq!(world.row_and_column(world.index(8, 13)), (8, 13));
    }

    #[test]
    fn should_get_neighbours_over_lower_bounds() {
        let world = World::new(20, 20);
        assert_eq!(
            world.neighbours(19, 19),
            [
                world.index(18, 18),
                world.index(18, 19),
                world.index(18, 0),
                world.index(19, 18),
                world.index(19, 0),
                world.index(0, 18),
                world.index(0, 19),
                world.index(0, 0),
            ]
        );
    }

    #[test]
    fn should_get_neighbours_over_upper_bounds() {
        let world = World::new(20, 20);
        assert_eq!(
            world.neighbours(0, 0),
            [
                world.index(19, 19),
                world.index(19, 0),
                world.index(19, 1),
                world.index(0, 19),
                world.index(0, 1),
                world.index(1, 19),
                world.index(1, 0),
                world.index(1, 1),
            ]
        );
    }
}
