//! 
//! An array of masks to help isolate the correct cells and to add 
//! weights to their values
//! 
#[allow(unused)]


/// Mask that isolates the working cell
pub const SELF_MASK: [[i8; 3]; 3] = [
    [0, 0, 0],
    [0, 1, 0],
    [0, 0, 0]
];
/// Mask that estimates derivative in x direction
pub const SOBEL_X: [[i8; 3]; 3] = [
    [-1, 0, 1],
    [-2, 0, 2],
    [-1, 0, 1]
];
/// Mask that estimates derivative in y direction
pub const SOBEL_Y: [[i8; 3]; 3] = [
    [ 1, 2, 1],
    [ 0, 0, 0],
    [-1,-2,-1]
];
