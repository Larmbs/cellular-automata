use rand::{self, Rng};

/// Represents a individual cell on the grid
#[derive(Clone, Debug)]
pub struct Cell {
    // Channels represent store data of cell and can serve as communication channels between them
    // The first channel is interpreted as alpha value to constrain sim
    pub channels: [f32; 4],
} impl Cell {
    /// Creates a new cell with is channels filled with zeroes
    pub fn new() -> Self {
        Self {
            channels: [0.,0.,0.,0.,]
        }
    }
    /// Creates a new cell with is channels filled with ones
    pub fn new_ones() -> Self {
        Self {
            channels: [1.,1.,1.,1.,]
        }
    }
    /// Creates a new cell with its channel filled with random numbers
    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            channels:[
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
            ]
        }
    }
}
