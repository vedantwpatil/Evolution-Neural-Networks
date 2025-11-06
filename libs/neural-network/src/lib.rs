use rand::{Rng, RngCore};

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
    // Dyn stands for dynamic traits, we can switch this to generics for better performance
    pub fn random<R: RngCore>(rng: &mut R, layers: &[LayerTopology]) -> Self {
        // We need to force our network to have more than one layer as it makes more sense
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
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

    fn random<R: RngCore>(rng: &mut R, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            // Accept an argument we don't care about for our closure
            .map(|_| Neuron::random(rng, input_size))
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

        (self.bias + output).max(0.0)
    }

    fn random<R: RngCore>(rng: &mut R, input_size: usize) -> Neuron {
        let bias = rng.random_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    use super::*;

    #[test]
    fn random() {
        let mut rng = ChaCha20Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        // Verify we're creating random biases
        assert_relative_eq!(neuron.bias, 0.35842037);

        // Verify we're creating random weights
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.12689018, 0.79230833, -0.6817162, 0.43828797].as_ref()
        );
    }

    #[test]
    fn layer_random_structure() {
        let mut rng = ChaCha20Rng::from_seed(Default::default());
        let input_size = 3;
        let output_size = 2;
        let layer = Layer::random(&mut rng, input_size, output_size);

        // Verify the layer has the correct number of neurons
        assert_eq!(layer.neurons.len(), output_size);

        // Verify each neuron has the correct number of weights
        for neuron in &layer.neurons {
            assert_eq!(neuron.weights.len(), input_size);
        }
    }

    #[test]
    fn layer_random_values() {
        let mut rng = ChaCha20Rng::from_seed(Default::default());
        let layer = Layer::random(&mut rng, 4, 2);

        // Verify deterministic generation with seeded RNG
        // First neuron's bias should match what Neuron::random generates first
        assert_relative_eq!(layer.neurons[0].bias, 0.35842037);

        // First neuron's weights should match deterministic sequence
        assert_relative_eq!(
            layer.neurons[0].weights.as_slice(),
            [0.12689018, 0.79230833, -0.6817162, 0.43828797].as_ref()
        );

        // Second neuron should have different values (RNG state has advanced)
        assert_relative_eq!(layer.neurons[1].bias, -0.78962564);
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // Ensure .max (our Relu ) works
        // https://www.wikiwand.com/en/articles/Rectifier_(neural_networks)
        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

        // `0.5` and `1.0` chosen by a fair dice roll:
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        );
    }
}
