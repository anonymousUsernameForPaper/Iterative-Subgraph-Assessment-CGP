use std::fs::File;
use std::io::prelude::*;
use std::{thread, time::Duration};
use rand::Rng;

pub fn get_dataset(dataset_name: &String) -> (Vec<Vec<f32>>, Vec<f32>) {
    let mut rng = rand::thread_rng();
    let random_sleep_timer = rng.gen_range(0..10*1000);  // up to 10 seconds
    thread::sleep(Duration::from_millis(random_sleep_timer));

    let mut train_data: Vec<Vec<f32>> = vec![];
    let mut labels: Vec<f32> = vec![];

    let mut file = File::open(r"/feynman_files/".to_owned() + dataset_name).expect("Failed to open file, worng path");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let contents = contents.lines().collect::<Vec<&str>>();

    for line in &contents[0..100_000] {
        let line = line.trim().split(' ').collect::<Vec<&str>>();
        
        let mut line_train_data: Vec<f32> = vec![];
        for value in &line[0..(line.len() - 1)] {
            line_train_data.push(value.parse::<f32>().expect("Failed to parse line"));
        }
        
        train_data.push(line_train_data);
        
        labels.push(line[line.len() - 1].parse::<f32>().expect("Failed to parse line"));
    }
    return (train_data, labels);
}

pub fn get_eval_dataset(dataset_name: &String) -> (Vec<Vec<f32>>, Vec<f32>) {
    let mut eval_data: Vec<Vec<f32>> = vec![];
    let mut eval_labels: Vec<f32> = vec![];

    let mut file = File::open(r"/feynman_files/".to_owned() + dataset_name).expect("Failed to open file, worng path");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let contents = contents.lines().collect::<Vec<&str>>();

    for line in &contents {
        let line = line.trim().split(' ').collect::<Vec<&str>>();

        let mut line_train_data: Vec<f32> = vec![];
        for value in &line[0..(line.len() - 1)] {
            line_train_data.push(value.parse::<f32>().expect("Failed to parse line"));
        }

        eval_data.push(line_train_data);

        eval_labels.push(line[line.len() - 1].parse::<f32>().expect("Failed to parse line"));
    }
    return (eval_data, eval_labels);
}


