#[allow(unused)]
use super::Cell;

/// Two layer neural network responsible for determining behavior of cells
#[allow(unused)]
pub fn network(identity: &Cell, conv_cellx: &Cell, conv_celly: &Cell) -> Cell {
    Cell::new(identity.alpha, identity.value)
}
