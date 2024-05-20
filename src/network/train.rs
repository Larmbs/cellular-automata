//!
//! Module to train our ai by running the simulation 
//! and calculating losses for back propagation
//! 

// Importing model
use super::model::{
    Model,
    ModelConfig,
};

// Importing game
use crate::automata::{
    CellularGrid,
    Cell,
};

// Importing burn
use burn::{
    optim::{
        AdamConfig,
        Adam,
    },
    prelude::*,
};

// Importing rng modules
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::sync::Arc;


/* Training */

///
/// This module plays the game but for every n steps it checks how close the 
/// current image is to the desired one and calcs a loss begins back propagation
/// 
/// Training config
/// 
/// Max iterations = Int (Maximum checks to run)
/// Step size = Int (Sim steps between checks)
/// 
/// 
/// Training goes like this
/// 
/// Start:
///     The image will start from a an grid full of cells with 0s and a single cell filled with 1s
/// 
/// Loop:   
///     Move simulation n steps 
///     Calc the accuracy
///     Calc delta accuracy
///     Make any other calculations to speed up convergence
///     Apply backpropigation
/// 
///     Check if image as died
///     if so restart the process
/// 

/// Function to train the network while playing it
pub fn train<B: Backend>(config: TrainingConfig, device: &B::Device) {
    // Setting random seed
    let mut rng = StdRng::seed_from_u64(config.seed);

    // Creating the model
    let model: Model<B> = ModelConfig {
        input_size: 4 * 3,
        hidden_size: 4 * 3 * 3,
        output_size: 4,
    }.init(device);

    // Setting up optimizer
    let mut optimizer = AdamConfig::new();

    // Initializing sim
    let mut sim = CellularGrid::new(20, 20);

    for i in 0..config.max_steps {
        for j in 0..config.step_size {
            sim.update()
        }
    }
}


/* Defining training config */
#[derive(Config)]
/// Represents configuration settings for network
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
    // The step size between sims checks
    pub step_size: usize,
    // Maximum checks to run during sim
    pub max_steps: usize,
}

