//!
//! Module to help calculate the loss the loss between the 
//! result values and the target ones
//! 

use crate::automata::Cell;

/// Compares a grid of cells to a target grid: returns the mean difference
pub fn compare_images_for_loss(current: &Vec<Cell>, target: &Vec<f32>) -> f32 {
    // Catch invalid inputs
    assert_eq!(current.len(), target.len());

    // Calculating the difference between the two grids
    let score: f32 = current.iter()
        .zip(target.iter())
        .map(|(a, b)| (a.channels[0]-b).powi(2))
        .sum();

    score / current.len() as f32
}

/// Compare images for accuracy
pub fn compare_images_for_accuracy(current: &Vec<Cell>, target: &Vec<f32>) -> f32 {
    // Catch invalid inputs
    assert_eq!(current.len(), target.len());

    // Accuracy represents the amount of correct elements in both grids
    let accuracy: usize = current.iter()
        .zip(target.iter())
        .map(|(a, b)| {(a.channels[0], *b)})
        .filter(|(a, b)| {b - 0.01 < *a && *a < b + 0.01})
        .count();


    accuracy as f32 / current.len() as f32 * 100.
}
