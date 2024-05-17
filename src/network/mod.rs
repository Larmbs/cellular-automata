use tch::{nn, nn::Module, nn::OptimizerConfig, Device};

const INPUT_SIZE: i64 = 2 * 2 + 2;
const HIDDEN_NODES: i64 = 32;
const OUTPUT_SIZE: i64 = 2;

///
/// Layers
/// 6 -> 32 -> 2
/// 
/// 

/// Returns structure of neural network
fn net(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::linear(
            vs / "layer1",
            INPUT_SIZE,
            HIDDEN_NODES,
            Default::default(),
        ))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs, HIDDEN_NODES, OUTPUT_SIZE, Default::default()))
}

fn train() {
    
}