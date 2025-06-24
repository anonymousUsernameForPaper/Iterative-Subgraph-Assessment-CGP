use crate::global_params::CgpParameters as g_params;
use crate::utils::fitness_metrics;
use crate::utils::node_type::NodeType;
use crate::utils::utility_funcs;
use crate::cgp_files::node::Node;
use nohash_hasher::BuildNoHashHasher;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Chromosome {
    pub params: g_params,
    pub nodes_grid: Vec<Node>,
    pub output_node_ids: Vec<usize>,
    pub active_nodes: Vec<usize>,
    pub fitness_value_per_node: Vec<f32>,
    // pub nbr_fitness_evaluations: usize,
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "+++++++++++++++++ Chromosome +++++++++++")?;
        writeln!(f, "Nodes:")?;
        for node in &self.nodes_grid {
            write!(f, "{}", *node)?;
        }
        writeln!(f, "Active_nodes: {:?}", self.active_nodes)?;
        writeln!(f, "Output_nodes: {:?}", self.output_node_ids)
    }
}

impl Chromosome {
    pub fn new(params: g_params) -> Self {
        assert_eq!(params.nbr_outputs, 1);

        let mut nodes_grid: Vec<Node> = vec![];
        let mut output_node_ids: Vec<usize> = vec![];
        nodes_grid.reserve(params.nbr_inputs + params.graph_width + params.nbr_outputs);
        output_node_ids.reserve(params.nbr_outputs);

        // input nodes
        for position in 0..params.nbr_inputs {
            nodes_grid.push(Node::new(
                position,
                params.nbr_inputs,
                params.graph_width,
                NodeType::InputNode,
            ));
        }
        // computational nodes
        for position in params.nbr_inputs..(params.nbr_inputs + params.graph_width) {
            nodes_grid.push(Node::new(
                position,
                params.nbr_inputs,
                params.graph_width,
                NodeType::ComputationalNode,
            ));
        }
        // output nodes
        for position in (params.nbr_inputs + params.graph_width)
            ..(params.nbr_inputs + params.graph_width + params.nbr_outputs)
        {
            nodes_grid.push(Node::new(
                position,
                params.nbr_inputs,
                params.graph_width,
                NodeType::OutputNode,
            ));
        }

        // get position of output nodes
        for position in (params.nbr_inputs + params.graph_width)
            ..(params.nbr_inputs + params.graph_width + params.nbr_outputs)
        {
            output_node_ids.push(position);
        }
        
        Self {
            fitness_value_per_node: vec![f32::MAX; params.graph_width + params.nbr_inputs],
            params,
            nodes_grid,
            output_node_ids,
            active_nodes: vec![],
            // current_best_fitness: f32::MAX,
            // nbr_fitness_evaluations: 0,
        }
    }

    fn clear_node_fitness_vec(&mut self) {
        if self.params.bend_connection_nodes {
            self.fitness_value_per_node.fill(f32::MAX);
        }
    }

    pub fn evaluate(&mut self, inputs: &Vec<Vec<f32>>, labels: &Vec<f32>, label_mean: &f32) -> (f32, usize) {
        let mut nbr_fitness_evals: usize = 0;
        // delete old values from list to not falsify results
        self.clear_node_fitness_vec();

        self.get_active_nodes_id();

        let mut outputs: HashMap<usize, Vec<f32>, BuildNoHashHasher<usize>> =
            HashMap::with_capacity_and_hasher(
                self.params.nbr_inputs + self.params.graph_width + self.params.nbr_outputs,
                BuildNoHashHasher::default(),
            );

        // iterate through each input and calculate for each new vector its output
        // as the inputs are transposed, the n-th element of the whole dataset is input
        // i.e. given a dataset with 3 datapoints per entry; and 5 entries.
        // then it will input the first datapoint of all 5 entries first. Then the second, etc.
        for node_id in &self.active_nodes {
            let current_node: &Node = &self.nodes_grid[*node_id];

            match current_node.node_type {
                NodeType::InputNode => {
                    outputs.insert(*node_id, inputs[*node_id].clone());
                }
                NodeType::OutputNode => {
                    let con1 = current_node.connection0;
                    let prev_output1 = outputs.get(&con1).unwrap();
                    outputs.insert(*node_id, prev_output1.clone());
                }
                NodeType::ComputationalNode => {
                    let con1 = current_node.connection0;
                    let prev_output1 = outputs.get(&con1).unwrap();

                    let calculated_result: Vec<f32>;
                    if current_node.function_id <= 3 {
                        // case: two inputs needed
                        let con2 = current_node.connection1;
                        let prev_output2 = outputs.get(&con2).unwrap();

                        calculated_result =
                            current_node.execute(&prev_output1, Some(&prev_output2));
                    } else {
                        // case: only one input needed
                        calculated_result = current_node.execute(&prev_output1, None);
                    }

                    if self.params.bend_connection_nodes {
                        // get the intermediate fitness result of a node
                        let node_fitness =
                            fitness_metrics::fitness_regression(&calculated_result, &labels, label_mean);
                        self.fitness_value_per_node[*node_id] = node_fitness;
                        nbr_fitness_evals += 1;
                    }

                    outputs.insert(*node_id, calculated_result);
                }
            }
        }

        let output_start_id = self.params.nbr_inputs + self.params.graph_width;
        // let output_end_id = self.params.nbr_inputs + self.params.graph_width + self.params.nbr_outputs;
        let outs: &Vec<f32> = outputs.get(&output_start_id).unwrap();
        // println!("{:?}", outs);
        assert!(self.nodes_grid[output_start_id].node_type == NodeType::OutputNode);

        let fitness_of_current_output_node = fitness_metrics::fitness_regression(&outs, &labels, label_mean);
        nbr_fitness_evals += 1;

        return (fitness_of_current_output_node, nbr_fitness_evals);
    }

    pub fn bend_outputnode_connection(&mut self, output_node_fitness: f32) -> f32 {
        // // todo delete
        // println!("intermediate result: ");
        // for i in 0..self.params.graph_width +self.params.nbr_inputs{
        //     if self.fitness_value_per_node[i] != f32::MAX {
        //         print!("{}: {}\t", i, self.fitness_value_per_node[i]);
        //     }
        // }
        // println!("\nfinal fitness: {:?}", output_node_fitness);
        // // !todo delete

        // this ID corresponds to the node with the lowest fitness
        let min_index = utility_funcs::get_argmin(&self.fitness_value_per_node);
        // println!("min index: {}", min_index);

        //     currently, there is only one output node. Thus, just bend the first ID
        //     println!("old output node connection id: {}, {}",  self.nodes_grid[self.output_node_ids[0]].connection0,self.nodes_grid[self.output_node_ids[0]].connection1);
        self.nodes_grid[self.output_node_ids[0]].connection0 = min_index;
        // println!("new output node connection id: {}, {}",  self.nodes_grid[self.output_node_ids[0]].connection0,self.nodes_grid[self.output_node_ids[0]].connection1);
        self.get_active_nodes_id();

        return self.fitness_value_per_node[min_index];
    }

    pub fn get_active_nodes_id(&mut self) {
        let mut active: HashSet<usize, BuildNoHashHasher<usize>> =
            HashSet::with_capacity_and_hasher(
                self.params.nbr_inputs + self.params.graph_width + self.params.nbr_outputs,
                BuildNoHashHasher::default(),
            );

        let mut to_visit: Vec<usize> = vec![];
        to_visit
            .reserve(self.params.nbr_inputs + self.params.graph_width + self.params.nbr_outputs);

        for output_node_id in &self.output_node_ids {
            active.insert(*output_node_id);
            to_visit.push(*output_node_id);
        }

        while let Some(current_node_id) = to_visit.pop() {
            let current_node: &Node = &self.nodes_grid[current_node_id];

            match current_node.node_type {
                NodeType::InputNode => continue,

                NodeType::ComputationalNode => {
                    let connection0 = current_node.connection0;
                    if !active.contains(&connection0) {
                        to_visit.push(connection0);
                        active.insert(connection0);
                    }
                    if current_node.function_id <= 3 {
                        /// case: it needs two inputs instead of just one
                        let connection0 = current_node.connection1;
                        if !active.contains(&connection0) {
                            to_visit.push(connection0);
                            active.insert(connection0);
                        }
                    }
                }

                NodeType::OutputNode => {
                    let connection0 = current_node.connection0;
                    if !active.contains(&connection0) {
                        to_visit.push(connection0);
                        active.insert(connection0);
                    }
                }
            }
        }
        let mut active: Vec<usize> = active.into_iter().collect();
        active.sort_unstable();

        self.active_nodes = active;
    }

    pub fn mutate_single(&mut self) {
        let mut start_id = self.params.nbr_inputs;
        if start_id == 1 {
            // Serious edge case: if start_id == 1; then only the first node can be mutated.
            // if its connection gets mutated, it can only mutate a connection to 0, because
            // the first node must have a connection to the input.
            // As the code currently forces a change of value, this will not terminate.
            start_id = 2;
        }
        let end_id = self.params.nbr_inputs + self.params.graph_width + self.params.nbr_outputs;

        let between = Uniform::from(start_id..end_id);
        let mut rng = rand::thread_rng();

        loop {
            let random_node_id = between.sample(&mut rng);
            self.nodes_grid[random_node_id].mutate();

            if self.active_nodes.contains(&random_node_id) {
                break;
            }
        }
    }

    pub fn mutate_prob(&mut self, prob: f32) {
        let mut start_id = self.params.nbr_inputs;
        if start_id == 1 {
            // Serious edge case: if start_id == 1; then only the first node can be mutated.
            // if its connection gets mutated, it can only mutate a connection to 0, because
            // the first node must have a connection to the input.
            // As the code currently forces a change of value, this will not terminate.
            start_id = 2;
        }
        let end_id = self.params.nbr_inputs + self.params.graph_width + self.params.nbr_outputs;

        for node_id in start_id..end_id {
            let random_prob: f32 = rand::thread_rng().gen::<f32>();
            if random_prob < prob {
                self.nodes_grid[node_id].mutate();
            };
        }
    }
}
