use rand::Rng;

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
    pub fn random(layers: &[LayerTopology]) -> Self {
        // We need to force our network to have more than one layer as it makes more sense
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
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

    fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            // Accept an argument we don't care about for our closure
            .map(|_| Neuron::random(input_size))
            .collect();

        Self { neurons }
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

    fn random(input_size: usize) -> Neuron {
        let mut rng = rand::rng();
        let bias = rng.random_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}
