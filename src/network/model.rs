//!
//! Defines the neural network used
//! 

use burn::{
    nn::{
        Linear, LinearConfig, Relu,
    },
    prelude::*,
};

// The structure of a cell
use crate::automata::Cell;


/* Defining the structure of our model as well as forward prop */

///
/// Three layer neural network
/// 
/// 
/// Inputs: 
///     Takes 3 cells worth of data
///     (Represents the current cell and its neighbors)
/// 
/// 
/// First:  
///     Linear transform to take cells values to hidden layer size
/// 
/// Second: 
///     ReLu activation layer on prev values
/// 
/// Third:
///     Linear transform on the hidden layer size worth 
///     of data and turns it into one cells worth
/// 
/// 
/// Output:
///     Returns one cell worth of data
///     (Represents the current cells new value)
/// 

#[derive(Module, Debug)]
/// Represents the structure of the neural network
pub struct Model<B: Backend> {
    linear1: Linear<B>,
    activation: Relu,
    linear2: Linear<B>,
}impl<B: Backend> Model<B> {
    /// Forward propegation ordering of functions
    pub fn forward(&self, tensor: Tensor<B, 1>) -> Tensor<B, 1> {
        let x = self.linear1.forward(tensor);
        let x = self.activation.forward(x);
        self.linear2.forward(x)
    }
}


/* Model config that creates the model for us with the correct size and dimensions*/

#[derive(Config, Debug)]
/// Represents the model configuration
pub struct ModelConfig {
    pub input_size: usize,
    pub hidden_size: usize,
    pub output_size: usize,
}impl ModelConfig {
    /// Returns the initialized model.
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            linear1: LinearConfig::new(self.input_size, self.hidden_size).init(device),
            activation: Relu::new(),
            linear2: LinearConfig::new(self.hidden_size, self.output_size).init(device),
        }
    }
}


/* Helper functions to properly deal with data */

/// Helps to convert the cell object to the appropriate format
pub fn prepare_data_for_input<B: Backend>(identity_cell: &Cell, sobel_x_cell: &Cell, sobel_y_cell: &Cell, device: &B::Device) -> Tensor<B, 1> {
    let len = identity_cell.channels.len();

    // Combining data into one array
    let mut combined_channels = [0.0; 12]; // Total length = 3 * 4 = 12
    combined_channels[0..len].copy_from_slice(&identity_cell.channels);
    combined_channels[len..len*2].copy_from_slice(&sobel_x_cell.channels);
    combined_channels[len*2..len*3].copy_from_slice(&sobel_y_cell.channels);


    // Structuring the data
    let data = Data::from(combined_channels);

    let tensor = Tensor::<B, 1>::from_floats(data, device);

    tensor
}

/// Helps convert tensor output object to cell
pub fn output_data_to_cell<B: Backend>(tensor: Tensor<B, 1>) -> Cell {
    let mut iter = tensor.into_data().convert::<f32>().value.into_iter();

    Cell {
        channels: [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ]
    }
}
