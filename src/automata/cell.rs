use rand::{self, Rng};


/// Represents a cell in grid
#[derive(Clone, Debug)]
pub struct Cell {
    pub channels: [f32; 4],
}

impl Cell {
    pub fn new() -> Self {
        Self {
            channels: [
                0.0,
                0.0,
                0.0,
                0.0,
            ]
        }
    }

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
