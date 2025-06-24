pub fn fitness_regression(prediction: &Vec<f32>, label: &Vec<f32>, label_mean: &f32) -> f32 {
    assert_eq!(prediction.len(), label.len());
    let mut sum_sqared_error: f32 = 0.;
    let mut sum_error_over_mean: f32 = 0.;

    prediction.iter().zip(label.iter()).for_each(|(pred, lbl)| {
        sum_sqared_error += (pred - lbl).powi(2);
        sum_error_over_mean += (pred - label_mean).powi(2);
    });

    // if sum_error_over_mean <= 0.000_001 {
    //     return 0;
    // }
    return sum_sqared_error / sum_error_over_mean;

    // fitness = fitness / (prediction.len() as f32);

    // return fitness;
}
