//! # Cell
//!

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Represents a single cell in the game, where life can be contained
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    /// Returns whether cell is alive
    pub fn is_alive(&self) -> bool {
        *self == Self::Alive
    }

    /// Toggle current value
    pub fn toggle(&mut self) {
        *self = match *self {
            Self::Alive => Self::Dead,
            Self::Dead => Self::Alive,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_cell_is_alive() {
        assert_eq!(Cell::Dead.is_alive(), false,);
        assert_eq!(Cell::Alive.is_alive(), true);
    }

    #[test]
    fn should_toggle_cell() {
        let mut cell = Cell::Alive;
        cell.toggle();
        assert_eq!(cell, Cell::Dead);
        cell.toggle();
        assert_eq!(cell, Cell::Alive);
    }
}
