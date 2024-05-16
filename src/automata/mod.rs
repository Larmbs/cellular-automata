
mod masks;
mod cell;
mod network;
pub use cell::Cell;
use rand::Rng;

pub struct CellularGrid {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

#[allow(unused)]
impl CellularGrid {
    /// Returns a grid
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![Cell::new(0., 1.)]
        }
    }

    /// Creates a grid full of noise
    pub fn new_with_noise(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(width*height);

        // Populate grid with random cells
        let mut rng = rand::thread_rng();
        for _ in 0..width*height {
            grid.push(Cell::new(rng.gen(), rng.gen()))
        }

        Self {
            width,
            height,
            grid,
        }
    }

    /// Returns a reference to grid
    pub fn grid_as_ref(&self) -> &Vec<Cell> {
        &self.grid
    }

    /// Returns grid size
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Wraps xy cors so they are in bounds
    fn wrap_xy(&self, x: i32, y: i32) -> (usize, usize) {
        let x = ((x % self.width as i32) + self.width as i32) % self.width as i32;
        let y = ((y % self.height as i32) + self.height as i32) % self.height as i32;
        (x as usize, y as usize)
    }
}
    
impl CellularGrid {
    /// Updates grid based off conditions
    pub fn update(&mut self) {
        // Creating the result grid that will overwrite the previous one
        let mut new_grid = vec![Cell::new(0., 0.); self.grid.len()];

        for (index, cell) in self.grid.iter().enumerate() {
            let x = index % self.width;
            let y = index / self.width;

            // Using Sobel masks to get x and y partial derivatives
            let mut sum_sobel_x = Cell::new(0., 0.);
            let mut sum_soble_y = Cell::new(0., 0.);
            for mx in -1..=1 {
                for my in -1..=1 {
                    // p stands for pointer as we point to each neighbor cell
                    let (px, py) = self.wrap_xy(x as i32 + mx, y as i32 + my);
                    let pcell = &self.grid[px + py * self.width];

                    // add to cumulative sum
                    sum_sobel_x.alpha += pcell.alpha * masks::SOBEL_X[(my + 1) as usize][(mx + 1) as usize] as f64;
                    sum_soble_y.value += pcell.value * masks::SOBEL_Y[(my + 1) as usize][(mx + 1) as usize] as f64;
                }
            }

            // Using neural network to choose value for cell based off params
            new_grid[index] = network::network(cell, &sum_sobel_x, &sum_soble_y);
        }
        
        // Overwriting previous grid
        self.grid = new_grid;
    }
}