use rand::{self, Rng};

use macroquad::{math::UVec2, prelude};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

// Math lib
use libm;

const EMPTY: Cell = Cell::Empty;

fn activation(x: f64) -> f64 {
    1. - (-(x/3.).powi(2).exp())
}
/// A mask grid for sampling grid
const MASK: [[f64; 3]; 3] = 
    [[0.6747402548789978, -0.8332203030586241, 0.6747402548789978],
     [-0.8332203030586241, 0.988332867223755, -0.8332203030586241],
     [0.6747402548789978, -0.8332203030586241, 0.6747402548789978]];


#[derive(Clone, Debug)]
/// Represents a cell in grid
pub enum Cell {
    // Represents an alive cell
    Alive(f64),
    // No living creature in this cell
    Empty,
}

    
pub struct Grid {
    width: usize,
    height: usize,

    window_width: usize,
    window_height: usize,

    cell_width: f32,
    cell_height: f32,

    grid: Vec<Cell>
}

impl Grid {
    pub fn new(width: usize, height: usize, window_width: usize, window_height: usize) -> Self {
        let grid = vec![EMPTY; width*height];

        prelude::request_new_screen_size(window_width as f32, window_height as f32);

        Self {
            width, 
            height,
            window_width,
            window_height,
            grid,
            cell_width: window_width as f32/width as f32,
            cell_height: window_height as f32/height as f32,
        }
    }

    pub fn gen_random(&mut self, num: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..num {
            self.grid[rng.gen_range(0..self.width * self.height)] = Cell::Alive(0.6);
        }
    }

    pub fn xy_to_index(&self, x: isize, y: isize) -> (usize, usize) {
        let x = {
            let m = x % self.width as isize;
            (if m >= 0 {m} else {m + self.width as isize}) as usize
        };
        let y = {
            let m = y % self.height as isize;
            (if m >= 0 {m} else {m + self.height as isize}) as usize
        };
        (x, y)
    }

    pub fn get(&self, x: isize, y: isize) -> &Cell {
        let (x, y) = self.xy_to_index(x, y);
        &self.grid[y * self.width + x]
    }

    pub fn set(&mut self, x: isize, y: isize, value: Cell) {
        let (x, y) = self.xy_to_index(x, y);
        self.grid[y * self.width + x] = value
    }


pub fn update(&mut self) {
    // Create a new grid to hold the updated states
    let mut new_grid = Arc::new(Mutex::new(vec![EMPTY; self.width * self.height]));

    // Split the grid into chunks for parallel processing
    let chunk_size = 100; // Adjust chunk size as needed
    let chunks: Vec<_> = (0..self.width)
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    // Process each chunk in parallel
    chunks.into_par_iter().for_each(|chunk| {
        for x in chunk {
            for y in 0..self.height {
                // Summing up neighbor outputs
                let mut sum = 0.;
                let mut found = 1;
                for my in 0..MASK.len() {
                    for mx in 0..MASK[0].len() {
                        if let Cell::Alive(val) = self.get(x as isize - 1 + mx as isize, y as isize - 1 + my as isize) {
                            sum += val * MASK[my][mx];
                            found += 1;
                        }
                    }
                }

                let val = activation(sum);
                // Update the corresponding cell in the new grid
                match self.grid.get(y * self.width + x).unwrap() {
                    Cell::Alive(prev) => {
                        if prev > &0.2 {
                            new_grid.lock().unwrap()[y * self.width + x] = Cell::Alive(val);
                        }
                    }
                    Cell::Empty => {
                        if val > 0.4 {
                            new_grid.lock().unwrap()[y * self.width + x] = Cell::Alive(val);
                        }
                    }
                }
            }
        }
    });

    // Update the current grid with the new states
    self.grid = Arc::try_unwrap(new_grid).unwrap().into_inner().unwrap();
}

    
    
    pub fn display(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = &self.grid[y * self.width + x];
    
                let pos_x = x as f32 * self.cell_width;
                let pos_y = y as f32 * self.cell_height;
                match cell {
                    Cell::Alive(_) => prelude::draw_rectangle(pos_x, self.window_height as f32 - pos_y, self.cell_width, -self.cell_height, prelude::WHITE),
                    Cell::Empty => {},
                };
    
              
    
            }
        }
    }
}
