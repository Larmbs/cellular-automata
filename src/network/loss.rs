//!
//! Module to help calculate the loss the loss between the 
//! result values and the target ones
//! 

use crate::automata::Cell;

/// Compares a grid of cells to a target grid: returns the mean difference
pub fn compare_images(current: &Vec<Cell>, target: &Vec<f32>) -> f32 {
    // Catch invalid inputs
    assert_eq!(current.len(), target.len());

    // Calculating the difference between the two grids
    let score: f32 = current.iter()
        .zip(target.iter())
        .map(|(a, b)| (a.channels[0]-b).powi(2))
        .sum();

    score / current.len() as f32
}
