use crate::global_params::CgpParameters as g_params;
use crate::utils::utility_funcs;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};

use crate::cgp_files::chromosome::Chromosome;

pub struct Runner {
    params: g_params,
    data: Vec<Vec<f32>>,
    label: Vec<f32>,
    // eval_data: Vec<Vec<f32>>,
    // eval_label: Vec<f32>,
    population: Vec<Chromosome>,
    best_fitness: f32,
    fitness_vals: Vec<f32>,
    parent_id: usize,
    label_mean: f32,
    pub total_numer_evals: usize,
}

impl Display for Runner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parent: {}", self.population[self.parent_id])?;
        writeln!(f, "Fitness: {}", self.best_fitness)
    }
}

impl Runner {
    pub fn new(params: g_params, data: Vec<Vec<f32>>, label: Vec<f32>) -> Self {
        let mut chromosomes: Vec<Chromosome> = Vec::with_capacity(params.mu + params.lambda);
        let mut fitness_vals: Vec<f32> = Vec::with_capacity(params.mu + params.lambda);

        let label_mean: f32 = label.iter().sum::<f32>() / label.len() as f32;
        
        // transpose so a whole row of the dataset can be used as an array for calculation
        let data = utility_funcs::transpose(data);
        
        let mut total_nbr_fitness_evals = 0;
        
        for _ in 0..(params.mu + params.lambda) {
            let mut chromosome = Chromosome::new(params.clone());
            let (mut fitness, nbr_fitness_evals) = chromosome.evaluate(&data, &label, &label_mean);
            if fitness.is_nan() {
                fitness = f32::MAX;
            }
            fitness_vals.push(fitness);

            total_nbr_fitness_evals += nbr_fitness_evals;
            
            chromosomes.push(chromosome);
        }

        let best_fitness = utility_funcs::get_min(&fitness_vals);
        let parent_id = utility_funcs::get_argmin(&fitness_vals);

        Self {
            params,
            data,
            label,
            // eval_data,
            // eval_label,
            population: chromosomes,
            best_fitness,
            fitness_vals,
            parent_id,
            label_mean,
            total_numer_evals: total_nbr_fitness_evals,
        }
    }

    pub fn learn_step(&mut self) {
        self.mutate_chromosomes();

        self.eval_chromosomes();

        self.new_parent_by_neutral_search();
        
    }
    
    fn new_parent_by_neutral_search(&mut self) {
        let mut min_keys: Vec<usize> = Vec::with_capacity(self.params.mu + self.params.lambda);

        utility_funcs::get_argmins_of_value(&self.fitness_vals, &mut min_keys, self.best_fitness);

        if min_keys.len() == 1 {
            self.parent_id = min_keys[0];
        } else {
            if min_keys.contains(&self.parent_id) {
                let index = min_keys.iter().position(|x| *x == self.parent_id).unwrap();
                min_keys.remove(index);
            }
            self.parent_id = *min_keys.choose(&mut rand::thread_rng()).unwrap();
        }
    }

    fn mutate_chromosomes(&mut self) {
        // mutate new chromosomes; do not mutate parent
        for i in 0..(self.params.mu + self.params.lambda) {
            if i == self.parent_id {
                continue;
            }
            self.population[i] = self.population[self.parent_id].clone();

            match self.params.mutation_type {
                0 => {
                    self.population[i].mutate_single();
                }
                1 => {
                    self.population[i].mutate_prob(self.params.mutation_rate);
                }

                _ => {
                    panic!("mutatio ntype not def")
                }
            }
        }
    }

    fn eval_chromosomes(&mut self) {
        for i in 0..(self.params.mu + self.params.lambda) {
            if i != self.parent_id {
                let (mut fitness, nbr_fitness_evals) = self.population[i].evaluate(&self.data, &self.label, &self.label_mean);
                
                self.total_numer_evals += nbr_fitness_evals;
                
                if self.params.bend_connection_nodes {
                    fitness = self.population[i].bend_outputnode_connection(fitness)
                }

                if fitness.is_nan() {
                    fitness = f32::MAX;
                }
                self.fitness_vals[i] = fitness;
            }
        }

        let best_fitness = utility_funcs::get_min(&self.fitness_vals);

        self.best_fitness = best_fitness;
    }

    pub fn get_test_fitness(&mut self, eval_data: &Vec<Vec<f32>>, eval_label: &Vec<f32>) -> f32 {
        let eval_label_mean: f32 =  eval_label.iter().sum::<f32>() / eval_label.len() as f32;
        
        let (mut best_fitness, _) = self.population[self.parent_id].evaluate(eval_data, eval_label, &eval_label_mean);

        // for individual in &mut self.population {
        //     let fitness = individual.evaluate(&self.data, &self.label, &self.label_mean);
        // 
        //     if !fitness.is_nan() && fitness < best_fitness {
        //         best_fitness = fitness;
        //     }
        // }
        return best_fitness;
    }

    pub fn get_best_fitness(&self) -> f32 {
        return self.best_fitness;
    }

    pub fn get_parent(&self) -> Chromosome {
        return self.population[self.parent_id].clone();
    }
}
