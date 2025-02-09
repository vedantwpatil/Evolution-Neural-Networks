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

// Higher order functions are often prefered in rust due to faster compile times
impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // let mut outputs = Vec::new();

        // for neuron in &self.neurons {
        //     // We have to ensure that we are borrowing the ownership of inputs and not moving the
        //     // ownership
        //     let output = neuron.propagate(&inputs);
        //     outputs.push(output);
        // }

        // We can instead use the map higher ordered function
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Network {
    // We have to ensure that in each iteration we are borrowing the ownership of inputs and not
    // move it into layer.propagate, otherwise when we try to run the next iteration we don't
    // have access to inputs

    // We also have to ensure that our function works on borrowed elements
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // This is using higher order functions

        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }
}

impl Neuron {
    // We have to ensure that our code allows for the use borrowed ownership of inputs rather than
    // moved ownership of inputs
    fn propagate(&self, inputs: &[f32]) -> f32 {
        // This implementation assumes that inputs.len() is always the same as self.weights.len()
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
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
