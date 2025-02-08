pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::new();

        for neuron in &self.neurons {
            let output = neuron.propagate(inputs);
            outputs.push(output);
        }

        outputs
    }
}

impl Network {
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Neuron {
    fn propagate(&self, inputs: Vec<f32>) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
