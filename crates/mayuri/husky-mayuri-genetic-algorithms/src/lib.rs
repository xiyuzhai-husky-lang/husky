use rand::*;
use rand_chacha::ChaCha8Rng;
use std::cmp::Ordering;

#[derive(Clone)]
struct Individual {
    genes: Vec<bool>,
    fitness: i32,
}

impl Individual {
    fn new(num_genes: usize, target: &[bool], rng: &mut ChaCha8Rng) -> Self {
        let genes: Vec<bool> = (0..num_genes).map(|_| rng.gen()).collect();
        let fitness = Self::calculate_fitness_value(&genes, target);
        Self { genes, fitness }
    }

    fn update_fitness(&mut self, target: &[bool]) {
        self.fitness = Self::calculate_fitness_value(&self.genes, target);
    }

    fn calculate_fitness_value(genes: &[bool], target: &[bool]) -> i32 {
        genes
            .iter()
            .zip(target.iter())
            .filter(|&(a, b)| a == b)
            .count() as i32
    }

    fn mutate(&mut self, mutation_rate: f64, target: &[bool], rng: &mut ChaCha8Rng) {
        for gene in &mut self.genes {
            if rng.gen::<f64>() < mutation_rate {
                *gene = !*gene;
            }
        }
        self.update_fitness(target);
    }

    fn crossover(&self, other: &Individual, target: &[bool], rng: &mut ChaCha8Rng) -> Individual {
        let crossover_point = rng.gen_range(0..self.genes.len());
        let new_genes: Vec<bool> = self.genes[..crossover_point]
            .iter()
            .chain(&other.genes[crossover_point..])
            .cloned()
            .collect();

        let fitness = Self::calculate_fitness_value(&new_genes, target);
        Individual {
            genes: new_genes,
            fitness,
        }
    }
}

#[derive(Default)]
struct Report {
    generations: Vec<GenerationReport>,
}

struct GenerationReport {
    generation: usize,
    best_fitness: i32,
    best_genes: Vec<bool>,
    population_size: usize,
}

struct Lab {
    population_size: usize,
    num_genes: usize,
    mutation_rate: f64,
    max_generations: usize, // Changed from 'generations' to 'max_generations'
    target: Vec<bool>,
    population: Vec<Individual>,
    max_population_size: usize,
    rng: ChaCha8Rng,
    report: Report, // New field
}

impl Lab {
    fn new(
        population_size: usize,
        num_genes: usize,
        mutation_rate: f64,
        max_generations: usize, // Changed parameter name
        target: Vec<bool>,
        seed: u64,
    ) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let population = (0..population_size)
            .map(|_| Individual::new(num_genes, &target, &mut rng))
            .collect();

        Self {
            population_size,
            num_genes,
            mutation_rate,
            max_generations, // Changed field name
            target,
            population,
            max_population_size: population_size * 2, // Set a maximum population size
            rng,
            report: Report::default(), // Initialize report
        }
    }

    fn run(&mut self) {
        for generation in 0..self.max_generations {
            // Changed to use 'max_generations'
            // Sort population by fitness
            self.population
                .sort_unstable_by(|a, b| b.fitness.cmp(&a.fitness));

            // Print best individual of the generation
            println!(
                "Generation {}: Best fitness = {}, Genes = {:?}",
                generation, self.population[0].fitness, self.population[0].genes
            );

            // Create new individuals through crossover and append to population
            let new_individuals: Vec<Individual> = (0..self.population_size)
                .map(|_| {
                    let parent1 = &self.population[self.rng.gen_range(0..self.population.len())];
                    let parent2 = &self.population[self.rng.gen_range(0..self.population.len())];
                    let mut offspring = parent1.crossover(parent2, &self.target, &mut self.rng);
                    offspring.mutate(self.mutation_rate, &self.target, &mut self.rng);
                    offspring
                })
                .collect();

            // Append new individuals to the population
            self.population.extend(new_individuals);

            // Trim population if it exceeds the maximum size
            if self.population.len() > self.max_population_size {
                self.population
                    .sort_unstable_by(|a, b| b.fitness.cmp(&a.fitness));
                self.population.truncate(self.max_population_size);
            }

            // Update report
            self.report.generations.push(GenerationReport {
                generation,
                best_fitness: self.population[0].fitness,
                best_genes: self.population[0].genes.clone(),
                population_size: self.population.len(),
            });
        }
    }
}

#[test]
fn mayuri_generic_algorithms_works() {
    use expect_test::expect;

    let mut lab = Lab::new(
        100,  // population_size
        10,   // num_genes
        0.01, // mutation_rate
        100,  // max_generations (changed comment)
        vec![
            true, false, true, true, false, true, false, true, false, true,
        ], // target
        42,   // seed
    );

    lab.run();

    // Add assertions or expectations here
}
