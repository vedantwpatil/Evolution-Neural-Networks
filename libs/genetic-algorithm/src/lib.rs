use rand::seq::IndexedRandom;
use rand::{Rng, RngCore};

pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub struct RouletteWheelSelection;

pub trait SelectionMethod {
    fn select<'a, I, R: RngCore>(&self, rng: &mut R, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I, R: RngCore>(&self, rng: &mut R, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

impl GeneticAlgorithm {
    // I is a type parameter, I is a individual
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len()).map(|_| todo!()).collect()
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[derive(Clone, Debug)]
    struct TestIndvidual {
        fitness: f32,
    }

    impl TestIndvidual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndvidual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }
    #[test]
    fn roulette_wheel_selection() {
        let mut rng = ChaCha20Rng::from_seed(Default::default());

        let population = vec![
            TestIndvidual::new(2.0),
            TestIndvidual::new(1.0),
            TestIndvidual::new(4.0),
            TestIndvidual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            let fitness = RouletteWheelSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter(vec![
            // (fitness, how many times this fitness has been chosen)
            (1, 98),
            (2, 196),
            (3, 304),
            (4, 402),
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
