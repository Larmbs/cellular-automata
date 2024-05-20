//!
//! Module to train our ai by running the simulation 
//! and calculating losses for back propagation
//! 

// Importing model
use super::model::{
    Model,
    ModelConfig,
};

// Importing burn
use burn::{
    optim::{
        AdamConfig,
    },
    prelude::*,
};


/* Training */

///
/// This module plays the game but for every n steps it checks how close the 
/// current image is to the desired one and calcs a loss begins back propagation
/// 

/// Function to train the network while playing it
pub fn train(config: TrainingConfig, device: B::Device) {

}


/* Defining training config */
#[derive(Config)]
pub struct TrainingConfig {
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 64)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

