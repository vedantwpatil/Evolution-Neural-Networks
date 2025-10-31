#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}
// Can reimplement using matricies instead of vectors
#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
// We don't need layer or neuron to be exposed publically since it doesn't need to be visible
// across towards other directories
#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }
    pub fn random(layers: Vec<LayerTopology>) -> Self {
        todo!()
    }
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias * output).max(0.0)
    }
}
