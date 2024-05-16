
/// Represents a cell in grid
#[derive(Clone, Debug)]
pub struct Cell {
    // Determines how dead or alive a cell is
    // If cell has alpha > 0.1 then it is alive
    // Neighbors to this cell will be called growing
    pub alpha: f64,
    // Represents a channel in which the cells can communicate
    pub value: f64,
}

impl Cell {
    pub fn new(alpha: f64, value: f64) -> Self {
        Self {
            alpha,
            value,
        }
    }
}
