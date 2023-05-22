use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Region {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Region {
    pub fn new() -> Region {
        let width = 1000;
        let height = 1000;

        Region { width, height }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tick(&mut self) {
        let _timer = utils::Timer::new("Region::tick");

        let mut next = {
            let _timer = utils::Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            let _timer = utils::Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (Cell::Dead, 3) => Cell::Alive,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };

                    next[idx] = next_cell;
                }
            }
        }

        let _timer = utils::Timer::new("free old cells");
        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

impl Region {}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A region of width {} and height {}",
            self.width, self.height
        );

        Ok(())
    }
}
