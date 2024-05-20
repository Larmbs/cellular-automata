use burn::{
    nn::{
        Linear, LinearConfig, Relu,
    },
    prelude::*,
};

use crate::automata::Cell;


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
        let x = self.linear2.forward(x);

        x
    }
}

/// Helps to convert the cell object to the appropriate format
pub fn prepare_data<B: Backend>(identity_cell: &Cell, sobel_x_cell: &Cell, sobel_y_cell: &Cell, device: &B::Device) -> Tensor<B, 1> {
    // Structuring the data
    let data = Data::from([
        identity_cell.channels,
        sobel_x_cell.channels,
        sobel_y_cell.channels,
    ]);

    let tensor = Tensor::<B, 1>::from_floats(
        Data::from(
            [
            identity_cell.channels,
            sobel_x_cell.channels,
            sobel_y_cell.channels,
            ]
        ).convert(), device);

    
    let tensor = tensor.flatten(1, 1);

    tensor
}










#[derive(Config, Debug)]
/// 
pub struct ModelConfig {
    num_classes: usize,
    hidden_size: usize,
    #[config(default = "0.5")]
    dropout: f64,
}

impl ModelConfig {
    /// Returns the initialized model.
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            linear1: LinearConfig::new(128, self.hidden_size).init(device),
            activation: Relu::new(),
            linear2: LinearConfig::new(self.hidden_size, self.num_classes).init(device),
        }
    }
}