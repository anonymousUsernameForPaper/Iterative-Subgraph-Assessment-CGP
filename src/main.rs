use cgp::datasets::*;
use cgp::global_params::CgpParameters;
use cgp::utils::runner::Runner;
use cgp::utils::utility_funcs;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Parser)]
#[clap(author, version, about, name = "testname")]
struct Args {
    #[arg(long, default_value_t = 3)]
    run_id: usize,


    #[arg(long, default_value_t = 500)]
    nbr_nodes: usize,

    #[arg(long, default_value_t = 0)]
    cgp_type: usize,

    // 0: single
    // 1: prob
    #[arg(long, default_value_t = 0)]
    mutation_type: usize,

    #[arg(long, default_value_t = 0.2)]
    mutation_rate: f32,

    // 0: no bending
    // 1: bending on
    #[arg(long, default_value_t = 0)]
    bend_connection_nodes: usize,

    #[arg(long, default_value = "I.10.7")]
    // #[arg(long)]
    dataset_name: String,

    // 0: time
    // 1: fit. evals
    // 2: fit. iteration
    #[arg(long, default_value_t = 0)]
    budget_type: usize,
}

fn main() {
    for run_id in (0..1000).step_by(500) {
        // ################################################################################
        // ############################ Arguments #########################################
        // ################################################################################
        let mut args = Args::parse();

        args.run_id = run_id + args.run_id;
        println!("start {}", args.run_id);

        let (data, label) = feynman_function_loader::get_dataset(&args.dataset_name);
        let (data_eval, label_eval) = feynman_function_loader::get_eval_dataset(&args.dataset_name);

        // for efficient calculation - already done for each chromosome
        let data_eval = utility_funcs::transpose(data_eval);

        let mut params = CgpParameters::default();

        params.graph_width = args.nbr_nodes;

        let nbr_inputs = data[0].len();
        let nbr_outputs = 1;

        params.nbr_inputs = nbr_inputs;
        params.nbr_outputs = nbr_outputs;

        params.mutation_type = args.mutation_type;
        params.mutation_rate = args.mutation_rate;

        if args.bend_connection_nodes == 0 {
            params.bend_connection_nodes = false;
        } else {
            params.bend_connection_nodes = true;
        }

        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        // ################################################################################
        // ############################ Logger ####### ####################################
        // ################################################################################
        let mutation_type_string;
        if args.mutation_type == 0 {
            mutation_type_string = format!("single");
        } else {
            mutation_type_string = format!("prob");
        }

        let dataset_string = "feynman_function_".to_owned() + &*args.dataset_name;

        let cgp_type_string = match args.cgp_type {
            0 => "Baseline_Standard",
            1 => "DAG",
            2 => "E_Reorder",
            _ => panic!("Wrong type"),
        };

        let bend_connection_string = match params.bend_connection_nodes {
            true => "connection_bend",
            false => "no_connection_bend",
        };

        let budget_type_string = match args.budget_type {
            0 => "Time_budget",
            1 => "num_eval_budget",
            2 => "iteration_budget",
            _ => {
                panic!("Wrong type");
            }
        };

        let mut save_path = Path::new("")
            .join("Experiments_Output")
            .join(budget_type_string)
            .join(bend_connection_string)
            .join(cgp_type_string)
            .join(dataset_string)
            .join(format!(
                "number_nodes_{}_{}",
                args.nbr_nodes, mutation_type_string
            ));
        if params.mutation_type == 1 {
            let mut temp = save_path.into_os_string();
            temp.push(format!("mut_rate_{}", args.mutation_rate));
            save_path = temp.into();
        }

        fs::create_dir_all(save_path.clone()).unwrap();

        let save_file_iteration = format!("run_{}_iteration.txt", args.run_id);
        let mut output_file = BufWriter::new(
            File::create(save_path.join(save_file_iteration)).expect("cannot create file"),
        );


        // ################################################################################
        // ############################ Training ##########################################
        // ################################################################################

        let mut runner = Runner::new(params.clone(), data, label);

        let mut runtime: usize = 0;

        let mut total_time_spend: u128 = 0;
        loop {
            if args.budget_type == 0 {
                writeln!(
                    output_file,
                    "fit: {:.4}, t: : {:?}",
                    runner.get_test_fitness(&data_eval, &label_eval),
                    total_time_spend
                )
                .expect("write not okay??");

                let start_time = Instant::now();
                runner.learn_step(); // lern step
                total_time_spend += start_time.elapsed().as_millis();
            } else {
                writeln!(
                    output_file,
                    "fit: {:.4}, fit. eval: {:?}",
                    runner.get_test_fitness(&data_eval, &label_eval),
                    runner.total_numer_evals
                )
                .expect("write not okay??");
                runner.learn_step()
            }

            runtime += 1;

            if args.budget_type == 0 {
                if total_time_spend > 1000 * 60 * 60 * 12 {
                    // 1000ms * 60 seconds * 60 minutes * 12 hours
                    break;
                }
            } else if args.budget_type == 1 {
                if runner.total_numer_evals > 1_000_000 {
                    break;
                }
            } else {
                if runtime >= 250_000 {
                    break;
                }
            }

            if runner.get_best_fitness() <= 0.001 {
                break;
            }
        }

        if args.budget_type == 0 {
            writeln!(
                output_file,
                "fit: {:.4}, t: : {:?}",
                runner.get_test_fitness(&data_eval, &label_eval),
                total_time_spend
            )
                .expect("write not okay??");
        } else {
            writeln!(
                output_file,
                "fit: {:.4}, fit. eval: {:?}",
                runner.get_test_fitness(&data_eval, &label_eval),
                runner.total_numer_evals
            )
                .expect("write not okay??");
        }
        // ################################################################################
        // ############################ Saving to text ####################################
        // ################################################################################
        let mut fitness_eval = f32::MAX;
        let mut fitness_train = f32::MAX;
        fitness_eval = runner.get_test_fitness(&data_eval, &label_eval);
        fitness_train = runner.get_best_fitness();

        println!("{fitness_eval}");

        writeln!(output_file, "End at it: {}", runtime).expect("cannot write");
        writeln!(output_file, "Fitness Eval: {}", fitness_eval).expect("cannot write");
        writeln!(output_file, "Fitness Train: {}", fitness_train).expect("cannot write");

        let save_file_active_node = format!("run_{}_active_node.txt", args.run_id);
        let mut output_file_active_node = BufWriter::new(
            File::create(save_path.join(save_file_active_node)).expect("cannot create file"),
        );

        let mut parent = runner.get_parent();
        parent.get_active_nodes_id();

        write!(output_file_active_node, "{:?}", parent.active_nodes).expect("cannot write");

    }
}
