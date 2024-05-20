//!
//! Module to train our ai by running the simulation 
//! and calculating losses for back propagation
//! 
#![allow(unused)]

// Importing model
use super::{
    model::{
        Model,
        ModelConfig,
        prepare_data_for_input,
        output_data_to_cell,
    },
    loss::{
        compare_images_for_accuracy,
        compare_images_for_loss,
    },
    image::get_image_of_floats,
};

// Importing game
use crate::automata::{
    CellularGrid,
    Cell,
};

// Importing burn
use burn::{
    nn::loss::CrossEntropyLoss, optim::{
        Adam, AdamConfig
    }, prelude::*, record::CompactRecorder
};

// Importing rng modules
use rand::SeedableRng;
use rand::rngs::StdRng;

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
    B::seed(config.seed);

    let target_image_path = "data/images/whale.png";
    let (target_image, (width, height)) = get_image_of_floats(&target_image_path).expect("Path provided was invalid");

    // Creating the model
    let model: Model<B> = ModelConfig {
        input_size: 4 * 3,
        hidden_size: 4 * 3 * 3,
        output_size: 4,
    }.init(device);

    // Setting up optimizer
    let mut optimizer = AdamConfig::new();

    // Initializing sim
    let mut sim = CellularGrid::new_with_seed(width, height);

    // Main loop
    for i in 0..config.max_steps {
        // Step n steps
        for _ in 0..config.step_size {
            sim.update(|cell1, cell2, cell3| {
                let tensor = prepare_data_for_input(cell1, cell2, cell3, device);
                let output = model.forward(tensor);
                output_data_to_cell(output)
            })
        }

        // Getting current grid state
        let output = sim.grid_as_ref();
        
        let accuracy = compare_images_for_accuracy(output, &target_image);

        // Calculate the loss
        let loss = compare_images_for_loss(sim.grid_as_ref(), &target_image);

        
        ///model = optimizer.step(config.learning_rate, model, grads);

        println!("Epoch: {} Loss: {} Accur: {}", i, loss, accuracy);

        // Checking if sim has died
        if sim.has_died() {
            sim = CellularGrid::new_with_seed(width, height);

            println!("Sim has died")
        }
    }

    model
        .save_file("models/model1", &CompactRecorder::new())
        .expect("Trained model should be saved")
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

