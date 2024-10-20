use cubic_transformation_rs::{
    cubic_transformation_sampling, kurtosis, mean, skewness, variance, Statistics,
};

fn main() {
    let moments = Statistics {
        mean: 0.,
        var: 1.,
        skew: 0.5,
        ex_kurt: 1.,
    };
    let n_scenario = 1000;
    let max_mse = 0.0001;
    let max_cubic_iteration = 10;
    let bias = true;
    let samples =
        cubic_transformation_sampling(&moments, n_scenario, max_mse, max_cubic_iteration).unwrap();
    println!(
        "samples: mean:{:.2}, var:{:.2}, skew:{:.2}, ex_kurt:{:.2}",
        mean(&samples),
        variance(&samples, bias),
        skewness(&samples, bias),
        kurtosis(&samples, bias)
    );
}
