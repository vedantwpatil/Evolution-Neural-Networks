# Simulating Evolution Using Neural Networks in Rust

Combining rust and neural networks to create something cool(hopefully)

## Thought Process

The idea is to create a simulated world with animals and food where each animal is able to go and gather food with a brain and specific vision which helps them determine which food they can get

### Steps

We can model brains as something that takes in information to produce certain outputs, or F(information) = actions

Each neuron will hold random weights and biases, this is so that we can optimize them as we propagate information throughout each layer

### Implementation details

**Layer Topology**
We could've chosen to have a field for both output and input neurons, however this would be unnecessary since the previous s layer output nodes is the current layer's input nodes. With this we can perform the required calculation

Instead we can create a struct to hold the amount of neurons that are in each respective layer, with this information we can then more easily propagate data from each subsequent layer

**Testing**
Regularly it would be difficult to test non deterministic data so we ensure that we are using a random seed to ensure that we create the same values and able to ensure effectiveness

## Learning Rust

Rust calculations are more efficient when using functional programming and iterators, this is something to get more used to.
